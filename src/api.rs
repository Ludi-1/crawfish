use reqwest::{Client};
use tokio_util::io::StreamReader;
use std::io::{Error, ErrorKind};
use futures_util::stream::{TryStreamExt};
use tokio::io::{AsyncReadExt};

pub struct Lichess {
    client: Client,
    url_base: String,
    headers: reqwest::header::HeaderMap,
}

impl Lichess {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let url_base = String::from("https://lichess.org");
        let token = String::from("TOKEN");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        Self{client, url_base, headers}
    }

    pub async fn event_stream(&self) {
        let url = format!("{}/api/stream/event", self.url_base);
        // // // let response = self.client.get(&url).send().await.map_err(error_handler);
        println!("test");
        let response = self.client.get(&url).headers(self.headers.clone()).send().await.unwrap();
        println!("response");
        let body = response.text().await;
        println!("body response");
        println!("{:?}", body);
        // let stream = response.bytes_stream().map_err(|err| Error::new(ErrorKind::Other, err));
        // let mut reader = StreamReader::new(stream);
        // let mut buf = [0; 5];
        // let val = reader.read_exact(&mut buf).await.unwrap();
        // println!("{val}");
        // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    }
}