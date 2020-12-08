use sqlx::{mysql::MySqlPoolOptions};
use warp::{Filter};
use std::{collections::HashMap};

mod database;

#[tokio::main]
async fn main() {
    // get a database url.
    // TODO - IMPLEMENT ENVIRONMENT VARIABLES
    let database_url = String::from("mysql://user:@pi:3306/charitable");
    // generate a connection pool for the database
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
        .expect("Failed to connect to the database");
    // prepare the service:
    // POST /reg/<code> => 200 OK, body "here is the code! <code>"
    let service = warp::post() // listen only for POST requests
        .and(warp::path!("reg")) // make sure the user is accessing /reg/<capture>
        .and(warp::body::content_length_limit(4096)) // make sure the body is not too big
        .and(warp::body::json())
        .and_then(move |body: HashMap<String, String>| { // make a service that runs on an async function
            // the pool is captured by the closure
            let pool = pool.clone(); // the pool is cloned and passed with ownership to the async
            async move {
                match (body.get("user"), body.get("code")) { // using a match statement to make sure we get all the required JSON parameters
                    (Some(user), Some(code)) => { // if all the correct parameters are present we extract them and generate a proper response
                        let result = sqlx::query!("call verify(?, ?)", code, user)
                            .fetch_one(&pool).await.unwrap();
                        if let Ok(true) = sqlx::Row::try_get(&result, 0) {

                            Ok(format!("hai chiesto di registrare {} sotto {}.\n\
                                        Il codice da te richiesto è libero?: sì\n",
                                       code, user))
                        } else {
                            Err(warp::reject::not_found())
                        }
                    }// async block returns a response for the client
                    _ => Err(warp::reject::not_found()) // if any of the two parameters are absent then we send a not found code and close it off like that
                }
            } // async block is returned as a future
        });

    warp::serve(service)
        .run(([127, 0, 0, 1], 3030))
        .await;
}