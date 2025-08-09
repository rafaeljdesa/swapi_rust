use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SwapiPlanet {
    pub name: String,
    pub climate: String,
    pub terrain: String,
    #[serde(rename = "orbital_period")]
    pub orbital_period_days: String,
}

pub async fn fetch_planet_by_id(id: i32) -> Option<SwapiPlanet> {
    let url: String = format!("https://swapi.dev/api/planets/{}", id);

    //let response = reqwest::get(&url).await.ok()?;

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = match client.get(&url).send().await {
        Ok(resp) => {
            println!("Status: {}", resp.status());
            resp
        }
        Err(e) => {
            println!("Erro ao fazer request: {:?}", e);
            return None;
        }
    };

    response.json::<SwapiPlanet>().await.ok()
}
