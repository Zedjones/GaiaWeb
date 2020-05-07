-- Your SQL goes here
CREATE TABLE computations (
    id SERIAL PRIMARY KEY,
    hr_png BLOB NOT NULL,
    trimmed_png BLOB NOT NULL,
    distance_png BLOB NOT NULL,
    pm_png BLOB,
    correctly_clustered INTEGER,

)