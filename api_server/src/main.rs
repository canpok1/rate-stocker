use log::info;
use std::env;

mod server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let port = env::var("PORT").expect("PORT must be set");

    let addr = format!("127.0.0.1:{}", port);

    info!("run server {}", addr);

    server::create(&addr).await;
}
