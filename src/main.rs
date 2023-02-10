mod chess_game_bitboard;
mod movesets;
mod utils;
use chess_game_bitboard::ChessBoard;

fn main() {
    let chess_board : ChessBoard = ChessBoard::new();
    println!("{}", chess_board.visualize());
}