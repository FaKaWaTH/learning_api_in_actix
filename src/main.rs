use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", get(index_html));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("\nrun on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn index_html() -> axum::response::Html<&'static str> {
    include_str!("../static/index.html").into()
}
