mod chess_game;
mod chess_game_bitboard;
mod board_utils;
mod pawn;
mod knight;
mod king;
use chess_game_bitboard::ChessBoard;

fn main() {
    let chess_board : ChessBoard = ChessBoard::new();
    println!("{}", chess_board.visualize());
}