use crate::engine;
use chess::Board;
use chess::Game;
use std::io::{self, Write};

pub struct Uci {}

impl Uci {
    pub fn run() -> Result<(), String> {
        // Connect to Chess Arena
        let input = io::stdin();
        let mut output = io::stdout();
        let mut buffer = String::new();

        let game = Game::new();
        assert_eq!(game.current_position(), Board::default());

        let mut engine = engine::Engine::new("startpos");

        loop {
            buffer.clear();
            input.read_line(&mut buffer).map_err(|e| e.to_string())?;

            let tokens: Vec<&str> = buffer.trim().split(' ').collect();

            match tokens[0] {
                "uci" => {
                    writeln!(output, "id name Crawfish").map_err(|e| e.to_string())?;
                    writeln!(output, "id author Ludi-1 and Longjie99hu")
                        .map_err(|e| e.to_string())?;
                    writeln!(output, "uciok").map_err(|e| e.to_string())?;
                }
                "isready" => {
                    writeln!(output, "readyok").map_err(|e| e.to_string())?;
                }
                "position" => {
                    // Update the current position
                    if tokens[1] == "startpos" {
                        engine = engine::Engine::new("startpos");
                        if tokens.len() >= 3 {
                            for uci_move in &tokens[3..] {
                                engine.play_uci_move(uci_move);
                            }
                        }
                    }
                }
                "go" => {
                    // Send the best move for the current position
                    let best_move = engine.calc_move()?.to_string();
                    writeln!(output, "info depth 1").map_err(|e| e.to_string())?;
                    writeln!(output, "info multipv 1 depth 1 score cp 1 pv {best_move}")
                        .map_err(|e| e.to_string())?;
                    writeln!(output, "bestmove {best_move}").map_err(|e| e.to_string())?;
                }
                "stop" => {
                    let best_move = engine.calc_move()?.to_string();
                    writeln!(output, "bestmove {best_move}").map_err(|e| e.to_string())?;
                }
                "quit" => {
                    // Quit the program
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
