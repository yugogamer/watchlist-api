#[macro_use]
extern crate diesel;
use rocket_okapi::{request::{OpenApiFromRequest, RequestHeaderInput}, gen::OpenApiGenerator, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};
use rocket_sync_db_pools::{database};

mod entity;
mod service;
mod controller;

#[database("main_db")]
struct Db(diesel::PgConnection);


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build().attach(Db::fairing());
    let loader = controller::series::load_road(loader);
    let loader = loader.mount(
        "/doc/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/series/openapi.json".to_owned(),
            ..Default::default()
        }),
    );


    loader.launch().await
}

impl<'r> OpenApiFromRequest<'r> for Db {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}