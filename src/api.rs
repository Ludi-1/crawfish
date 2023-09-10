use crate::engine;
use futures_util::stream::TryStreamExt;
use reqwest::Client;
use serde_json::Value;
use std::io::{Error, ErrorKind};
use tokio::io::AsyncBufReadExt;
use tokio::runtime::Runtime;
use tokio_util::io::StreamReader;

pub struct Lichess {
    client: Client,
    url_base: String,
    headers: reqwest::header::HeaderMap,
    token: String,
}

impl Lichess {
    pub fn new(token: &str) -> Self {
        let client = reqwest::Client::new();
        let url_base = String::from("https://lichess.org");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
        );
        Self {
            token: token.to_string(),
            client,
            url_base,
            headers,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rt = Runtime::new()?;
        rt.block_on(self.event_stream());
        Ok(())
    }

    // Stream the events reaching a lichess user in real time as ndjson.
    pub async fn event_stream(&self) {
        let url = format!("{}/api/stream/event", self.url_base);
        let response = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Response error");

        let stream = response
            .bytes_stream()
            .map_err(|err| Error::new(ErrorKind::Other, err));

        let mut lines = StreamReader::new(stream).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let json_stream =
                serde_json::Deserializer::from_str(line.as_str()).into_iter::<Value>();
            for event in json_stream {
                let lichess_token = self.token.clone();
                tokio::spawn(async move {
                    let lichess_obj = Lichess::new(&lichess_token);
                    lichess_obj
                        .handle_event_stream(Ok(event.expect("Bad event_stream")))
                        .await;
                });
            }
        }
    }

    pub async fn handle_event_stream(&self, event: Result<Value, serde_json::Error>) {
        let req_type = event.as_ref().unwrap()["type"].as_str();
        if req_type == Some("challenge") {
            // A player sends you a challenge or you challenge someone
            println!("challenge");
            let challenge_id = event.as_ref().unwrap()["challenge"]["id"].as_str().unwrap();
            self.challenge_accept(challenge_id)
                .await
                .expect("Challenge error");
        } else if req_type == Some("gameStart") {
            // Start of a game
            println!("gameStart");
            let game_id = event.as_ref().unwrap()["game"]["gameId"].as_str().unwrap();
            self.stream_game(game_id).await;
        } else {
            // challengeCanceled or challengeDeclined
            println!("{req_type:?}");
        }
    }

    // Accept an incoming challenge.
    pub async fn challenge_accept(&self, challenge_id: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/challenge/{}/accept", self.url_base, challenge_id);
        let response = self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Challenge accept error");
        response.text().await
    }

    // Stream positions and moves of any ongoing game, in ndjson.
    pub async fn stream_game(&self, game_id: &str) {
        let url = format!("{}/api/bot/game/stream/{game_id}", self.url_base);
        let response = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Response error");

        let stream = response
            .bytes_stream()
            .map_err(|err| Error::new(ErrorKind::Other, err));

        let mut lines = StreamReader::new(stream).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let json_stream =
                serde_json::Deserializer::from_str(line.as_str()).into_iter::<Value>();
            for event in json_stream {
                // print!("{:?}", event);
                self.handle_game_stream(Ok(event.expect("Stream_game error")), game_id)
                    .await;
            }
        }
    }

    pub async fn handle_game_stream(&self, event: Result<Value, serde_json::Error>, game_id: &str) {
        let game_type = event.as_ref().unwrap()["type"].as_str().unwrap();
        // println!("gameState: {:?}", game_type);
        if game_type == "gameFull" {
            // TODO: Later support other gamemodes
            // let fen = event.as_ref().unwrap()["initialFen"].as_str().unwrap();
            // engine = engine::Engine::new(fen);
            let status = event.as_ref().unwrap()["state"]["status"].as_str().unwrap();
            if status == "started" {
                let played_moves = event.as_ref().unwrap()["state"]["moves"].as_str().unwrap();
                self.calculate_engine(game_id, played_moves).await;
            }
        } else if game_type == "gameState" {
            let status = event.as_ref().unwrap()["status"].as_str().unwrap();
            if status == "started" {
                let played_moves = event.as_ref().unwrap()["moves"].as_str().unwrap();
                self.calculate_engine(game_id, played_moves).await;
            }
        } else {
            println!("notimplemented");
        }
    }

    pub async fn calculate_engine(&self, game_id: &str, played_moves: &str) {
        let mut engine = engine::Engine::new("startpos");
        if !played_moves.is_empty() {
            let move_list = played_moves.split_whitespace();
            for moves in move_list {
                engine.play_uci_move(moves);
            }
        }
        let uci_move = engine.calc_move();
        self.make_move(game_id, uci_move)
            .await
            .expect("Make move error");
    }

    pub async fn make_move(
        &self,
        game_id: &str,
        uci_move: String,
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/bot/game/{game_id}/move/{uci_move}", self.url_base);
        // println!("make_move url: {url}");
        let response = self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Response error");
        response.text().await
    }
}
