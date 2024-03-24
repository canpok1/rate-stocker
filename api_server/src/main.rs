//! Main binary entry point for openapi_client implementation.

#![allow(missing_docs)]

use log::info;

mod server;

/// Create custom server, wire it to the autogenerated router,
/// and pass it to the web server.
#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = "127.0.0.1:8080";

    info!("run server {}", addr);

    server::create(addr).await;
}
