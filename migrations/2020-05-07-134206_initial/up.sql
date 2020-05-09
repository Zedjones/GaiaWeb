-- Your SQL goes here
CREATE TABLE computations (
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

CREATE TABLE clusters
(
	computation_id INTEGER NOT NULL,
	cluster_number INTEGER NOT NULL,
	stars_number INTEGER NOT NULL,
	CONSTRAINT clusters_pk
		PRIMARY KEY (computation_id, cluster_number)
);