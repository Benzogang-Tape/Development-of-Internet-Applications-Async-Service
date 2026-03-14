use serde::{Deserialize, Serialize};

/// Кровельный материал из заявки (входящие данные)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoofingMaterial {
    pub id: i64,
    pub min_angle: i32,
    pub max_angle: i32,
}

/// Тело входящего POST-запроса на расчёт
#[derive(Serialize, Deserialize, Debug)]
pub struct CalcRequest {
    pub snow_load: i32,
    pub wind_load: i32,
    pub materials: Vec<RoofingMaterial>,
}

/// Результат расчёта для одного материала
#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialResult {
    pub id: i64,
    pub optimal_angle: i32,
}

/// Тело callback-запроса, отправляемого на основной сервис
#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackPayload {
    pub async_token: String,
    pub results: Vec<MaterialResult>,
}
