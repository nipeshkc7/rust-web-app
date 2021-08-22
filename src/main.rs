#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate rocket;

use askama::Template;
use dotenv::dotenv;
use rocket::Build;
use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::figment::map;
use rocket::figment::value::Map;
use rocket::figment::value::Value;
use rocket::response::content;
use rocket::response::status::Created;
use rocket::response::status::NotFound;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use std::env;
use std::env::VarError;
use std::num::ParseIntError;
use std::path::PathBuf;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {
    confession: Option<String>,
    total_confessions: i64
}

#[get("/")]
async fn root() -> Result<content::Html<String>, NotFound<String>> {
    let template = HomepageTemplate {
        confession: Some(String::from("random text")),
        total_confessions: 23
    };

    let response = content::Html(template.to_string());
    Ok(response)
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("site").join(path);
    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, static_files])
}