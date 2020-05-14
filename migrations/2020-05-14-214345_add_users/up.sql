CREATE TABLE computations_dg_tmp
(
	id INTEGER NOT NULL
		PRIMARY KEY AUTOINCREMENT,
	hr_png BLOB,
	trimmed_png BLOB,
	distance_png BLOB,
	pm_png BLOB,
	correctly_clustered INTEGER,
	incorrectly_clustered INTEGER,
	accuracy REAL,
	anomaly INTEGER,
	email TEXT NOT NULL
);

INSERT INTO computations_dg_tmp(id, hr_png, trimmed_png, distance_png, pm_png, correctly_clustered, incorrectly_clustered, accuracy, anomaly, email) select id, hr_png, trimmed_png, distance_png, pm_png, correctly_clustered, incorrectly_clustered, accuracy, anomaly, email from computations;

DROP TABLE computations;

ALTER TABLE computations_dg_tmp rename TO computations;

