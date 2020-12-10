use serde::Serialize;
use sqlx::{Pool, MySql, Error, Row, FromRow, Decode, Type, ColumnIndex};
use std::collections::HashMap;

#[derive(Serialize)]
//pub struct Output(pub bool, pub String, pub String);
pub struct Output{
    success: bool,
    code: String,
    user: String
}

impl<'r, R> FromRow<'r, R> for Output where
    R: Row,
    bool: Decode<'r, <R as Row>::Database> + Type<<R as Row>::Database>,
    String: Decode<'r, <R as Row>::Database> + Type<<R as Row>::Database>,
    //&'r str: ColumnIndex<R>, // currently not working. possible issue with the api. for now only use column ordinals
    usize: ColumnIndex<R> {
    fn from_row(row: &'r R) -> Result<Self, Error> {
        match (
            row.try_get::<bool, _>(0),
            row.try_get::<String, _>(1),
            row.try_get::<String, _>(2))
        {
            (Ok(success), Ok(code), Ok(user)) => Ok(Output{success, code, user}),
            (Err(e), _, _) | (_, Err(e), _) |(_, _, Err(e))  => {
                //eprintln!("{:?}\n{:?}\n{:?}\n", success, code, user);
                Err(e)
            }
        }
    }
}

impl Output {
    pub async fn new(body: HashMap<String, String>, pool: &Pool<MySql>) -> Result<Self, Error> {
        match (body.get("user"), body.get("code")) { // using a match statement to make sure we get all the required JSON parameters
            (Some(user), Some(code)) => { // if all the correct parameters are present we extract them and generate a proper response
                sqlx::query_as::<_, Output>(
                    //let result = sqlx::query(
                    "call charitable.register( ? , ? )")
                    .bind(code)
                    .bind(user) // use an ad-hoc procedure to register the code to a user
                    .fetch_one(pool).await // if the code cannot be registered, the procedure will return false, else it will return true
            }
            _ => {
                Err(Error::ColumnNotFound(String::from("failed to find column 'user' or 'code'")))
            }
        }
    }
}
