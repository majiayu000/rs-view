use crate::db::{self, establish_connection};
use crate::error_handler::AccountError;
use crate::schema::accounts;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
// use self::models::*;
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "accounts"]
pub struct Accounts {
    pub id: i32,
    pub exchange: String,
    pub symbol: String,
    pub pricedot: i32,
    pub amountdot: i32,
    pub info: String,
    pub api_key: String,
    pub secret_key: String,
    pub extra: String,
}

impl Accounts {
    pub fn find_all() -> Result<Vec<Self>, AccountError> {
        // let conn = db::connection()?;
        let connection = &mut establish_connection();
        let accounts = accounts::table.load::<Accounts>(connection)?;
        Ok(accounts)
    }
}

