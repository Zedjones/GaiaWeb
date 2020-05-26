table! {
    computations (id) {
        id -> Integer,
        email -> Text,
        title -> Text,
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
