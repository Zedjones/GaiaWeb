-- This file should undo anything in `up.sql`
CREATE TABLE computations_dg_tmp
(
	id INTEGER NOT NULL
		PRIMARY KEY AUTOINCREMENT,
	email TEXT NOT NULL,
	csv_file BLOB NOT NULL,
	hr_png BLOB,
	trimmed_png BLOB,
	distance_png BLOB,
	pm_png BLOB,
	correctly_clustered INTEGER,
	incorrectly_clustered INTEGER,
	accuracy REAL,
	anomaly INTEGER,
    clusters TEXT
);

DROP TABLE computations;

ALTER TABLE computations_dg_tmp RENAME TO computations;