pub async fn run() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("\nrun on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn app() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(index_html))
}

async fn index_html() -> axum::response::Html<&'static str> {
    include_str!("../static/index.html").into()
}
