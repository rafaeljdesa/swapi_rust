mod config;
mod config_loader;
mod db;
mod feature_flags;
mod handlers;
mod models;
mod repository;
mod schema;
mod swapi;

use actix_web::{App, HttpServer, web};
use config_loader::load_settings;
use db::create_pool;
use dotenvy::dotenv;

use crate::{feature_flags::FeatureFlagManager, repository::Repository};
use actix_ws::Session;
use tokio::sync::RwLock;

pub struct AppState {
    pub sessions: RwLock<Vec<Session>>,
}

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();
    let settings = load_settings();
    let pool = create_pool(&settings.db.url);
    let repository = web::Data::new(Repository::new(pool));
    let feature_flag_manager = web::Data::new(FeatureFlagManager::new().await);
    let app_state = web::Data::new(AppState {
        sessions: RwLock::new(Vec::new()),
    });

    let address = "127.0.0.1";
    let port = 8080;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(repository.clone())
            .app_data(feature_flag_manager.clone())
            .app_data(app_state.clone())
            .service(handlers::get_planet_by_id)
            .service(handlers::ws_connect)
    })
    .bind((address, port))?;

    println!("🚀 Application is running at http://{address}:{port}");

    server.run().await
}