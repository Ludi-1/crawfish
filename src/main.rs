use std::{env, fs};
mod api;
mod engine;
mod uci;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        uci::Uci::run()?;
        Ok(())
    } else if args.len() == 2 {
        match args[1].as_str() {
            "uci" => {
                // UCI <-> GUI
                uci::Uci::run()?;
                Ok(())
            }
            "lichess" => {
                // Lichess API
                let token =
                    fs::read_to_string("tokens/lichess_api.txt").map_err(|e| e.to_string())?;
                let api = api::Lichess::new(token)?;
                api.run().expect("Lichess API run error");
                Ok(())
            }
            _ => return Err(format!("Unknown argument {}", args[1])),
        }
    } else {
        Err("Too many arguments".to_string())
    }
}
