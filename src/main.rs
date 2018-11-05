#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
#[macro_use]
extern crate slog;
extern crate actix_web;
#[macro_use]
extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use actix_web::{http, server, App};
use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod data;
mod external;
mod handlers;
mod logging;

const SECRETS_FILE: &str = "./me.secret";

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "API_KEY", default = "")]
    pub api_key: String,

    #[envconfig(from = "API_SECRET", default = "")]
    pub api_secret: String,
}

#[derive(Debug)]
pub struct AppState {
    jwt: String,
    log: slog::Logger,
}

fn get_credentials(config: &Config) -> Result<(String, String), Error> {
    if config.api_key != "" && config.api_secret != "" {
        return Ok((config.api_key.to_string(), config.api_secret.to_string()));
    }
    let file = File::open(SECRETS_FILE).expect("Could not open file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .take(2)
        .map(std::result::Result::unwrap_or_default)
        .collect();
    if lines[0].is_empty() || lines[1].is_empty() {
        return Err(format_err!(
            "The first line needs to be the apiKey, the second line the apiSecret"
        ));
    }
    Ok((lines[0].to_string(), lines[1].to_string()))
}

fn main() {
    let log = logging::setup_logging();
    let config = match Config::init() {
        Ok(v) => v,
        Err(e) => panic!("Could not read config from environment: {}", e),
    };
    let (api_key, api_secret) = match get_credentials(&config) {
        Ok(v) => v,
        Err(e) => panic!("Could not get credentials: {}", e),
    };
    let jwt = match external::get_jwt(&api_key, &api_secret) {
        Ok(v) => v,
        Err(e) => panic!("Could not get the JWT: {}", e),
    };
    info!(log, "Server Started on localhost:8080");
    server::new(move || {
        App::with_state(AppState {
            jwt: jwt.to_string(),
            log: log.clone(),
        })
        .scope("/rest/v1", |v1_scope| {
            v1_scope.nested("/activities", |activities_scope| {
                activities_scope
                    .resource("", |r| {
                        r.method(http::Method::GET).f(handlers::get_activities);
                        r.method(http::Method::POST)
                            .with_config(handlers::create_activity, |cfg| {
                                (cfg.0).1.error_handler(handlers::json_error_handler);
                            })
                    })
                    .resource("/{activity_id}", |r| {
                        r.method(http::Method::GET).with(handlers::get_activity);
                        r.method(http::Method::DELETE)
                            .with(handlers::delete_activity);
                        r.method(http::Method::PATCH)
                            .with_config(handlers::edit_activity, |cfg| {
                                (cfg.0).1.error_handler(handlers::json_error_handler);
                            });
                    })
            })
        })
        .resource("/health", |r| {
            r.method(http::Method::GET).f(handlers::health)
        })
        .finish()
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run();
}
