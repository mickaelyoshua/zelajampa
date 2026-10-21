use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use sqlx::PgPool;
use std::env;
use tokio::sync::Mutex;
use tower::ServiceExt;
use zelajampa::api::{routes::app_router, state::AppState};
use zelajampa::core::models::{Category, Coordinate, CreateReportCmd, MapCluster};
use zelajampa::infra::db;

static TEST_MUTEX: Mutex<()> = Mutex::const_new(());

async fn setup() -> (tokio::sync::MutexGuard<'static, ()>, AppState) {
    let guard = TEST_MUTEX.lock().await;
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    sqlx::query!("TRUNCATE TABLE reports")
        .execute(&pool)
        .await
        .unwrap();

    (guard, AppState { db: pool })
}

#[tokio::test]
async fn index_returns_ok_html() {
    let (_guard, state) = setup().await;
    let pool = state.db.clone();
    let app = app_router(state);

    let cmd = CreateReportCmd {
        title: "Buraco na Epitácio Pessoa".to_string(),
        description: "Próximo ao supermercado".to_string(),
        category: Category::Pothole,
        coord: Coordinate::new(-34.8465, -7.1192).unwrap(),
    };
    db::insert_report(&pool, &cmd).await.unwrap();

    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(body.contains("ZelaJampa"));
    assert!(body.contains("Buraco na Epitácio Pessoa"));
    assert!(body.contains("id=\"map\""));
}

#[tokio::test]
async fn create_report_valid_returns_created_and_html_row() {
    let (_guard, state) = setup().await;
    let app = app_router(state);

    let body = "title=Buraco+na+rua&description=Perto+do+mercado&category=Pothole&lon=-34.8465&lat=-7.1192";
    let request = Request::builder()
        .method("POST")
        .uri("/reports")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(body.contains("Buraco na rua"));
    assert!(body.contains("Buraco"));
}

#[tokio::test]
async fn create_report_invalid_coords_returns_bad_request() {
    let (_guard, state) = setup().await;
    let app = app_router(state);

    let body = "title=Invalid&description=Test&category=Pothole&lon=999.0&lat=999.0";
    let request = Request::builder()
        .method("POST")
        .uri("/reports")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_clusters_returns_ok_with_data() {
    let (_guard, state) = setup().await;
    let pool = state.db.clone();
    let app = app_router(state);

    let cmd = CreateReportCmd {
        title: "Test report".to_string(),
        description: "Test description".to_string(),
        category: Category::Pothole,
        coord: Coordinate::new(-34.8465, -7.1192).unwrap(),
    };
    db::insert_report(&pool, &cmd).await.unwrap();

    let request = Request::builder()
        .uri("/clusters?precision=7")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let clusters: Vec<MapCluster> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].report_count, 1);
}

#[tokio::test]
async fn get_clusters_missing_precision_returns_bad_request() {
    let (_guard, state) = setup().await;
    let app = app_router(state);

    let request = Request::builder()
        .uri("/clusters")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_report_unknown_category_defaults_to_other() {
    let (_guard, state) = setup().await;
    let app = app_router(state);

    let body = "title=Problema&description=Algo+estranho&category=Alien&lon=-34.85&lat=-7.12";
    let request = Request::builder()
        .method("POST")
        .uri("/reports")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
