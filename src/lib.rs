mod api;
mod client;
mod domain;
mod infrastructure;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use config_loader::load_settings;
use dotenvy::dotenv;

use crate::{
    api::handlers,
    domain::repository::{PlanetRepository, Repository},
    infrastructure::{config_loader, db_pool::create_pool, feature_flags::{FeatureFlag, FeatureFlagLaunchDarkly}},
};
use actix_ws::Session;
use tokio::sync::RwLock;

pub struct AppState {
    pub sessions: RwLock<Vec<Session>>,
}

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();
    let settings = load_settings();
    let pool = create_pool(&settings.db.url);

    let repo_conc = Repository::new(pool);
    let ff_conc   = FeatureFlagLaunchDarkly::new().await;

    let repo_obj: Arc<dyn PlanetRepository> = Arc::new(repo_conc);
    let ff_obj:   Arc<dyn FeatureFlag>      = Arc::new(ff_conc);

    let repository = web::Data::from(repo_obj);
    let feature_flag_manager = web::Data::from(ff_obj);

    let app_state = web::Data::new(AppState {
        sessions: RwLock::new(Vec::new()),
    });

    let address = "0.0.0.0";
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
