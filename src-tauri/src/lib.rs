#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![fetch_khan_course, fetch_khan_explore_menu])
    .run(tauri::generate_context!())
    .expect("error while running Khan Academy Desktop");
}

#[tauri::command]
async fn fetch_khan_course(course_id: String) -> Result<serde_json::Value, String> {
  if course_id.trim().is_empty() || course_id.len() > 128 {
    return Err("Invalid course identifier".into());
  }
  let query = include_str!("../../src/khanCourseQuery.graphql");
  let client = reqwest::Client::builder()
    .user_agent("KhanAcademy/8.5.1 (org.khanacademy.android; Android 35; UnknownDeviceName; Google; phone)")
    .build()
    .map_err(|e| e.to_string())?;
  let response = client
    .post("https://www.khanacademy.org/api/internal/graphql/CourseQueryV8")
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    // Match the Android client GraphQL transport and its native app origin.
    .header("Origin", "app:org.khanacademy.android")
    .json(&serde_json::json!({
      "fromGraphiql": false,
      "operationName": "CourseQueryV8",
      "query": query,
      "variables": { "courseId": course_id }
    }))
    .send().await.map_err(|e| format!("Could not reach Khan Academy: {e}"))?;
  let status = response.status();
  let response_text = response.text().await
    .map_err(|e| format!("Could not read Khan Academy response: {e}"))?;
  let body: serde_json::Value = serde_json::from_str(&response_text)
    .map_err(|e| format!("Khan Academy returned HTTP {status} with a non-JSON response: {e}; body: {}", response_text.chars().take(500).collect::<String>()))?;
  if !status.is_success() {
    return Err(format!("Khan Academy returned HTTP {status}: {}", body.get("message").and_then(|v|v.as_str()).unwrap_or("request failed")));
  }
  if let Some(errors) = body.get("errors") {
    return Err(errors.as_array().unwrap_or(&vec![]).iter()
      .filter_map(|e| e.get("message").and_then(|m|m.as_str()))
      .collect::<Vec<_>>().join("; "));
  }
  Ok(body)
}


#[tauri::command]
async fn fetch_khan_explore_menu() -> Result<Vec<serde_json::Value>, String> {
  const ALL: &str = "query AllCurriculumsQuery { curriculums(includeEmptyCurriculum: true) { key country region name courseMenuRevision { sha } } }";
  const MENU: &str = "query LearnMenuQuery($curriculum: String!) { learnMenuTopics(curriculum: $curriculum) { domainId translatedTitle icon children { courseId translatedTitle href } } }";
  let client = reqwest::Client::builder()
    .user_agent("KhanAcademy/8.5.1 (org.khanacademy.android; Android 35; UnknownDeviceName; Google; phone)")
    .build().map_err(|e| format!("Could not initialize Khan Academy client: {e}"))?;
  let response = client.post("https://www.khanacademy.org/api/internal/graphql/AllCurriculumsQuery")
    .header("Accept", "application/json").header("Content-Type", "application/json")
    .header("Origin", "app:org.khanacademy.android")
    .json(&serde_json::json!({"fromGraphiql":false,"operationName":"AllCurriculumsQuery","query":ALL,"variables":{}}))
    .send().await.map_err(|e| format!("Could not fetch Khan Academy curriculums: {e}"))?;
  let status = response.status();
  let body: serde_json::Value = response.json().await.map_err(|e| format!("Could not parse curriculum response (HTTP {status}): {e}"))?;
  if !status.is_success() { return Err(format!("Curriculum request returned HTTP {status}: {}", body)); }
  if let Some(errors) = body.get("errors") { return Err(format!("Curriculum GraphQL error: {errors}")); }
  let curriculums = body.pointer("/data/curriculums").and_then(|v|v.as_array()).ok_or("Khan Academy response omitted data.curriculums")?;
  let curriculum = curriculums.iter().find(|c| c.get("key").and_then(|v|v.as_str()).map(|k| k.starts_with("us-")).unwrap_or(false))
    .or_else(|| curriculums.first()).ok_or("Khan Academy returned no curriculums")?;
  let key = curriculum.get("key").and_then(|v|v.as_str()).ok_or("Curriculum has no key")?;
  let response = client.post("https://www.khanacademy.org/api/internal/graphql/LearnMenuQuery")
    .header("Accept", "application/json").header("Content-Type", "application/json")
    .header("Origin", "app:org.khanacademy.android")
    .json(&serde_json::json!({"fromGraphiql":false,"operationName":"LearnMenuQuery","query":MENU,"variables":{"curriculum":key}}))
    .send().await.map_err(|e| format!("Could not fetch Khan Academy learn menu for {key}: {e}"))?;
  let status = response.status();
  let body: serde_json::Value = response.json().await.map_err(|e| format!("Could not parse learn menu response (HTTP {status}): {e}"))?;
  if !status.is_success() { return Err(format!("Learn menu request returned HTTP {status}: {}", body)); }
  if let Some(errors) = body.get("errors") { return Err(format!("Learn menu GraphQL error: {errors}")); }
  let topics = body.pointer("/data/learnMenuTopics").and_then(|v|v.as_array()).ok_or("Khan Academy response omitted data.learnMenuTopics")?;
  let mut result = Vec::new();
  for topic in topics {
    let title = topic.get("translatedTitle").and_then(|v|v.as_str()).unwrap_or("Explore").to_string();
    let children = topic.get("children").and_then(|v|v.as_array()).cloned().unwrap_or_default();
    let courses: Vec<serde_json::Value> = children.into_iter().filter_map(|child| {
      let href = child.get("href").and_then(|v|v.as_str())?;
      let course_title = child.get("translatedTitle").and_then(|v|v.as_str()).unwrap_or(href);
      let mut item = serde_json::json!({"title":course_title,"href":href});
      if let Some(id) = child.get("courseId") { item["courseId"] = id.clone(); }
      Some(item)
    }).collect();
    if !courses.is_empty() { result.push(serde_json::json!({"group":title,"courses":courses})); }
  }
  if result.is_empty() { return Err(format!("Khan Academy returned no learn-menu entries for curriculum {key}")); }
  Ok(result)
}
