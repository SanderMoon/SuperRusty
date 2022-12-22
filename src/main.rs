mod chess_game;
mod chess_game_bitboard;
mod pawn;
use chess_game::ChessGame;

fn main() {
    let chess : ChessGame = initialize_chess_board();
    println!("{:#?}", &chess);
    println!("{}", chess.visualize())
}

fn initialize_chess_board() -> ChessGame{
    ChessGame::new()
}