use actix_web::{get, web, HttpResponse, Result, error::ErrorInternalServerError};
use crate::{feature_flags::FeatureFlagManager, models::{NewPlanet, Planet}, repository::Repository, swapi};

#[get("/api/v1/planets/swapi/{id}")]
pub async fn get_planet_by_id(
    repository: web::Data<Repository>,
    feature_flag_manager: web::Data<FeatureFlagManager>,
    path: web::Path<i32>
) -> Result<HttpResponse> {
    let planet_id = path.into_inner();
    let is_forcing_api_call = feature_flag_manager.is_forcing_api_call();

    println!("Forcing API call value is {}", is_forcing_api_call);

    if !is_forcing_api_call {
        // Try to find in the database
        let local_result: Option<Planet> = repository
            .find_planet_by_id(planet_id)
            .map_err(ErrorInternalServerError)?;

        if let Some(p) = local_result {
            println!("Found in the database");
            return Ok(HttpResponse::Ok().json(p));
        }

        println!("Not found in the database. Getting from external API.");
    }

    // Otherwise, find in the API
    if let Some(swapi_p) = swapi::fetch_planet_by_id(planet_id).await {
        let new_planet = NewPlanet {
            swapi_id: planet_id,
            name: swapi_p.name,
            climate: swapi_p.climate,
            terrain: swapi_p.terrain,
            orbital_period_days: swapi_p.orbital_period_days
        };

        // Update database
        let result = repository
            .insert_planet(&new_planet)
            .map_err(ErrorInternalServerError)?;

        return Ok(HttpResponse::Ok().json(result));
    }

    Ok(HttpResponse::NotFound().body("Planet not found"))
}
