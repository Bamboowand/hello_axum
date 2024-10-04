use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
