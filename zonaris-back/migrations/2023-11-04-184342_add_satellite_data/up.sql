CREATE TABLE satellite_data (
    id SERIAl PRIMARY KEY,
    satellite_id INT REFERENCES satellites (id) NOT NULL,
    path VARCHAR NOT NULL
)