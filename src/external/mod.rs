use super::data::{
    ActivitiesResponse, ActivityRequest, ActivityResponse, EditActivityRequest, ErrorListResponse,
    SignInResponse,
};
use super::handlers::AnalyzerError;
use failure::{Error, ResultExt};
use reqwest::Response;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.timeular.com/api/v2";

pub fn get_activities(jwt: &str) -> Result<ActivitiesResponse, Error> {
    let activities_path = format!("{}/activities", BASE_URL);
    let result = get(&activities_path, jwt)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

pub fn get_activity(id: &str, jwt: &str) -> Result<ActivityResponse, AnalyzerError> {
    let activities_path = format!("{}/activities", BASE_URL);
    let result = get(&activities_path, jwt).map_err(|_| AnalyzerError::ExternalServiceError)?;
    let parsed: ActivitiesResponse =
        serde_json::from_str(&result).map_err(|_| AnalyzerError::ExternalServiceError)?;
    let activity = parsed
        .activities
        .iter()
        .cloned()
        .find(|activity| activity.id == id);
    match activity {
        Some(a) => Ok(a),
        None => Err(AnalyzerError::ActivityNotFoundError),
    }
}

pub fn create_activity(activity: &ActivityRequest, jwt: &str) -> Result<ActivityResponse, Error> {
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("name", &activity.name);
    body.insert("color", &activity.color);
    body.insert("integration", &activity.integration);
    let path = format!("{}/activities", BASE_URL);
    let result = post(&path, &body, jwt)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

pub fn edit_activity(
    id: &str,
    activity: &EditActivityRequest,
    jwt: &str,
) -> Result<ActivityResponse, Error> {
    let mut body: HashMap<&str, &str> = HashMap::new();
    let act = activity.clone();
    match act.name.as_ref() {
        Some(v) => body.insert("name", v),
        None => None,
    };
    match act.color.as_ref() {
        Some(v) => body.insert("color", v),
        None => None,
    };
    let path = format!("{}/activities/{}", BASE_URL, id);
    let result = patch(&path, &body, jwt)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

pub fn delete_activity(id: &str, jwt: &str) -> Result<ErrorListResponse, Error> {
    let path = format!("{}/activities/{}", BASE_URL, id);
    let result = delete(&path, jwt)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

pub fn get_jwt(api_key: &str, api_secret: &str) -> Result<String, Error> {
    let mut body = HashMap::new();
    body.insert("apiKey", api_key);
    body.insert("apiSecret", api_secret);
    let jwt_path = format!("{}/developer/sign-in", BASE_URL);
    let result = post(&jwt_path, &body, "")?;
    let json: Result<SignInResponse, Error> = serde_json::from_str(&result)
        .map_err(|e| format_err!("could not parse json, reason: {}", e));
    Ok(json.unwrap().token)
}

fn get(path: &str, jwt: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(path)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .context("error during get request")?;
    parse_result(res)
}

fn post(path: &str, body: &HashMap<&str, &str>, jwt: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(path)
        .json(&body)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .context("error during post request")?;
    parse_result(res)
}

fn patch(path: &str, body: &HashMap<&str, &str>, jwt: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .patch(path)
        .json(&body)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .context("error during patch request")?;
    parse_result(res)
}

fn delete(path: &str, jwt: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .delete(path)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .context("error during delete request")?;
    parse_result(res)
}

fn parse_result(mut res: Response) -> Result<String, Error> {
    let mut buf: Vec<u8> = vec![];
    if res.status().is_success() {
        res.copy_to(&mut buf)
            .context("could not copy response into buffer")?;
    } else {
        return Err(format_err!("request error: {}", res.status()));
    }
    let result = std::str::from_utf8(&buf)?;
    Ok(result.to_string())
}
