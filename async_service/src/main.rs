mod calculation;
mod handlers;
mod models;
mod server;

use crate::server::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new();
    server::start_server(settings).await
}
