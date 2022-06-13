mod app;
pub mod objects;
pub mod services;

#[tokio::main]
async fn main() {
    app::init().await;
}
