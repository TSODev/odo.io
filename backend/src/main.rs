use axum::{
    extract::{State, Request},
    routing::get,
    Json, Router, http::StatusCode,
    middleware::{self, Next},
    response::Response,
};
use sqlx::PgPool;
use common::{Vehicle, AccessRole, VehicleWithAccess};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

use tracing::{info, error, instrument};
use tracing_subscriber;
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

#[tokio::main]
async fn main() {

    // 1. Charge les variables du fichier .env dans l'environnement du processus
    dotenv().ok(); 
    let db_url = std::env::var("DATABASE_URL").expect("La variable DATABASE_URL doit être définie dans le fichier .env");
    let pool = PgPool::connect(&db_url).await.expect("Impossible de se connecter à la base de données Neon");

    // 1. Initialise le tracing
    tracing_subscriber::fmt()
        .pretty() // Ajoute des couleurs et indente le JSON/Structs
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    //        .add_directive(tracing::Level::DEBUG.into()))
        .init();

    info!("Le backend démarre...");

    // Configuration CORS
    let cors = CorsLayer::new()
        // Autorise ton frontend (ex: Tauri ou React)
        // .allow_origin("http://localhost:1420".parse::<HeaderValue>().unwrap()) 
        // Ou plus simple pour le dev :
        .allow_origin(Any) 
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/vehicles", get(list_vehicles))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
        )
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Backend ODO lancé sur http://{}", addr);
    info!("Connexion à la base de données Neon réussie !");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- Handler pour lister les véhicules ---
async fn list_vehicles(
    State(pool): State<PgPool>,
    // En production, on ajouterait ici notre extracteur ClaimsUser
) -> Result<Json<Vec<VehicleWithAccess>>, StatusCode> {
    
    // Simulation d'un ID utilisateur (à remplacer par user.0.sub plus tard)
    let mock_user_id = Uuid::parse_str("a7985c6d-7acd-4384-ade3-c5764dd8edf0").unwrap();

    let vehicles = sqlx::query_as!(
        VehicleWithAccess,
        r#"
        SELECT v.id as "id!", v.make, v.model, v.plate_number, v.owner_id, va.role as "my_role: AccessRole"
        FROM public.vehicles v
        JOIN public.vehicle_access va ON v.id = va.vehicle_id
        WHERE va.user_id = $1
        "#,
        mock_user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Erreur SQL: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(vehicles))
}