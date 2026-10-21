use askama::Template;
use axum::{
    Form, Json,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use tracing::{error, info};

use crate::api::state::AppState;
use crate::core::errors::AppError;
use crate::core::models::{Category, Coordinate, CreateReportCmd, ReportItem};
use crate::infra::db;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub reports: Vec<ReportItem>,
}

#[derive(Template)]
#[template(path = "report_row.html")]
pub struct ReportRowTemplate<'a> {
    pub report: &'a ReportItem,
}

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let reports = db::get_recent_reports(&state.db, 20).await?;
    let template = IndexTemplate { reports };
    let body = template.render().map_err(|e| {
        error!("IndexTemplate rendering failed: {e}");
        AppError::InternalError
    })?;
    Ok(Html(body))
}

// Payload simplificado liso, feito para receber direto do <form> HTMX
#[derive(Deserialize, Debug)]
pub struct ReportFormPayload {
    pub title: String,
    pub description: String,
    pub category: String,
    pub lon: f64,
    pub lat: f64,
}

pub async fn create_report(
    State(state): State<AppState>,
    Form(payload): Form<ReportFormPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!("Nova denúncia recebida: {}", payload.title);

    let category = match payload.category.as_str() {
        "Pothole" => Category::Pothole,
        "Lighting" => Category::Lighting,
        "Sanitation" => Category::Sanitation,
        _ => Category::Other,
    };

    let cmd = CreateReportCmd {
        title: payload.title,
        description: payload.description,
        category,
        coord: Coordinate::new(payload.lon, payload.lat)?, // Falha aqui vira AppError (400)
    };

    let report = db::insert_report(&state.db, &cmd).await?; // Falha aqui vira AppError (500)
    let template = ReportRowTemplate { report: &report };
    let html = template.render().map_err(|e| {
        error!("ReportRowTemplate rendering failed: {e}");
        AppError::InternalError
    })?;

    Ok((StatusCode::CREATED, Html(html)))
}

#[derive(Deserialize)]
pub struct ClusterQuery {
    pub precision: i32,
}

pub async fn get_clusters(
    State(state): State<AppState>,
    Query(query): Query<ClusterQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Nível de segurança basilar pra precisão
    let precision = query.precision.clamp(1, 9);
    let clusters = db::get_clusters(&state.db, precision).await?;

    Ok(Json(clusters))
}
