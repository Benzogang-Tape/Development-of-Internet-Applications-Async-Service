use crate::{
    calculation::calculate_all,
    models::{CalcRequest, CallbackPayload},
};
use actix_web::{post, web, HttpResponse};
use rand::Rng;
use reqwest::Client;
use std::env;
use tokio::spawn;
use tokio::time::{sleep, Duration};

/// POST /api/roof_angle_calculation/{id}/finish
/// Принимает запрос на расчёт, сразу возвращает 200 и запускает вычисления в фоне.
#[post("/api/roof_angle_calculation/{id}/finish")]
async fn handle_request(
    path: web::Path<i64>,
    req: web::Json<CalcRequest>,
) -> HttpResponse {
    let id = path.into_inner();
    let calc_request = req.into_inner();

    // Валидация: проверяем что есть хотя бы один материал
    if calc_request.materials.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Массив материалов не может быть пустым"
        }));
    }

    // Запускаем асинхронный расчёт в фоне
    start_calculation(id, calc_request);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Data received successfully"
    }))
}

/// Запускает вычисления в отдельной асинхронной задаче (аналог горутины в Go).
fn start_calculation(id: i64, req: CalcRequest) {
    spawn(async move {
        // Выполняем расчёт для всех материалов
        let results = calculate_all(&req);

        // Имитируем долгие вычисления (5–10 секунд)
        let delay = rand::thread_rng().gen_range(5..=10);
        sleep(Duration::from_secs(delay)).await;

        // Отправляем результат обратно на основной сервис
        if let Err(err) = send_callback(id, results).await {
            eprintln!("Ошибка отправки callback для заявки {}: {:?}", id, err);
        } else {
            println!("Callback для заявки {} успешно отправлен (задержка {} сек)", id, delay);
        }
    });
}

/// Отправляет результаты расчёта на основной сервис через callback.
async fn send_callback(
    id: i64,
    results: Vec<crate::models::MaterialResult>,
) -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let callback_base_url =
        env::var("CALLBACK_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let async_token =
        env::var("ASYNC_TOKEN").unwrap_or_else(|_| "secret".to_string());

    let url = format!(
        "{}/api/roof_angle_calculation/{}/callback",
        callback_base_url, id
    );

    let payload = CallbackPayload {
        async_token,
        results,
    };

    let client = Client::new();
    client
        .put(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// Регистрация маршрутов.
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_request);
}
