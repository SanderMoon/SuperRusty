mod chess_game;
mod chess_game_bitboard;
use chess_game::ChessGame;

fn main() {
    let chess : ChessGame = initialize_chess_board();
    println!("{:#?}", &chess);
}

fn initialize_chess_board() -> ChessGame{
    ChessGame::new()
}