#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel_migrations;

use askama::Template;
use diesel::OptionalExtension;
use diesel::{RunQueryDsl, QueryDsl};
use dotenv::dotenv;
use errors::CustomError;
use models::Confession;
use models::NewConfession;
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
use diesel::pg::PgConnection;
use thiserror::Error;
use serde::{Deserialize, Serialize};

mod schema;
mod models;
mod errors;

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {
    confession: Option<String>,
    total_confessions: i64
}

#[get("/")]
async fn root() -> Result<content::Html<String>, CustomError> {
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