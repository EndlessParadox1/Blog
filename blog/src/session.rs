use axum::http::{header, HeaderMap};

const COOKIE_NAME: &str = "BLOG_SESSION_ID";

pub fn get_session_id(headers: &HeaderMap) -> Option<String> {
    let cookies = headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if cookies.is_empty() {
        return None;
    }
    let mut session_id: Option<String> = None;
    let cookies: Vec<&str> = cookies.split(';').collect();
    for cookie in cookies {
        let cookie_pair: Vec<&str> = cookie.split('=').collect();
        let cookie_name = cookie_pair[0].trim();
        let cookie_value = cookie_pair[1].trim();
        if cookie_name == COOKIE_NAME && !cookie_value.is_empty() {
            session_id = Some(cookie_value.to_string());
            break;
        }
    }
    session_id
}

pub fn set_session_id(value: &str) -> String {
    format!("{}={}", COOKIE_NAME, value)
}
