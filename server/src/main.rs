use std::{
    net::SocketAddr,
    convert::Infallible
};
use hyper::{Body,
            Request,
            Response,
            Server,
            Method,
            StatusCode,
            service::{make_service_fn,
                      service_fn
            }
};

mod database;
mod test;

use crate::database::{Database};

#[tokio::main]
pub async fn main() {
    test::start()
    /*
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| {
        async move {
            let _test = String::from("ciao");
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(make_svc))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

     */
}

async fn make_svc(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/register") => {
            // Collects the whole body of the POST and generates a lossy converted utf8 string
            let _body_text = hyper::body::to_bytes(req.into_body())
                .await
                .expect("body collection went wrong")
                .iter()
                .cloned()
                .collect::<Vec<u8>>();
            let body_text = String::from_utf8_lossy(&_body_text);
            // generates a string which matches the encoded bytes of the received body
            //TODO: add database integration
            //let mut database = Database::connect("mysql://charitable:charitable@192.168.1.4:3001/charitable");
            println!("{}", body_text);
            //confirm that everything went well
            *response.status_mut() = StatusCode::OK;
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok::<_, Infallible>(response)
}
