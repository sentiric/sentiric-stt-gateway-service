mod config;
mod app;
mod error;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    App::bootstrap().await?.run().await
}