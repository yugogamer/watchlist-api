use rocket_sync_db_pools::{diesel, database};

#[database("main_db")]
struct Db(diesel::PgConnection);


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build().attach(Db::fairing());
    
    loader.launch().await
}