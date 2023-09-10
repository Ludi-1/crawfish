use std::{env, fs};
mod api;
mod engine;
mod uci;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if !args.is_empty() {
        match args[1].as_str() {
            "uci" => {
                // UCI <-> GUI
                uci::Uci::run().expect("Error");
                Ok(())
            }
            "lichess" => {
                // Lichess API
                match fs::read_to_string("tokens/lichess_api.txt") {
                    Ok(token) => {
                        let api = api::Lichess::new(token.as_str());
                        api.run().expect("Lichess API run error");
                        Ok(())
                    }
                    Err(err) => return Err(format!("Could not read lichess token: {err}")),
                }
            }
            _ => return Err(format!("Unknown argument {}", args[1])),
        }
    } else {
        Err("No arguments given".to_string())
    }
}
