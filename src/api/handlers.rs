use crate::{
    AppState,
    client::swapi,
    domain::{
        entities::{NewPlanet, Planet},
        repository::Repository,
    },
    infrastructure::feature_flags::FeatureFlagManager,
};
use actix_web::{
    Error, HttpRequest, HttpResponse, Result, error::ErrorInternalServerError, get, rt, web,
};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

#[get("/api/v1/planets/swapi/{id}")]
pub async fn get_planet_by_id(
    repository: web::Data<Repository>,
    feature_flag_manager: web::Data<FeatureFlagManager>,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
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
            orbital_period_days: swapi_p.orbital_period_days,
        };

        // Update database
        let result = repository
            .insert_planet(&new_planet)
            .map_err(ErrorInternalServerError)?;

        // Broadcast to all connected websocket sessions; drop closed ones
        let mut to_remove: Vec<usize> = Vec::new();
        let mut sessions = app_state.sessions.write().await;
        println!("Session count for broadcasting: {}", sessions.len());
        for (idx, s) in sessions.iter_mut().enumerate() {
            println!("Session found, broadcasting!");
            if s.text(format!("{:?}", &result)).await.is_err() {
                println!("Session error, mark to clean!");
                to_remove.push(idx);
            }
        }
        if !to_remove.is_empty() {
            // remove from the end to keep indices valid
            to_remove.sort_unstable();
            for i in to_remove.into_iter().rev() {
                let _ = sessions.remove(i);
            }
        }

        return Ok(HttpResponse::Ok().json(result));
    }

    Ok(HttpResponse::NotFound().body("Planet not found"))
}

#[get("/api/v1/planets/ws-connect")]
pub async fn ws_connect(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    {
        let mut sessions = app_state.sessions.write().await;
        sessions.push(session.clone());
        println!("Sessions count: {}", sessions.len());
    }

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        // Cleanup is handled on send failures, so no special action required on stream end.
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
