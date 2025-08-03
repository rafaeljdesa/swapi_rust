mod schema;
mod models;
mod db;
mod swapi;
mod handlers;
mod config_loader;
mod config;
mod repository;
mod feature_flags;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use config_loader::load_settings;
use db::create_pool;

use crate::{feature_flags::FeatureFlagManager, repository::Repository};

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
            .service(handlers::get_planet_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
