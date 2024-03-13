use std::process;
use tokio;
use asmov_else_zone_server as server;

#[tokio::main]
async fn main() -> process::ExitCode {
    server::run().await
}
