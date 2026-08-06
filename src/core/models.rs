use crate::core::errors::AppError;
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub lon: f64,
    pub lat: f64,
}

impl Coordinate {
    pub fn new(lon: f64, lat: f64) -> Result<Self, AppError> {
        if !(-180.0..=180.0).contains(&lon) || !(-90.0..=90.0).contains(&lat) {
            warn!(
                lon,
                lat, "Tentativa de criar coordenada fora dos limites geográficos"
            );
            return Err(AppError::ValidationError(
                "Invalid geo coordinates".to_string(),
            ));
        }
        Ok(Self { lon, lat })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum Category {
    Pothole,
    Lighting,
    Sanitation,
    Other,
}

#[derive(Debug, Clone)]
pub struct CreateReportCmd {
    pub title: String,
    pub description: String,
    pub category: Category,
    pub coord: Coordinate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCluster {
    pub hash_id: String,
    pub report_count: i64,
    pub center_lon: f64,
    pub center_lat: f64,
}
