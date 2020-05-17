use super::schema::computations;
use super::DbPool;

use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use serde::{Serialize, Serializer, ser::SerializeSeq};

#[derive(Queryable)]
pub struct Cluster {
    pub computation_id: i32,
    pub cluster_number: i32,
    pub stars_number: i32
}

#[derive(Queryable, Debug, Serialize)]
pub struct Computation {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub csv_file: Vec<u8>,
    #[serde(serialize_with = "base64_serialize")]
    pub hr_png: Option<Vec<u8>>,
    #[serde(serialize_with = "base64_serialize")]
    pub trimmed_png: Option<Vec<u8>>,
    #[serde(serialize_with = "base64_serialize")]
    pub distance_png: Option<Vec<u8>>,
    #[serde(serialize_with = "base64_serialize")]
    pub pm_png: Option<Vec<u8>>,
    pub correctly_clustered: Option<i32>,
    pub incorrectly_clustered: Option<i32>,
    pub accuracy: Option<f32>,
    pub anomaly: Option<i32>,
    #[serde(serialize_with = "cluster_serialize")]
    pub clusters: Option<String>
}

///
/// We need this function to serialize our clusters.
/// Our clusters are in the format of a serialized list, so we need to deserialize
/// into a Vector before re-serializing, e.g.:
/// '[1, 2, 3]' -> Vec[1, 2, 3] -> (json) [1, 2, 3]
///
fn cluster_serialize<S>(cluster: &Option<String>, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if let Some(cluster) = cluster {
        let cluster_vec: Vec<i32> = serde_json::from_str(&cluster).unwrap();
        let mut seq = ser.serialize_seq(Some(cluster_vec.len()))?;
        for cluster in cluster_vec {
            seq.serialize_element(&cluster)?;
        }
        seq.end()
    }
    else {
        ser.serialize_seq(Some(0))?.end()
    }
}

fn base64_serialize<S>(bytes: &Option<Vec<u8>>, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if let Some(bytes) = bytes {
        ser.serialize_str(&base64::encode(bytes))
    }
    else {
        ser.serialize_none()
    }
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