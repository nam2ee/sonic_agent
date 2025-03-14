use sonic_defai_net::build::build_server;
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    build_server().await;
}
