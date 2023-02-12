mod chess_game_bitboard;
mod movesets;
mod utils;
use chess_game_bitboard::ChessBoard;

fn main() {
    let mut chess_board : ChessBoard = ChessBoard::new(false);
    println!("{}", chess_board.visualize());
}