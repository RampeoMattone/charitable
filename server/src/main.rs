use sqlx::{mysql::MySqlPoolOptions};
use warp::{Filter};
use std::{convert::Infallible};


#[tokio::main]
async fn main() {
    let database_url = String::from("mysql://anoni:anoni@192.168.1.4:3306/anoni");
    let _pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
        .expect("Failure to connect to the database");
    let pool = _pool.clone();
    // POST /reg/<code> => 200 OK, body "here is the code! <code>"
    let code = warp::post().and(warp::path!("reg" / String))
        .and_then(move |path| {
                let pool_clone = pool.clone();
                async move {
                    println!("ciao!!!! {}", path);
                    sqlx::query("SHOW TABLES").execute(&pool_clone).await.unwrap();
                    Ok::<String, Infallible>(format!("done!"))
                }
            });

    warp::serve(code)
        .run(([127, 0, 0, 1], 3030))
        .await;
}