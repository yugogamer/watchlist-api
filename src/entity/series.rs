use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};
use super::schema::series;


#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct Series{
    pub id: i32,
    pub title: String,
    pub description : String,
    pub rating : i32,
}


#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[table_name="series"]
pub struct NewSeries{
    pub title: String,
    pub description: String,
    pub rating : i32
}