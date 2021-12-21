use std::result;

use rocket::http::Status;
use rocket::{post, get};
use rocket_okapi::{openapi_get_routes, openapi, JsonSchema};
use rocket_okapi::okapi::schemars;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::entity::user::Login;
use crate::service::auth;
use crate::Db;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    return loader.mount("/", openapi_get_routes![settings: login]);
}

/// # get a json
#[openapi]
#[post("/login", format = "json", data = "<data>")]
async fn login(conn : Db, data : Json<Login>) -> Result<Status, Status> {
    let result = conn.run(move |c| auth::login(c, data.email.clone(), data.password.clone())).await;
    
    match result{
        Ok(_) => return Ok(Status::Ok),
        Err(_) => return Err(Status::BadRequest),
    }
}