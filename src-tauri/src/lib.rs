static KHAN_COOKIE_JAR: OnceLock<Arc<reqwest::cookie::Jar>> = OnceLock::new();

fn khan_client() -> Result<reqwest::Client, String> {
  let jar = KHAN_COOKIE_JAR.get_or_init(|| Arc::new(reqwest::cookie::Jar::default())).clone();
  reqwest::Client::builder()
    .cookie_provider(jar)
    .user_agent("KhanAcademy/8.5.1 (org.khanacademy.android; Android 35; UnknownDeviceName; Google; phone)")
    .build().map_err(|e| format!("Could not initialize Khan Academy client: {e}"))
}

async fn khan_graphql_post(operation: &str, query: &str, variables: serde_json::Value) -> Result<serde_json::Value, String> {
  let client = khan_client()?;
  let url = format!("https://www.khanacademy.org/api/internal/graphql/{operation}");
  let response = client.post(url).header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Origin", "app:org.khanacademy.android")
    .json(&serde_json::json!({"fromGraphiql":false,"operationName":operation,"query":query,"variables":variables}))
    .send().await.map_err(|e| format!("{operation} request failed: {e}"))?;
  let status = response.status();
  let text = response.text().await.map_err(|e| format!("{operation} response could not be read: {e}"))?;
  if !status.is_success() {
    return Err(format!("{operation} returned HTTP {status}: {}", text.chars().take(1000).collect::<String>()));
  }
  let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{operation} returned invalid JSON: {e}"))?;
  if let Some(errors) = value.get("errors") {
    return Err(format!("{operation} GraphQL error: {errors}"));
  }
  Ok(value)
}

#[tauri::command]
async fn start_device_auth() -> Result<serde_json::Value, String> {
  // Android initializes a phantom account before requesting a device authorization code.
  let phantom = "mutation LoginAsPhantomMutation { forcePhantom { user { kaid avatarUrl } error { code } } }";
  let _ = khan_graphql_post("LoginAsPhantomMutation", phantom, serde_json::json!({})).await?;
  let query = "mutation StartDeviceAuthMutation($input: StartDeviceAuthInput) { startDeviceAuth(input: $input) { userCode deviceCode verificationUrl expiresAt intervalSeconds error { code } } }";
  let result = khan_graphql_post("StartDeviceAuthMutation", query, serde_json::json!({"input":{"deviceModel":"Khan Academy Desktop"}})).await?;
  let request = result.pointer("/data/startDeviceAuth").cloned().ok_or("Khan Academy did not return a device authorization request")?;
  if let Some(error) = request.get("error").filter(|v| !v.is_null()) {
    return Err(format!("Khan Academy device authorization failed: {error}"));
  }
  for key in ["userCode","deviceCode","verificationUrl","expiresAt","intervalSeconds"] {
    if request.get(key).map_or(true, |v| v.is_null()) {
      return Err(format!("Khan Academy device authorization response omitted {key}"));
    }
  }
  Ok(request)
}

#[tauri::command]
async fn poll_device_auth(device_code: String) -> Result<serde_json::Value, String> {
  if device_code.trim().is_empty() { return Err("Missing device authorization code".into()); }
  let query = "mutation DeviceAuthTokenMutation($deviceCode: String!) { login: loginWithDeviceAuthToken(deviceCode: $deviceCode) { user { kaid nickname username avatarUrl points includesDistrictOwnedData isPhantom isChild isTeacher isParent hasCoach canModifyCoaches badgeCounts } error { code } } }";
  let result = khan_graphql_post("DeviceAuthTokenMutation", query, serde_json::json!({"deviceCode":device_code})).await?;
  let login = result.pointer("/data/login").cloned().unwrap_or(serde_json::Value::Null);
  if let Some(code) = login.pointer("/error/code").and_then(|v|v.as_str()) {
    return match code {
      "AUTHORIZATION_PENDING" | "SLOW_DOWN" | "TOO_MANY_REQUESTS" => Ok(serde_json::json!({"status":"pending"})),
      "EXPIRED" => Ok(serde_json::json!({"status":"expired"})),
      "DENIED" => Ok(serde_json::json!({"status":"denied"})),
      other => Err(format!("Khan Academy sign-in failed: {other}")),
    };
  }
  if let Some(user) = login.get("user").filter(|v| !v.is_null()) {
    return Ok(serde_json::json!({"status":"success","user":user}));
  }
  Ok(serde_json::json!({"status":"pending"}))
}

use std::sync::{Arc, OnceLock};\n#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![fetch_khan_course, fetch_khan_explore_menu, start_device_auth, poll_device_auth])
    .run(tauri::generate_context!())
    .expect("error while running Khan Academy Desktop");
}

#[tauri::command]
async fn fetch_khan_course(course_id: String) -> Result<serde_json::Value, String> {
  if course_id.trim().is_empty() || course_id.len() > 128 {
    return Err("Invalid course identifier".into());
  }
  let query = format!("{}\n", include_str!("../../src/khanCourseQuery.graphql"));
  let mut hash: u32 = 5381;
  for ch in query.chars().rev() {
    hash = hash.wrapping_mul(33) ^ (ch as u32);
  }
  let variables = serde_json::json!({"courseId": course_id});
  let variables_json = serde_json::to_string(&variables).map_err(|e| e.to_string())?;
  let client = khan_client()?;
  let mut url = reqwest::Url::parse("https://www.khanacademy.org/api/internal/graphql/CourseQueryV8?fastlyCacheable=persist_until_publish")
    .map_err(|e| e.to_string())?;
  url.query_pairs_mut().append_pair("hash", &hash.to_string()).append_pair("variables", &variables_json);
  let response = client.get(url).header("Accept", "application/json")
    .header("Origin", "app:org.khanacademy.android").send().await
    .map_err(|e| format!("Could not reach Khan Academy: {e}"))?;
  let status = response.status();
  let text = response.text().await.map_err(|e| format!("Could not read course response: {e}"))?;
  // Android's persisted GET is an optimization, not the only way to execute this query.
  // When the server has evicted/does not recognize the persisted hash, retry as a normal
  // GraphQL POST exactly as graphqlFetch does for non-cacheable operations.
  let body_text = if status.as_u16() == 400 && text.contains("No query found for this hash") {
    let endpoint = "https://www.khanacademy.org/api/internal/graphql/CourseQueryV8";
    let fallback = client.post(endpoint).header("Accept", "application/json")
      .header("Content-Type", "application/json")
      .header("Origin", "app:org.khanacademy.android")
      .json(&serde_json::json!({
        "fromGraphiql": false,
        "operationName": "CourseQueryV8",
        "query": query,
        "variables": variables
      })).send().await.map_err(|e| format!("Persisted query was unavailable and GraphQL POST failed: {e}"))?;
    let fallback_status = fallback.status();
    let fallback_text = fallback.text().await.map_err(|e| format!("Could not read GraphQL POST response: {e}"))?;
    if !fallback_status.is_success() {
      return Err(format!("Khan Academy CourseQueryV8 POST returned HTTP {fallback_status}: {}", fallback_text.chars().take(1000).collect::<String>()));
    }
    fallback_text
  } else {
    if !status.is_success() {
      return Err(format!("Khan Academy CourseQueryV8 returned HTTP {status}: {}", text.chars().take(1000).collect::<String>()));
    }
    text
  };
  let body: serde_json::Value = serde_json::from_str(&body_text)
    .map_err(|e| format!("Khan Academy CourseQueryV8 returned invalid JSON: {e}; body: {}", body_text.chars().take(800).collect::<String>()))?;
  if let Some(errors) = body.get("errors") {
    let messages = errors.as_array().map(|items| items.iter().filter_map(|e|e.get("message").and_then(|m|m.as_str())).collect::<Vec<_>>().join("; ")).unwrap_or_else(|| errors.to_string());
    return Err(format!("Khan Academy CourseQueryV8 GraphQL error: {messages}"));
  }
  Ok(body)
}

#[tauri::command]
async fn fetch_khan_explore_menu() -> Result<Vec<serde_json::Value>, String> {
  const ALL: &str = "query AllCurriculumsQuery { curriculums(includeEmptyCurriculum: true) { key country region name courseMenuRevision { sha __typename } __typename } __typename }";
  const MENU: &str = "query LearnMenuQuery($curriculum: String!) { learnMenuTopics(curriculum: $curriculum) { domainId translatedTitle icon children { courseId translatedTitle href __typename } __typename } __typename }";
  let client = khan_client()?;
  let response = client.post("https://www.khanacademy.org/api/internal/graphql/AllCurriculumsQuery")
    .header("Accept", "application/json").header("Content-Type", "application/json")
    .header("Origin", "app:org.khanacademy.android")
    .json(&serde_json::json!({"fromGraphiql":false,"operationName":"AllCurriculumsQuery","query":ALL,"variables":{}}))
    .send().await.map_err(|e| format!("Could not fetch Khan Academy curriculums: {e}"))?;
  let status = response.status();
  let text = response.text().await.map_err(|e| format!("Could not read curriculum response (HTTP {status}): {e}"))?;
  if !status.is_success() { return Err(format!("Curriculum request returned HTTP {status}: {}", text.chars().take(1000).collect::<String>())); }
  let body: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("Could not parse curriculum response: {e}"))?;
  if let Some(errors) = body.get("errors") { return Err(format!("AllCurriculumsQuery GraphQL error: {errors}")); }
  let curriculums = body.pointer("/data/curriculums").and_then(|v|v.as_array()).ok_or("Khan Academy response omitted data.curriculums")?;
  let eligible: Vec<&serde_json::Value> = curriculums.iter().filter(|c| c.pointer("/courseMenuRevision/sha").and_then(|v|v.as_str()).map_or(false, |v| !v.is_empty())).collect();
  if eligible.is_empty() { return Err("Khan Academy returned no curriculum with a published course menu revision".into()); }
  let mut result: Vec<serde_json::Value> = Vec::new();
  for curriculum in eligible {
    let key = curriculum.get("key").and_then(|v|v.as_str()).ok_or("Curriculum has no key")?;
    let response = client.post("https://www.khanacademy.org/api/internal/graphql/LearnMenuQuery")
      .header("Accept", "application/json").header("Content-Type", "application/json")
      .header("Origin", "app:org.khanacademy.android")
      .json(&serde_json::json!({"fromGraphiql":false,"operationName":"LearnMenuQuery","query":MENU,"variables":{"curriculum":key}}))
      .send().await.map_err(|e| format!("Could not fetch Khan Academy learn menu for {key}: {e}"))?;
    let status = response.status();
    let text = response.text().await.map_err(|e| format!("Could not read learn menu response (HTTP {status}): {e}"))?;
    if !status.is_success() { return Err(format!("LearnMenuQuery for {key} returned HTTP {status}: {}", text.chars().take(800).collect::<String>())); }
    let menu: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("Could not parse learn menu for {key}: {e}"))?;
    if let Some(errors) = menu.get("errors") { return Err(format!("LearnMenuQuery for {key} GraphQL error: {errors}")); }
    let topics = menu.pointer("/data/learnMenuTopics").and_then(|v|v.as_array()).ok_or("Khan Academy response omitted data.learnMenuTopics")?;
    for topic in topics {
      let title = topic.get("translatedTitle").and_then(|v|v.as_str()).unwrap_or("Explore").to_string();
      let domain_id = topic.get("domainId").cloned().unwrap_or(serde_json::Value::Null);
      let icon = topic.get("icon").cloned().unwrap_or(serde_json::Value::Null);
      let children = topic.get("children").and_then(|v|v.as_array()).cloned().unwrap_or_default();
      let courses: Vec<serde_json::Value> = children.into_iter().filter_map(|child| {
        let href = child.get("href").and_then(|v|v.as_str())?;
        let course_title = child.get("translatedTitle").and_then(|v|v.as_str())?;
        let mut item = serde_json::json!({"translatedTitle":course_title,"href":href});
        if let Some(id) = child.get("courseId") { item["courseId"] = id.clone(); }
        Some(item)
      }).collect();
      if !courses.is_empty() {
        if let Some(existing) = result.iter_mut().find(|entry| entry.get("domainId") == Some(&domain_id)) {
          if let Some(items) = existing.get_mut("children").and_then(|v|v.as_array_mut()) {
            for course in courses { if !items.iter().any(|item| item.get("href") == course.get("href")) { items.push(course); } }
          }
        } else {
          result.push(serde_json::json!({"domainId":domain_id,"translatedTitle":title,"icon":icon,"children":courses,"curriculum":key}));
        }
      }
    }
  }
  if result.is_empty() { return Err("Khan Academy returned no learn-menu entries across published curricula".into()); }
  Ok(result)
}
