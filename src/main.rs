use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zelajampa::api::{routes::app_router, state::AppState};

#[tokio::main]
async fn main() {
    // Carrega variáveis do arquivo local pra o ambiente
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "zelajampa=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("ZelaJampa Server Inicializando Infraestrutura...");

    let db_url = env::var("DATABASE_URL").expect("Variável DATABASE_URL ausente");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Falha massiva: Impossível conectar ao PostGIS");

    let state = AppState { db: pool };
    let app = app_router(state);

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    tracing::info!("ZelaJampa Server online! Escutando em {}", addr);

    axum::serve(listener, app).await.unwrap();
}
