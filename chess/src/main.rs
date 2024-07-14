use my_chess::{DisplayableBoard, prompt_for_move};
use chess::{Board, BoardStatus};
use termion::{clear, cursor};


fn main() {
    let mut dis_board = DisplayableBoard::default();

    loop {
        println!("{}{}", clear::All, cursor::Goto(1, 1)); 
        println!("{}", dis_board);

        
        match dis_board.0.status() {
            BoardStatus::Ongoing => {
                let mut chess_move = None;
                while chess_move.is_none() {
                    chess_move = prompt_for_move(&dis_board.0);
                    if chess_move == None {
                        println!("Please enter a valid move.");
                    }
                }
                
                dis_board.0 = dis_board.0.make_move_new(chess_move.unwrap());
            }
            BoardStatus::Stalemate => {
                println!("Stalemate!");
                break;
            }
            BoardStatus::Checkmate => {
                println!("Checkmate!");
                break;
            }
        }
    }
}