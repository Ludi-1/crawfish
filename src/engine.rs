use chess::{Board, ChessMove, MoveGen, Piece, Square};
use std::str::FromStr;

pub struct Engine {
    board: chess::Board,
}

impl Engine {
    pub fn new(start_pos: &str) -> Self {
        let board: chess::Board = if start_pos == "startpos" {
            Board::default()
        } else {
            Board::from_str(start_pos).expect("start_pos invalid")
        };
        Self { board }
    }

    pub fn calc_move(&self) -> String {
        // Implement your chess engine algorithm here
        // Return the best move as a string in UCI format
        // For example: "e2e4"
        let mut movegen = MoveGen::new_legal(&self.board);
        movegen.next().expect("Error movegen").to_string()
    }

    pub fn play_uci_move(&mut self, uci_move: &str) {
        assert!(4 <= uci_move.len() && uci_move.len() <= 5);
        let src_str = &uci_move[..2];
        let dest_str = &uci_move[2..];
        let mut promote = None;
        if uci_move.len() == 5 {
            let promote_char = uci_move.chars().nth(4);
            match promote_char {
                Some('q') => promote = Some(Piece::Queen),
                Some('n') => promote = Some(Piece::Knight),
                Some('r') => promote = Some(Piece::Rook),
                Some('b') => promote = Some(Piece::Bishop),
                _ => println!("play_uci_move: Uninplemented piece"),
            }
        }
        let src_square = Square::from_str(src_str).expect("Valid src square");
        let dest_square = Square::from_str(dest_str).expect("Valid dest square");
        let mov = ChessMove::new(src_square, dest_square, promote);
        let mut new_board: chess::Board = Default::default();
        self.board.make_move(mov, &mut new_board);
        self.board = new_board;
    }
}
