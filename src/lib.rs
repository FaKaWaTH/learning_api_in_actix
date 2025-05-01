mod routes;

pub async fn run() {
    let app = routes::create_route();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("\nrun on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
