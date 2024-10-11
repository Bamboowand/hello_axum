use std::env;

use axum::{routing::get, Router};
use clap::{Arg, Command};
use dotenv::dotenv;

async fn hello_world() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    println!("database_url: {}", database_url);
    println!("secret_key: {}", secret_key);

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
