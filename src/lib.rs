mod error;
mod routes;
mod web;

use self::error::{Error, Result};

//  startup the server
pub async fn run() {
    //  create the routes
    let app = routes::create_route();

    //  running on localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("\nrun on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
