pub async fn index() -> axum::response::Html<&'static str> {
    include_str!("../../static/index.html").into()
}
