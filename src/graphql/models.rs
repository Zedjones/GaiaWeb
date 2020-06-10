use crate::models::Computation;

impl Computation {
    fn bytes_to_str(bytes: &Option<Vec<u8>>) -> Option<String> {
        if let Some(hr_png) = bytes {
            Some(base64::encode(hr_png))
        } else {
            None
        }
    }
}

#[async_graphql::Object]
impl Computation {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn hr_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.hr_png)
    }

    async fn trimmed_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.trimmed_png)
    }

    async fn distance_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.distance_png)
    }

    async fn pm_png(&self) -> Option<String> {
        Computation::bytes_to_str(&self.pm_png)
    }

    async fn correctly_clustered(&self) -> Option<i32> {
        self.correctly_clustered
    }

    async fn incorrectly_clustered(&self) -> Option<i32> {
        self.incorrectly_clustered
    }

    async fn accuracy(&self) -> Option<f64> {
        self.accuracy.and_then(|val| Some(val as f64))
    }

    async fn anomaly(&self) -> Option<i32> {
        self.anomaly
    }

    async fn clusters(&self) -> Option<Vec<i32>> {
        self.clusters
            .as_ref()
            .and_then(|cluster_str| Some(serde_json::from_str::<Vec<i32>>(cluster_str).unwrap()))
    }
}
