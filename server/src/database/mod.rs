use serde;
use sqlx;
use sqlx::{Pool, Postgres, Error};
use std::collections::HashMap;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct ResultSet {
    status: bool,
    code: String,
    user: String,
}

pub async fn query(body: HashMap<String, String>, pool: &Pool<Postgres>) -> Result<ResultSet, Error> {
    match (body.get("user"), body.get("code")) { // using a match statement to make sure we get all the required parameters
        (Some(user), Some(code)) => { // if all the correct parameters are present we extract them and generate a proper response
            sqlx::query_as::<_, ResultSet>("select status, _code as code, _user as user from register( $1 , $2 ) as (status bool, _code bpchar, _user bpchar);")
                .bind(code)
                .bind(user) // use an ad-hoc procedure to register the code to a user
                .fetch_one(pool).await // if the code cannot be registered, the procedure will return false, else it will return true
        }
        _ => {
            Err(Error::ColumnNotFound(String::from("failed to find column 'user' or 'code'")))
        }
    }
}