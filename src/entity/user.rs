use diesel::Queryable;
use rocket::http::Accept;
use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};
use super::schema::account;
use super::schema::session;

type DB = diesel::pg::Pg;


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Account{
    pub id: i32,
    pub username: String,
    pub email : String,
}

// Attention, cette implémentation est OBLIGATOIRE, le trait Queryable renvoit forcément tout les colone
impl Queryable<account::SqlType, DB> for Account{
    type Row = (i32, String, String, String);

    fn build(row: Self::Row) -> Self {
        return Account{
            id : row.0,
            username : row.1,
            email : row.2
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct Session{
    pub id : i32,
    pub id_account: i32,
}

#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[table_name="session"]
pub struct NewSession{
    pub id_account: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Login{
    pub email : String,
    pub password: String,
}