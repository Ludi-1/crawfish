mod api;
mod engine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let mut engine_test = engine::Engine::new("startpos");
    // engine_test.play_uci_move("a2a3");

    let api = api::Lichess::new();
    api.event_stream().await;
    loop {
        println!("loop");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
