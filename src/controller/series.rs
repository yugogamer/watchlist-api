use rocket::{post, get};
use rocket_okapi::{openapi_get_routes, openapi, JsonSchema};
use rocket_okapi::okapi::schemars;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::service::series::*;
use crate::Db;

use crate::entity::series::{NewSeries, Series};




pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    return loader.mount("/series", openapi_get_routes![settings: get_series, add_series]);
}

/// # get a json
#[openapi]
#[get("/")]
async fn get_series(conn : Db) -> Json<Vec<Series>> {
    let result = conn.run(|c| get_list(c)).await;
    
    match result {
        Ok(list) => return Json(list),
        Err(_) => todo!(),
    }
}


/// # post a json
#[openapi]
#[post("/", format="json", data = "<new_series>")]
async fn add_series(new_series : Json<NewSeries>, conn : Db){
    let result = conn.run(move |c| create_series(c, &*new_series)).await;

    match result{
    Ok(_) => return,
    Err(_) => return,
}
}