use sqlx::PgPool;
use std::env;
use zelajampa::core::models::{Category, Coordinate, CreateReportCmd};
use zelajampa::infra::db::{get_clusters, insert_report};

#[tokio::test]
async fn test_db_geohash_clustering() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    sqlx::query!("TRUNCATE TABLE reports")
        .execute(&pool)
        .await
        .unwrap();

    let rep1 = CreateReportCmd {
        title: "Buraco grande".to_string(),
        description: "Perto do giradouro".to_string(),
        category: Category::Pothole,
        coord: Coordinate::new(-34.8465, -7.1192).unwrap(),
    };

    let rep2 = CreateReportCmd {
        title: "Poste sem luz".to_string(),
        description: "Poste 123".to_string(),
        category: Category::Lighting,
        coord: Coordinate::new(-34.8466, -7.1193).unwrap(),
    };

    insert_report(&pool, &rep1).await.unwrap();
    insert_report(&pool, &rep2).await.unwrap();

    let clusters = get_clusters(&pool, 6).await.unwrap();

    assert_eq!(
        clusters.len(),
        1,
        "Ambas denúncias deveriam estar no mesmo cluster"
    );
    assert_eq!(
        clusters[0].report_count, 2,
        "Cluster deve indicar 2 denúncias ativas"
    );
}
