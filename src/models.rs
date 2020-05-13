use super::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "clusters"]
struct Cluster {
    pub computation_id: i32,
    pub cluster_number: i32,
    pub stars_number: i32
}