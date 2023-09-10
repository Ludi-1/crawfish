use std::fs;
mod uci;
mod engine;
mod api;

fn main() -> Result<(), String>{

    // UCI <-> GUI
    // uci::Uci::run().expect("Error");

    // Lichess API
    match fs::read_to_string("tokens/lichess_api.txt"){
        Ok(token) => {
            let api = api::Lichess::new(token.as_str());
            api.run().expect("Lichess API run error");
            Ok(())
        }
        Err(err) => {
            Err(format!("Could not read lichess token: {err}"))
        }
    }
}
