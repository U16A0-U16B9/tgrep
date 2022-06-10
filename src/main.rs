mod app;
pub mod services;

#[tokio::main]
async fn main() {
    app::init().await;
}
