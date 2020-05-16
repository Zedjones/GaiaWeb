table! {
    clusters (computation_id, cluster_number) {
        computation_id -> Integer,
        cluster_number -> Integer,
        stars_number -> Integer,
    }
}

table! {
    computations (id) {
        id -> Integer,
        email -> Text,
        csv_file -> Binary,
        hr_png -> Nullable<Binary>,
        trimmed_png -> Nullable<Binary>,
        distance_png -> Nullable<Binary>,
        pm_png -> Nullable<Binary>,
        correctly_clustered -> Nullable<Integer>,
        incorrectly_clustered -> Nullable<Integer>,
        accuracy -> Nullable<Float>,
        anomaly -> Nullable<Integer>,
        clusters -> Nullable<Text>,
    }
}

joinable!(clusters -> computations (computation_id));

allow_tables_to_appear_in_same_query!(
    clusters,
    computations,
);
