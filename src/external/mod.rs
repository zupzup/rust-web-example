use super::data;
use failure::{Error, ResultExt};
use std::collections::HashMap;

pub fn do_request(path: &str, body: &HashMap<&str, String>) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let mut res = client
        .post(path)
        .json(&body)
        .send()
        .context("error during request")?;
    let mut buf: Vec<u8> = vec![];
    if res.status().is_success() {
        res.copy_to(&mut buf)
            .context("could not copy response into buffer")?;
    } else {
        return Err(format_err!("request error: {}", res.status()));
    }
    let result = std::str::from_utf8(&buf)?;
    let json: Result<data::SignInResponse, Error> =
        serde_json::from_str(result).map_err(|_| format_err!("could not parse json"));
    Ok(json.unwrap().token)
}
