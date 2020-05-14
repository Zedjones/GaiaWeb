use super::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "clusters"]
struct Cluster {
    pub computation_id: i32,
    pub cluster_number: i32,
    pub stars_number: i32
}

#[derive(Queryable, Insertable)]
#[table_name = "computations"]
struct Computation {
    pub id: i32,
    pub hr_png: Vec<u8>,
    pub trimmed_png: Vec<u8>,
    pub distance_png: Vec<u8>,
    pub pm_png: Option<Vec<u8>>,
    pub correctly_clustered: Option<i32>,
    pub incorrectly_clustered: Option<i32>,
    pub accuracy: Option<f32>,
    pub anomaly: Option<i32>
}