use axum::Router;

pub async fn run_http_server(app: Router) {
    let listener = create_listener().await;
    axum::serve(listener, app).await.unwrap();
}

pub async fn create_listener() -> tokio::net::TcpListener {
    tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap()
}
