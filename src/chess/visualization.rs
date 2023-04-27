use std::collections::HashMap;

use super::chess_board::ChessBoard;
use super::chess_board::get_square;
use super::piece::{PieceType, Color};

pub trait Visualize {
    fn visualize(&self) -> String;
}

impl Visualize for ChessBoard {

    fn visualize(& self) -> String {
        let mut result = String::new();
        for y in (0..8).rev() {
            for x in 0..8 {
                if x == 0 {
                    result += format!("{} ", y + 1).as_str();
                }
                result += format!("{} ", get_square_character(self, x, y)).as_str();
            }
            result += "\n";
        }
        
        result += "  A B C D E F G H ";
        result
    }
}

fn get_square_character(bitboard: &ChessBoard, x: i32, y: i32) -> &'static str {
    let piece_characters: HashMap<(PieceType, Color), &'static str> = [
        ((PieceType::Pawn, Color::White), "♙"),
        ((PieceType::Pawn, Color::Black), "♟︎"),
        ((PieceType::Rook, Color::White), "♖"),
        ((PieceType::Rook, Color::Black), "♜"),
        ((PieceType::Knight, Color::White), "♘"),
        ((PieceType::Knight, Color::Black), "♞"),
        ((PieceType::Bishop, Color::White), "♗"),
        ((PieceType::Bishop, Color::Black), "♝"),
        ((PieceType::Queen, Color::White), "♕"),
        ((PieceType::Queen, Color::Black), "♛"),
        ((PieceType::King, Color::White), "♔"),
        ((PieceType::King, Color::Black), "♚"),
    ].iter().cloned().collect();

    for ((piece_type, color), character) in piece_characters {
        let positions = bitboard.get_piece_info(piece_type, color).positions;
        if get_square(positions, x, y) {
            return character;
        }
    }

    if (x + y) % 2 == 0 {
        return "■";
    }
    
    "□"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualize(){
        let mut chessboard = ChessBoard::new(false);
        let result = chessboard.visualize().replace(" ", "");
        
        let expected = "
            8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
            7 ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ 
            6 □ ■ □ ■ □ ■ □ ■ 
            5 ■ □ ■ □ ■ □ ■ □ 
            4 □ ■ □ ■ □ ■ □ ■ 
            3 ■ □ ■ □ ■ □ ■ □ 
            2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
            1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
              A B C D E F G H 
        ".trim().replace(" ", "");
        assert_eq!(result, expected);
    }

}