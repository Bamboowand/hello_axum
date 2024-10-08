use std::{env::args, fmt::format};

use axum::{routing::get, Router};
use clap::{Arg, Command};

async fn hello_world() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let matches = Command::new("Hello axum")
        .version("1.0")
        .author("Jacob")
        .about("Does axum")
        .arg(
            Arg::new("port")
                .help("Api output")
                .default_value("3000")
                .long("port")
                .short('p'),
        )
        .arg(Arg::new("verbose").help("Print more output"))
        .get_matches();
    let port = matches.get_one::<String>("port").unwrap();
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
