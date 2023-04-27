use crate::movesets::king::calculate_individual_king_moves;
use std::ops::Not;

use super::chess_move::Move;
#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub  enum Color{
    Black,
    White
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq, Debug)]
pub(crate) struct PieceInfo {
    // Bitboard representing the positions of all pieces of this type on the board
    pub positions: u64,
    // Bitboard representing the possible moves for a piece of this type at a given position
    pub moves: u64,
    pub attacks: u64,
    pub color: Color,
    pub piece_type: PieceType,
}

fn create_piece_info(positions: u64, color: Color, piece_type : PieceType) -> PieceInfo {
    PieceInfo {
        positions,
        moves: 0,
        attacks: 0,
        color,
        piece_type

    }
}

impl PieceInfo {

    pub fn calculate_individual_moves(&mut self, empty_squares: u64, opponent_pieces: u64) -> Vec<Move> {
        match self.piece_type {
            //PieceType::Knight => calculate_individual_knight_moves(self, empty_squares, opponent_pieces),
            //PieceType::Pawn => calculate_individual_pawn_moves(self, empty_squares, opponent_pieces),
            PieceType::King => calculate_individual_king_moves(self, empty_squares, opponent_pieces),
            //PieceType::Bishop => calculate_individual_bishop_moves(self, empty_squares, opponent_pieces),
            //PieceType::Rook => calculate_individual_rook_moves(self, empty_squares, opponent_pieces),
            //PieceType::Queen => calculate_individual_queen_moves(self, empty_squares, opponent_pieces),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn new_pawn(empty: bool, color: Color) -> Self {
        let positions = match color {
            Color::White => {
                if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000 }
            }
            Color::Black => {
                if empty { 0 } else { 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000 }
            }
        };
        create_piece_info(positions, color, PieceType::Pawn)
    }
    

    pub(crate) fn new_rook(empty: bool, color: Color) -> Self {
        let positions = match color {
            Color::White => {
                if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001 }
            }
            Color::Black => {
                if empty { 0 } else { 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000 }
            }
        };
        create_piece_info(positions, color, PieceType::Rook)
    }

    pub(crate) fn new_knight(empty: bool, color: Color) -> Self {
        let positions = match color {
            Color::White => {
                if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010 }
            }
            Color::Black => {
                if empty { 0 } else { 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000 }
            }
        };
        create_piece_info(positions, color, PieceType::Knight)
    }

    pub(crate) fn new_bishop(empty: bool, color: Color) -> Self {
        let positions = match color {
            Color::White => {
                if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100 }
            }
            Color::Black => {
                if empty { 0 } else { 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000 }
            }
        };
        create_piece_info(positions, color, PieceType::Bishop)
    }

    pub(crate) fn new_queen(empty: bool, color: Color) -> Self {
        let positions = match color {
            Color::White => {
                if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000 }
            }
            Color::Black => {
                if empty { 0 } else { 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 }
            }
        };
        create_piece_info(positions, color, PieceType::Queen)
    }

    pub(crate) fn new_king(empty: bool, color: Color) -> Self {
        let positions = match color {
            Color::White => {
                if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000 }
            }
            Color::Black => {
                if empty { 0 } else { 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 }
            }
        };
        create_piece_info(positions, color, PieceType::King)
    }
}



#[derive(PartialEq, Debug)]
pub(crate) struct SinglePieceInfo {
    pub piece_type: PieceType,
    pub color: Color,
    pub position_x: usize,
    pub position_y: usize,
}

