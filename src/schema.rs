diesel::table! {
    planets (id) {
        id -> Integer,
        swapi_id -> Integer,
        name -> Varchar,
        climate -> Text,
        terrain -> Text,
        orbital_period_days -> Text
    }
}
