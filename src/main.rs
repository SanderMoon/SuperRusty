mod chess_game;
mod chess_game_bitboard;
mod board_utils;
mod pawn;
mod knight;
mod king;
use chess_game::ChessGame;
use chess_game_bitboard::ChessBitboard;

fn main() {
    let chess_board : ChessBitboard = ChessBitboard::new();
    println!("{}", chess_board.visualize());
}

fn initialize_chess_board() -> ChessGame{
    ChessGame::new()
}