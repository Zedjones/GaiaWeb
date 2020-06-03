use crate::models::Computation;

impl Computation {
    fn bytes_to_str(bytes: &Option<Vec<u8>>) -> Option<String> {
        if let Some(hr_png) = bytes {
            Some(base64::encode(hr_png))
        }
        else {
            None
        }
    }
}

#[juniper::graphql_object]
impl Computation {
    fn id(&self) -> i32 {
        self.id
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn hr_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.hr_png)
    }

    fn trimmed_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.trimmed_png)
    }

    fn distance_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.distance_png)
    }

    fn pm_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.pm_png)
    }

    fn correctly_clustered(&self) -> Option<i32> {
        self.correctly_clustered
    }

    fn incorrectly_clustered(&self) -> Option<i32> {
        self.incorrectly_clustered
    }

    fn accuracy(&self) -> Option<f64> {
        self.accuracy.and_then(|val| Some(val as f64))
    }

    fn anomaly(&self) -> Option<i32> {
        self.anomaly
    }

    fn clusters(&self) -> Option<Vec<i32>> {
        self.clusters.as_ref().and_then(|cluster_str| {
            Some(serde_json::from_str::<Vec<i32>>(cluster_str).unwrap())
        })
    }
}