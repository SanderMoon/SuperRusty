mod chess;
mod movesets;
mod utils;
use chess::chess_board::ChessBoard;
use chess::visualization::Visualize;

fn main() {
    let chess_board : ChessBoard = ChessBoard::new(false);
    println!("{}", chess_board.visualize());
}