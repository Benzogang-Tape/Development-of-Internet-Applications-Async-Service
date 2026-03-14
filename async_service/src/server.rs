use crate::handlers::init;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

/// Настройки сервера, загружаемые из переменных окружения.
pub struct Settings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("LISTEN_PORT")
                .unwrap_or_else(|_| "8083".to_string())
                .parse()
                .expect("LISTEN_PORT должен быть числом"),
        }
    }
}

/// Запуск HTTP-сервера на основе actix-web.
pub async fn start_server(settings: Settings) -> std::io::Result<()> {
    println!(
        "Async service запущен на {}:{}",
        settings.host, settings.port
    );

    HttpServer::new(move || App::new().configure(init))
        .bind((settings.host.clone(), settings.port))?
        .run()
        .await
}
