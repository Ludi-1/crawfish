use crate::engine;
use futures_util::stream::TryStreamExt;
use reqwest::Client;
use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::process::exit;
use tokio::io::AsyncBufReadExt;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tokio_util::io::StreamReader;

pub struct Lichess {
    client: Client,
    url_base: String,
    headers: reqwest::header::HeaderMap,
    token: String,
}

impl Lichess {
    pub fn new(token: String) -> Result<Self, String> {
        let client = reqwest::Client::new();
        let url_base = "https://lichess.org".to_string();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|e| e.to_string())?,
        );
        Ok(Self {
            client,
            url_base,
            headers,
            token,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rt = Runtime::new()?;
        rt.block_on(self.event_stream())?;
        Ok(())
    }

    // Stream the events reaching a lichess user in real time as ndjson.
    pub async fn event_stream(&self) -> Result<(), String> {
        let url = format!("{}/api/stream/event", self.url_base);
        let response = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let stream = response
            .bytes_stream()
            .map_err(|err| Error::new(ErrorKind::Other, err));

        let mut lines = StreamReader::new(stream).lines();
        let mut futures = Vec::new();
        while let Ok(Some(line)) = lines.next_line().await {
            let json_stream =
                serde_json::Deserializer::from_str(line.as_str()).into_iter::<Value>();

            for event in json_stream {
                let event = event.map_err(|e| e.to_string())?;
                let lichess_token = self.token.clone();
                let handle_future: JoinHandle<Result<(), String>> = tokio::spawn(async move {
                    let lichess_obj = Lichess::new(lichess_token)?;
                    lichess_obj.handle_event_stream(event).await?;
                    Ok(())
                });
                futures.push(handle_future);
            }
        }
            // Wait for all spawned futures to complete
        for result in futures {
            result.await.map_err(|e| e.to_string())?? // Unwrap and propagate any errors
        }
        Ok(())
    }

    pub async fn handle_event_stream(&self, event: Value) -> Result<(), String> {
        match event["type"].as_str() {
            Some(req_type) => {
                match req_type {
                    "challenge" => {
                        // A player sends you a challenge or you challenge someone
                        println!("challenge");
                        let challenge_id =
                            event["challenge"]["id"].as_str().ok_or("No challenge id")?;
                        self.challenge_accept(challenge_id)
                            .await
                            .expect("Challenge error");
                        Ok(())
                    }
                    "gameStart" => {
                        // Start of a game
                        println!("gameStart");
                        let game_id = event["game"]["gameId"].as_str().unwrap();
                        self.stream_game(game_id).await?;
                        Ok(())
                    }
                    "gameFinish" => Ok(()),
                    _ => Err(format!("Unknown req_type {req_type}")),
                }
            }
            None => Err("api.handle_event_tream: No event type".to_string()),
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
    pub async fn stream_game(&self, game_id: &str) -> Result<(), String> {
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
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn handle_game_stream(
        &self,
        event: Result<Value, serde_json::Error>,
        game_id: &str,
    ) -> Result<(), String> {
        let game_type = event.as_ref().unwrap()["type"].as_str().unwrap();
        // println!("gameState: {:?}", game_type);
        match game_type {
            "gameFull" => {
                // TODO: Later support other gamemodes
                // let fen = event.as_ref().unwrap()["initialFen"].as_str().unwrap();
                // engine = engine::Engine::new(fen);
                let status = event.as_ref().unwrap()["state"]["status"].as_str().unwrap();
                if status == "started" {
                    let played_moves = event.as_ref().unwrap()["state"]["moves"].as_str().unwrap();
                    self.calculate_engine(game_id, played_moves).await
                } else {
                    Err("gameFull status != started".to_string())
                }
            }
            "gameState" => {
                let status = event.as_ref().unwrap()["status"].as_str().unwrap();
                match status {
                    "started" => {
                        let played_moves = event.as_ref().unwrap()["moves"].as_str().unwrap();
                        self.calculate_engine(game_id, played_moves).await
                    }
                    "resign" => {
                        println!("resign");
                        Ok(())
                    }
                    _ => Err(format!("gameState status {status}")),
                }
            }
            _ => {
                Err(format!("Unknown game_type {game_type}"))
            }
        }
    }

    pub async fn calculate_engine(&self, game_id: &str, played_moves: &str) -> Result<(), String> {
        let mut engine = engine::Engine::new("startpos");
        if !played_moves.is_empty() {
            let move_list = played_moves.split_whitespace();
            for moves in move_list {
                engine.play_uci_move(moves);
            }
        }
        let uci_move = engine.calc_move()?.to_string();
        println!("{}", self.make_move(game_id, uci_move).await?);
        Ok(())
    }

    pub async fn make_move(&self, game_id: &str, uci_move: String) -> Result<String, String> {
        let url = format!("{}/api/bot/game/{game_id}/move/{uci_move}", self.url_base);
        // println!("make_move url: {url}");
        match self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(|e| e.to_string())
        {
            Ok(response) => response.text().await.map_err(|e| e.to_string()),
            Err(err) => Err(err),
        }
    }
}
