use crate::{
    domain::entities::{NewPlanet, Planet},
    infrastructure::db_pool::Pool,
};
use actix_web::{Error, error::ErrorInternalServerError};
use diesel::{prelude::*, r2d2::PooledConnection};
use mockall::automock;

type Connection = PooledConnection<diesel::r2d2::ConnectionManager<MysqlConnection>>;

#[automock]
pub trait PlanetRepository: Send + Sync {
    fn find_planet_by_id(&self, planet_id: i32) -> Result<Option<Planet>, Error>;
    fn insert_planet(&self, new_planet: &NewPlanet) -> Result<Planet, Error>;
}

pub struct Repository {
    pool: Pool
}

impl Repository {
    pub fn new(pool: Pool) -> Self {
        Repository { pool }
    }

    fn get_connection(&self) -> Result<Connection, Error> {
        let conn = self.pool.get().map_err(ErrorInternalServerError)?;
        Ok(conn)
    }
}

impl PlanetRepository for Repository {
    fn find_planet_by_id(&self, planet_id: i32) -> Result<Option<Planet>, Error> {
        let mut conn = self.get_connection()?;

        use crate::infrastructure::db_schema::planets::dsl::*;

        planets
            .filter(swapi_id.eq(planet_id))
            .first::<Planet>(&mut conn)
            .optional()
            .map_err(ErrorInternalServerError)
    }

    fn insert_planet(&self, new_planet: &NewPlanet) -> Result<Planet, Error> {
        let mut conn = self.get_connection()?;

        use crate::infrastructure::db_schema::planets;
        use crate::infrastructure::db_schema::planets::dsl::*;
        use diesel::Connection;

        let new_planet_copy = NewPlanet {
            swapi_id: new_planet.swapi_id.clone(),
            name: new_planet.name.clone(),
            climate: new_planet.climate.clone(),
            terrain: new_planet.terrain.clone(),
            orbital_period_days: new_planet.orbital_period_days.clone(),
        };

        let planet = conn
            .transaction::<Planet, diesel::result::Error, _>(|conn| {
                diesel::insert_into(planets)
                    .values(&new_planet_copy)
                    .execute(conn)?;

                planets::table
                    .order(planets::id.desc())
                    .select(Planet::as_select())
                    .first(conn)
            })
            .map_err(ErrorInternalServerError)?;

        Ok(planet)
    }
}
