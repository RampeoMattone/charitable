use sqlx::{mysql::MySqlPoolOptions};
use warp::{Filter};
use std::{convert::Infallible};


#[tokio::main]
async fn main() {
    // get a database url.
    // TODO - IMPLEMENT ENVIRONMENT VARIABLES
    let database_url = String::from("mysql://anoni:anoni@192.168.1.4:3306/anoni");
    // generate a connection pool for the database
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
        .expect("Failed to connect to the database");
    // prepare the service:
    // POST /reg/<code> => 200 OK, body "here is the code! <code>"
    let service = warp::
        post() // listen only for POST requests
        .and(warp::path!("reg" / String)) // make sure the user is accessing /reg/<capture>
        .and_then(move |path| { // make a service that runs on an async function
                // the pool is captured by the closure
                let pool = pool.clone(); // the pool is cloned and passed with ownership to the async
                async move {
                    //println!("ciao {}", path);
                    //println!("{:?}", sqlx::query("SHOW TABLES").execute(&pool).await.unwrap());
                    // TODO - add database actions
                    Ok::<String, Infallible>(format!("done!")) // async block returns a response for the client
                } // async block is returned as a future
            });

    warp::serve(service)
        .run(([127, 0, 0, 1], 3030))
        .await;
}