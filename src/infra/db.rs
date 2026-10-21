use crate::core::errors::AppError;
use crate::core::models::{Category, CreateReportCmd, MapCluster, ReportItem};
use sqlx::PgPool;
use tracing::instrument;

#[instrument(skip(pool), err)]
pub async fn insert_report(pool: &PgPool, cmd: &CreateReportCmd) -> Result<ReportItem, AppError> {
    // Passamos Category como string nativa (via bind type) e geramos o Point via SQL PostGIS puro
    let row = sqlx::query!(
        r#"
        INSERT INTO reports (title, description, category, location)
        VALUES ($1, $2, $3, ST_SetSRID(ST_MakePoint($4, $5), 4326))
        RETURNING 
            id,
            title,
            description,
            category as "category: Category",
            ST_X(location::geometry) as "lon!",
            ST_Y(location::geometry) as "lat!",
            created_at
        "#,
        cmd.title,
        cmd.description,
        cmd.category as _, // Let sqlx map the enum to VARCHAR
        cmd.coord.lon,
        cmd.coord.lat
    )
    .fetch_one(pool)
    .await?;

    Ok(ReportItem {
        id: row.id,
        title: row.title,
        description: row.description,
        category: row.category,
        lon: row.lon,
        lat: row.lat,
        created_at: row.created_at,
    })
}

#[instrument(skip(pool), err)]
pub async fn get_recent_reports(pool: &PgPool, limit: i64) -> Result<Vec<ReportItem>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id,
            title,
            description,
            category as "category: Category",
            ST_X(location::geometry) as "lon!",
            ST_Y(location::geometry) as "lat!",
            created_at
        FROM reports
        ORDER BY created_at DESC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;

    let reports = rows
        .into_iter()
        .map(|r| ReportItem {
            id: r.id,
            title: r.title,
            description: r.description,
            category: r.category,
            lon: r.lon,
            lat: r.lat,
            created_at: r.created_at,
        })
        .collect();

    Ok(reports)
}

#[instrument(skip(pool), err)]
pub async fn get_clusters(pool: &PgPool, precision: i32) -> Result<Vec<MapCluster>, AppError> {
    // 6 chars ~ 1.2km | 7 chars ~ 153m
    // ST_Collect -> ST_Centroid acha o centro verdadeiro dos pontos do cluster
    let clusters = sqlx::query_as!(
        MapCluster,
        r#"
        SELECT 
            ST_GeoHash(location, $1) as "hash_id!",
            COUNT(*)::BIGINT as "report_count!",
            ST_X(ST_Centroid(ST_Collect(location))) as "center_lon!",
            ST_Y(ST_Centroid(ST_Collect(location))) as "center_lat!"
        FROM reports
        GROUP BY 1
        "#,
        precision
    )
    .fetch_all(pool)
    .await?;

    Ok(clusters)
}
