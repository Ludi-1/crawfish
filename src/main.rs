mod api;
use serde::{Deserialize, Serialize, de::value};
use serde_json::Value;

// Define a struct to hold the response from the API
#[derive(Debug, Deserialize, Serialize)]
struct LichessUser {
    id: String,
    username: String,
    online: Option<bool>,
    perfs: Option<Perfs>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Perfs {
    blitz: Option<Perf>,
    bullet: Option<Perf>,
    rapid: Option<Perf>,
    classical: Option<Perf>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Perf {
    games: u32,
    rating: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let api = api::Lichess::new();
    api.event_stream().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
    // // Set your Lichess API token
    // let token = "lip_SQvpXw5Sq6dTaKWqdjig";
    // // Create the request headers with the API token
    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?);
    // // Join a 1+0 bullet game
    // let game_url = "https://lichess.org/api/challenge/Ludiminium";
    // let body = r#"{
    //     "variant": "standard",
    //     "clock": {
    //         "limit": 60,
    //         "increment": 0
    //     },
    //     "rated": false
    // }"#;
    // let client = reqwest::Client::new();
    // let res = client.post(game_url)
    //     .headers(headers.clone())
    //     .body(body)
    //     .send()
    //     .await?;
    // let response = res.text().await?;
    // let data: Value = serde_json::from_str(&response.to_string())?;
    // println!("{response}");

    // let game_id = &data["challenge"]["id"].as_str().unwrap();
    // println!("ID {}", game_id);

    // // Poll the game events endpoint periodically to fetch the moves played
    // let game_events_url = format!("https://lichess.org/api/stream/event");

    // loop {
    //     let res = client.get(&game_events_url)
    //         .headers(headers.clone())
    //         .header("User-Agent", "Reqwest Rust Test")
    //         .send()
    //         .await?;
    //     println!("wait respone");
    //     let body = res.text().await?;
    //     println!("wait response received");
    //     println!("{body}");
    //     // for line in body.lines() {
    //     //     if let Ok(event) = serde_json::from_str::<Value>(line) {
    //     //         if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
    //     //             if event_type == "gameFull" {
    //     //                 // Extract the initial game state and print it
    //     //                 let game_state = event.get("state").ok_or("No game state found in gameFull event")?;
    //     //                 let moves = game_state.get("moves").ok_or("No moves found in game state")?;
    //     //                 println!("Initial state:\n{}", moves.as_str().unwrap_or(""));
    //     //             } else if event_type == "gameState" {
    //     //                 // Extract the current game state and print the moves played since the last event
    //     //                 let game_state = event.get("moves").ok_or("No moves found in gameState event")?;
    //     //                 println!("Moves played:\n{}", game_state.as_str().unwrap_or(""));
    //     //             }
    //     //         }
    //     //     }
    //     // }
    //     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    // }
}
