#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![fetch_khan_course])
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
