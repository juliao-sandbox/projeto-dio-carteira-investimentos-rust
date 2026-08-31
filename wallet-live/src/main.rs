mod app;
pub mod auth;
pub mod error;
pub mod models;
pub mod routes;

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
