-- This file should undo anything in `up.sql`
CREATE TABLE computations_dg_tmp (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    hr_png BLOB NOT NULL,
    trimmed_png BLOB NOT NULL,
    distance_png BLOB NOT NULL,
    pm_png BLOB,
    correctly_clustered INTEGER,
    incorrectly_clustered INTEGER,
    accuracy REAL,
    anomaly INTEGER
);

DROP TABLE computations;

ALTER TABLE computations_dg_tmp RENAME TO computations;