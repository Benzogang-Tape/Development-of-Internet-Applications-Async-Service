use crate::models::{CalcRequest, MaterialResult, RoofingMaterial};

/// Вычисляет оптимальный угол наклона кровли для конкретного материала.
///
/// Формула: max(α_min, min(α_max, arctan((S_g / 200) * (W_0 / 50)) + (2/7) * α_min))
pub fn calculate_optimal_angle(
    snow_load: i32,
    wind_load: i32,
    material: &RoofingMaterial,
) -> i32 {
    let sg = snow_load as f64;
    let w0 = wind_load as f64;
    let alpha_min = material.min_angle as f64;
    let alpha_max = material.max_angle as f64;

    // Вычисляем по формуле
    let arctan_arg = (sg / 200.0) * (w0 / 50.0);
    let arctan_result = arctan_arg.atan();
    let arctan_degrees = arctan_result * (180.0 / std::f64::consts::PI);

    let calculated_angle = arctan_degrees + (2.0 / 7.0) * alpha_min;

    // Применяем ограничения: max(α_min, min(α_max, calculatedAngle))
    let result = alpha_min.max(alpha_max.min(calculated_angle));

    result.round() as i32
}

/// Выполняет расчёт оптимального угла для всех материалов в заявке.
pub fn calculate_all(req: &CalcRequest) -> Vec<MaterialResult> {
    req.materials
        .iter()
        .map(|material| MaterialResult {
            id: material.id,
            optimal_angle: calculate_optimal_angle(req.snow_load, req.wind_load, material),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_known_values() {
        // snow_load=200, wind_load=100 → arctan(1*2) = arctan(2) ≈ 63.43°
        // material: min=15, max=90
        // calculated = 63.43 + (2/7)*15 = 63.43 + 4.286 ≈ 67.71
        // clamped: max(15, min(90, 67.71)) = 67.71 → 68
        let material = RoofingMaterial {
            id: 1,
            min_angle: 15,
            max_angle: 90,
        };
        let result = calculate_optimal_angle(200, 100, &material);
        assert_eq!(result, 68);
    }

    #[test]
    fn test_calculate_clamped_to_min() {
        // Очень маленькие нагрузки → arctan(0) = 0
        // calculated = 0 + (2/7)*30 ≈ 8.57
        // clamped: max(30, min(60, 8.57)) = 30
        let material = RoofingMaterial {
            id: 2,
            min_angle: 30,
            max_angle: 60,
        };
        let result = calculate_optimal_angle(0, 0, &material);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_calculate_clamped_to_max() {
        // Большие нагрузки → arctan(huge) ≈ 90°
        // calculated = 90 + (2/7)*10 ≈ 92.86
        // clamped: max(10, min(45, 92.86)) = 45
        let material = RoofingMaterial {
            id: 3,
            min_angle: 10,
            max_angle: 45,
        };
        let result = calculate_optimal_angle(10000, 10000, &material);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_calculate_all() {
        let req = CalcRequest {
            snow_load: 200,
            wind_load: 100,
            materials: vec![
                RoofingMaterial {
                    id: 1,
                    min_angle: 15,
                    max_angle: 90,
                },
                RoofingMaterial {
                    id: 2,
                    min_angle: 30,
                    max_angle: 60,
                },
            ],
        };
        let results = calculate_all(&req);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, 1);
        assert_eq!(results[1].id, 2);
        // Проверяем что результаты в допустимых диапазонах
        assert!(results[0].optimal_angle >= 15 && results[0].optimal_angle <= 90);
        assert!(results[1].optimal_angle >= 30 && results[1].optimal_angle <= 60);
    }
}
