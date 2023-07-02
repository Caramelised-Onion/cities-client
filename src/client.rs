use reqwest;

use cities_common::models::City;
use cities_common::queries::{CitiesQuery, DistQuery};

#[derive(Debug, PartialEq, Clone)]
pub struct Client {
    base_url: String,
}

// let body = reqwest::get(format!("http://localhost:3000/cities?point=POINT(-0.1276%2051.5074)&radius={setting.radius}&sort_by_random=true&minimum_population={setting.min_population}limit=2"))

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
        }
    }
}
impl Client {
    pub fn new(base_url: &str) -> Self {
        Client { base_url: base_url.to_string() }
    }

    pub fn get_full_URI(&self, additional_path: &str) -> String {
        // TODO check for trailing or leading slash
        format!("{}/{}", self.base_url, additional_path)
    }
}

impl Client {
    pub async fn get_random_city(&self) -> Result<City, reqwest::Error> {
        let URI = self.get_full_URI("rand");
        reqwest::get(URI).await.unwrap().json::<City>().await
    }
    pub async fn get_cities(&self, query: &CitiesQuery) -> Result<Vec<City>, reqwest::Error> {
        let query_string = format!(
            "cities?point={}&radius={}&sort_by_random=true&minimum_population={}&limit=2",
            query.point.clone().unwrap(),
            query.radius.unwrap(),
            query.minimum_population.unwrap()
        );
        let URI = self.get_full_URI(&query_string);
        // reqwest::blocking::get(URI)?.json::<Vec<City>>()
        reqwest::get(URI).await.unwrap().json::<Vec<City>>().await
    }
    // http://127.0.0.1:3000/distance?city_id1=1&city_id2=2

    pub async fn get_distance(&self, query: &DistQuery) -> Result<f64, reqwest::Error> {
        let query_string = format!(
            "distance?city_id1={}&city_id2={}",
            query.city_id1, query.city_id2
        );
        let URI = self.get_full_URI(&query_string);
        reqwest::get(URI).await.unwrap().json::<f64>().await
    }
}

// server has to be running for these tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn rand() {
//         let client = Client::default();
//         let res = client.get_random_city();

//         assert!(res.is_ok())

//     }
// }
