use std::io::{self, Write};
use chess::Board;
use chess::Game;
use crate::engine;

pub struct Uci {
}

impl Uci {
    pub fn run() -> Result<(), std::io::Error> {
        // Connect to Chess Arena
        let input = io::stdin();
        let mut output = io::stdout();
        let mut buffer = String::new();

        let game = Game::new();
        assert_eq!(game.current_position(), Board::default());

        let mut engine = engine::Engine::new("startpos");

        loop {
            buffer.clear();
            input.read_line(&mut buffer).expect("Failed to read input");

            let tokens: Vec<&str> = buffer.trim().split(' ').collect();

            match tokens[0] {
                "uci" => {
                    writeln!(output, "id name Crawfish")?;
                    writeln!(output, "id author Ludi-1 and Longjie99hu")?;
                    writeln!(output, "uciok")?;
                }
                "isready" => {
                    writeln!(output, "readyok")?;
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
                    let best_move = engine.calc_move();
                    writeln!(output, "info depth 1")?;
                    writeln!(output, "info multipv 1 depth 1 score cp 1 pv {}", best_move)?;
                    writeln!(output, "bestmove {}", best_move)?;   
                }
                "stop" => {
                    let best_move = engine.calc_move();
                    writeln!(output, "bestmove {}", best_move)?;                
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