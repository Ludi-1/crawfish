mod uci;
mod engine;
mod api;

fn main() {

    // UCI <-> GUI
    uci::Uci::run().expect("Error");

    // Lichess API
    let api = api::Lichess::new("TOKEN");
    api.run().expect("Lichess API run error");
}
