use crate::core::errors::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::warn;
use uuid::Uuid;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum Category {
    Pothole,
    Lighting,
    Sanitation,
    Other,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Pothole => "Buraco",
            Category::Lighting => "Iluminação",
            Category::Sanitation => "Saneamento",
            Category::Other => "Outro",
        }
    }

    pub fn badge_class(&self) -> &'static str {
        match self {
            Category::Pothole => "bg-amber-100 text-amber-800 border-amber-300",
            Category::Lighting => "bg-yellow-100 text-yellow-800 border-yellow-300",
            Category::Sanitation => "bg-blue-100 text-blue-800 border-blue-300",
            Category::Other => "bg-gray-100 text-gray-800 border-gray-300",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateReportCmd {
    pub title: String,
    pub description: String,
    pub category: Category,
    pub coord: Coordinate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub lon: f64,
    pub lat: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCluster {
    pub hash_id: String,
    pub report_count: i64,
    pub center_lon: f64,
    pub center_lat: f64,
}
