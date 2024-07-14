use chess::{BitBoard, Board, BoardBuilder, CastleRights, Color, File, MoveGen, Rank, Square};
use std::fmt;

pub struct DisplayableBoard(pub Board);

impl DisplayableBoard {
    pub fn default() -> Self {
        DisplayableBoard(
            Board::default()
        )
    }
}

impl fmt::Display for DisplayableBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = 2; // Number of padding spaces around the board
        let horizontal_border = " ".repeat(padding) + &" ".repeat(16) + &" ".repeat(padding);
        
        writeln!(f, "{}", horizontal_border)?;
        writeln!(f, "{}", horizontal_border)?;
        for rank in (0..8).rev() {
            write!(f, "{}", " ".repeat(padding))?; // Left padding
            write!(f, "          ")?;

            for file in 0..8 {
                let square = Square::make_square(Rank::from_index(rank), File::from_index(file));
                let piece = self.0.piece_on(square);

                let symbol = match piece {
                    Some(piece) => {
                        let piece_char = piece.to_string(Color::White);
                        match self.0.color_on(square) {
                            Some(Color::White) => piece_char.to_ascii_uppercase(),
                            Some(Color::Black) => piece_char.to_ascii_lowercase(),
                            None => panic!("Oh no ...") 
                        } 
                    }
                    None => ".".to_string(),
                };

                write!(f, "{} ", symbol)?;
            }
            write!(f, "          ")?;
            writeln!(f, "{}", " ".repeat(padding))?; // Right padding
        }
        writeln!(f, "{}", horizontal_border)?;
        write!(f, "{}Side to move: {:?}", " ".repeat(padding), self.0.side_to_move())
    }
}

use inquire::Text;
use chess::ChessMove;

pub fn prompt_for_move(board: &Board) -> Option<ChessMove> {
    let prompt = Text::new("Enter your move:");
    let mut legal_moves = MoveGen::new_legal(board);

    match prompt.prompt() {
        Ok(input) => {
            if input.to_lowercase() == "exit" {
                return None;
            }
            match input.parse::<ChessMove>() {
                Ok(chess_move) => {
                    if legal_moves.any(|x| x == chess_move) {
                        return Some(chess_move)
                    }
                    None
                },
                Err(_) => {
                    println!("Invalid move format.");
                    None
                }
            }
        }
        Err(_) => {
            println!("Error reading input.");
            None
        }
    }
}

