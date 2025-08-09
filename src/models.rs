use crate::schema::planets;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = planets)]
pub struct Planet {
    pub id: i32,
    pub swapi_id: i32,
    pub name: String,
    pub climate: String,
    pub terrain: String,
    pub orbital_period_days: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = planets)]
pub struct NewPlanet {
    pub swapi_id: i32,
    pub name: String,
    pub climate: String,
    pub terrain: String,
    pub orbital_period_days: String,
}
