use reqwest;

use cities_common::models::{City, Country};
use cities_common::queries::{CitiesQuery, DistQuery};

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://localhost:3000".to_string(),
        }
    }
}
impl Client {
    pub fn new(base_url: &str) -> Self {
        Client {
            base_url: base_url.to_string(),
            ..Default::default()
        }
    }

    // TODO: make the query string only include the actual query string
    pub fn get_full_uri(&self, additional_path: &str) -> String {
        // TODO check for trailing or leading slash
        // or check if is there a nice inbuilt way to deal with this in one of deps
        // might also want to do some sort of url encoding
        format!("{}/{}", self.base_url, additional_path)
    }
}

impl Client {
    pub async fn get_random_city(&self) -> Result<City, reqwest::Error> {
        let uri = self.get_full_uri("rand");
        // reqwest::get(uri).await.unwrap().json::<City>().await
        self.client
            .get(uri)
            .send()
            .await
            .unwrap()
            .json::<City>()
            .await
    }
    pub async fn get_cities(&self, query: &CitiesQuery) -> Result<Vec<City>, reqwest::Error> {
        let uri = self.get_full_uri("cities");
        self.client
            .get(uri)
            .query(query)
            .send()
            .await
            .unwrap()
            .json::<Vec<City>>()
            .await
    }
    
    pub async fn get_distance(&self, query: &DistQuery) -> Result<f64, reqwest::Error> {
        let uri = self.get_full_uri("distance");
        self.client
            .get(uri)
            .query(query)
            .send()
            .await
            .unwrap()
            .json::<f64>()
            .await
    }

    // TODO use a query from models instead
    pub async fn get_country_outline(
        &self,
        country_code: String,
    ) -> Result<String, reqwest::Error> {
        let query_string = format!("countries?country_code={}", country_code);
        let uri = self.get_full_uri(&query_string);
        reqwest::get(uri)
            .await
            .unwrap()
            .json::<Country>()
            .await
            .map(|country| country.geom_wkt)
    }
}
