mod uci;
mod engine;
mod api;

fn main() {
    // uci::Uci::run().expect("Error");
    let api = api::Lichess::new("TOKEN");
    api.run().expect("Lichess API run error");
}
