pub mod services;
mod app;

#[tokio::main]
async fn main() {
    app::init().await;
}
