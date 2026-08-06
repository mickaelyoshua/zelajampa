use crate::core::errors::AppError;
use crate::core::models::{CreateReportCmd, MapCluster};
use sqlx::PgPool;
use tracing::instrument;

#[instrument(skip(pool), err)]
pub async fn insert_report(pool: &PgPool, cmd: &CreateReportCmd) -> Result<(), AppError> {
    // Passamos Category como string nativa (via bind type) e geramos o Point via SQL PostGIS puro
    sqlx::query!(
        r#"
        INSERT INTO reports (title, description, category, location)
        VALUES ($1, $2, $3, ST_SetSRID(ST_MakePoint($4, $5), 4326))
        "#,
        cmd.title,
        cmd.description,
        cmd.category as _, // Let sqlx map the enum to VARCHAR
        cmd.coord.lon,
        cmd.coord.lat
    )
    .execute(pool)
    .await?;

    Ok(())
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
