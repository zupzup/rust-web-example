#[macro_use]
extern crate slog;
extern crate actix_web;
#[macro_use]
extern crate failure;
use std::collections::HashMap;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use actix_web::{http, server, App, HttpRequest, Responder};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod data;
mod external;
mod logging;

const SECRETS_FILE: &str = "/Users/mario/dev/oss/rust/analyzer/me.secret";

// TODO: custom error response type
// (https://hgill.io/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/)

// TODO: look at scopeguard (defer macro)

// TODO: use https://github.com/rust-lang-nursery/failure for error management
fn read_credentials() -> Result<(String, String), String> {
    let file = File::open(SECRETS_FILE).expect("Could not open file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().take(2).map(|l| l.unwrap_or_default()).collect();
    if lines[0].is_empty() || lines[1].is_empty() {
        return Err(String::from("Could not read credentials."));
    }
    Ok((lines[0].to_owned(), lines[1].to_owned()))
}

// TODO: add route for time-entries and activities
// TODO: custom response type with nice JSON

fn index(_: &HttpRequest) -> impl Responder {
    "Hello, World!".to_string()
}

fn main() {
    let log = logging::setup_logging();
    info!(log, "Server Started on localhost:8080");
    let (api_key, api_secret) = match read_credentials() {
        Ok(v) => v,
        Err(e) => panic!("Could not get credentials: {}", e),
    };
    let mut jwt_body = HashMap::new();
    jwt_body.insert("apiKey", api_key);
    jwt_body.insert("apiSecret", api_secret);
    let jwt_path = "https://testing.timeular.com/auth-service/open-api/developer/sign-in";
    let jwt = match external::do_request(&jwt_path, &jwt_body) {
        Ok(v) => v,
        Err(e) => panic!("Could not get the JWT: {}", e),
    };
    info!(log, "JWT: {}", jwt);
    server::new(|| {
        App::new()
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .finish()
    })
    .bind("localhost:8080")
    .unwrap()
    .run();
}
