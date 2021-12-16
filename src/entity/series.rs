use serde::{Serialize, Deserialize};
use super::schema::series;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Series{
    pub id: i32,
    pub title: String,
    pub description : String,
    pub rating : i32,
}


#[derive(Insertable, Serialize, Deserialize)]
#[table_name="series"]
pub struct NewSeries<'a>{
    pub title: &'a str,
    pub description: &'a str,
    pub rating : i32
}