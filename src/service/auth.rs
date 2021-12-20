use diesel::{PgConnection, QueryDsl};
use crate::diesel::RunQueryDsl;
use crate::entity::schema::account::dsl::*;
use crate::entity::schema::session::dsl::*;
use crate::entity::user::{Account, NewSession, Session};
use crate::entity::schema::session;
use crate::diesel::ExpressionMethods;


pub fn login(conn : &PgConnection, username_ : String ,password_ : String) -> Result<i32, String>{
    let result = account.filter(username.eq(username_)).filter(password.eq(password_)).first::<Account>(conn);

    match result{
        Ok(user) => {
            let inserted = NewSession{id_account: user.id};
            let db_result = diesel::insert_into(session::table).values(inserted).execute(conn);

            match db_result{
                Ok(_) => {
                    let session_id = session.filter(id_account.eq(user.id)).first::<Session>(conn);
                    match session_id {
                        Ok(session_id) => return Ok(session_id.id),
                        Err(_) => return Err("id error".to_owned()),
                    }
                },
                Err(_) => return Err("already logged".to_owned())
            }
        },
        Err(_) => return Err("password or login not found".to_owned()),
    };
}

pub fn logout(conn : &PgConnection, id_ : i32) -> bool{
    let result = diesel::delete(session::table).filter(session::id.eq(id_)).execute(conn);
    match result {
        Ok(_) => return true,
        Err(_) => return false,
    }
}