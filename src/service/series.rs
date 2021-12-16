use diesel::PgConnection;
use crate::entity::schema::series;
use crate::diesel::RunQueryDsl;
use crate::entity::schema::series::dsl::*;
use diesel::result::Error;

use crate::entity::series::{Series, NewSeries};

pub fn create_series(conn : &PgConnection, new_series : &NewSeries) -> Result<usize, diesel::result::Error>{
    return diesel::insert_into(series::table).values(new_series).execute(conn);
}

pub fn get_list(conn : &PgConnection) -> Result<Vec<Series>, Error> {
    series.load::<Series>(conn)
}