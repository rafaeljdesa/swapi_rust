USE swapi_db;

CREATE TABLE IF NOT EXISTS planets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    swapi_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    climate TEXT,
    terrain TEXT,
    orbital_period_days TEXT
);