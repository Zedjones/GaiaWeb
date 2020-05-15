use super::schema::computations;
use super::DbPool;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

#[derive(Queryable)]
pub struct Cluster {
    pub computation_id: i32,
    pub cluster_number: i32,
    pub stars_number: i32
}

#[derive(Queryable)]
pub struct Computation {
    pub id: i32,
    pub email: String,
    pub csv_file: Vec<u8>,
    pub hr_png: Option<Vec<u8>>,
    pub trimmed_png: Option<Vec<u8>>,
    pub distance_png: Option<Vec<u8>>,
    pub pm_png: Option<Vec<u8>>,
    pub correctly_clustered: Option<i32>,
    pub incorrectly_clustered: Option<i32>,
    pub accuracy: Option<f32>,
    pub anomaly: Option<i32>
}

#[derive(Insertable)]
#[table_name = "computations"]
pub struct NewComputation {
    pub email: String,
    pub csv_file: Vec<u8>
}

impl NewComputation {
    pub fn insert_computation(&self, pool: &DbPool) -> Computation {

        use super::schema::computations::dsl::*;
        
        let db_conn = pool.get().unwrap();
        diesel::insert_into(super::schema::computations::table)
            .values(self)
            .execute(&db_conn)
            .expect("Error creating computation");
        // SQLite cannot return the last inserted, so let's get it ourselves
        computations
            .order(id.desc())
            .first(&db_conn)
            .unwrap()
    }
}