mod config;
mod config_loader;
mod db;
mod feature_flags;
mod handlers;
mod models;
mod repository;
mod schema;
mod swapi;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use config_loader::load_settings;
use db::create_pool;

use crate::{feature_flags::FeatureFlagManager, repository::Repository};
use actix_ws::Session;
use tokio::sync::RwLock;

pub struct AppState {
    pub sessions: RwLock<Vec<Session>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = load_settings();
    let pool = create_pool(&settings.db.url);
    let repository = Arc::new(Repository::new(pool));
    let feature_flag_manager = Arc::new(FeatureFlagManager::new().await);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(repository.clone()))
            .app_data(web::Data::from(feature_flag_manager.clone()))
            .app_data(web::Data::new(AppState {
                sessions: RwLock::new(Vec::new()),
            }))
            .service(handlers::get_planet_by_id)
            .service(handlers::ws_connect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
