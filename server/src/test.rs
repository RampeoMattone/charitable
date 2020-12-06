use std::{
    net::SocketAddr,
    convert::Infallible,
    sync::mpsc
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
use std::sync::Arc;
use futures::lock::Mutex;

#[tokio::main]
pub async fn start() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let (tx, rx) = mpsc::channel();

    async fn inner (tx: mpsc::Sender<String>, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let mut response = Response::new(Body::empty());
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/register") => {
                drop(tx);
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
                tx.send(String::from("oh shit"));
                drop(tx);
            },
        };

        Ok::<_, Infallible>(response)
    }
    let make_svc = make_service_fn(|_conn| {
        async move {
            let _test = String::from("ciao");
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(|req| inner(tx.clone() , req)))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Ok(string) = rx.recv() {
        println!("server error: {}", string);
    }
}
