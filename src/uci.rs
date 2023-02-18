use std::io::{self, Write};
use chess::MoveGen;
use chess::Board;
use chess::EMPTY;
use chess::Game;
use std::fs;

fn main() {
    // Connect to Chess Arena
    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut buffer = String::new();
    let position = String::new();

    // Send UCI identification information
    writeln!(output, "id name Your AI name");
    writeln!(output, "id author Your name");
    writeln!(output, "uciok");

    let game = Game::new();
    assert_eq!(game.current_position(), Board::default());

    let board = Board::default();

    loop {
        buffer.clear();
        input.read_line(&mut buffer).expect("Failed to read input");

        let tokens: Vec<&str> = buffer.trim().split(' ').collect();
        fs::write("~/Documents/GitHub/crawfish/debug.txt", tokens.join(" ")).expect("Unable to write file");
        match tokens[0] {
            "uci" => {
                // Send UCI identification information
                writeln!(output, "id name Crawfish");
                writeln!(output, "id author Your name");
                writeln!(output, "uciok");
            }
            "isready" => {
                // Respond to Chess Arena's isready command
                writeln!(output, "readyok");
            }
            "position" => {
                // Update the current position
                let position = tokens[2..].join(" ");
                println!("{position}");
            }
            "go" => {
                // Send the best move for the current position
                //let best_move = calculate_best_move(&position);
                //writeln!(output, "bestmove {}", best_move);
            }
            "quit" => {
                // Quit the program
                break;
            }
            _ => {}
        }
    }
}

fn calculate_best_move(game: &chess::Game) -> MoveGen {
    // Implement your chess engine algorithm here
    // Return the best move as a string in UCI format
    // For example: "e2e4"
    MoveGen::new_legal(&game.current_position())
}
