use sqlx::{postgres::{PgPoolOptions}};
use warp::{Filter};
use std::{env, collections::HashMap};

mod database;

#[tokio::main]
async fn main() {
    // get a database url from an environment var
    match env::var("DATABASE_URL") {
        Ok(database_url) => {
            // generate a connection pool for the database
            let pool = PgPoolOptions::new()
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
                        match database::query(body, &pool).await { // using an ad hoc method to query the database and export the result idiomatically
                            Ok(result) => // if the database call succeeded (if we received a row of values that we could convert from the query)
                                    Ok(warp::reply::json(&result)), // reply to the client with a json-formatted response that matches what we extracted from the database
                            Err(e) => { // if the database call failed (either because of an improper query or because the parameter from the request body were missing)
                                if cfg!(debug_assertions) { // ONLY WORKS WHILE IN DEBUG BUILDS
                                    eprintln!("{:?}", e); // print the error we get to the error buffer
                                }
                                Err(warp::reject::not_found()) // reply to the client with an empty 404
                            }
                        }
                    } // async block is returned as a future
                });

            warp::serve(service)
                .run(([127, 0, 0, 1], 3030))
                .await;
        },
        Err(_) => {
            eprintln!("please set DATABASE_URL to a mysql database url!")
        },
    };
}
