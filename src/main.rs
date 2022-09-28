pub mod core;
use axum::{routing::get, Router};

fn app() {}

#[tokio::main]
async fn main() {
    app();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    println!("Server running at port 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
