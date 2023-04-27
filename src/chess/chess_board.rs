use std::collections::HashMap;
use super::piece::Color;
use super::piece::PieceType;
use super::piece::PieceInfo;
use super::piece::SinglePieceInfo;
use super::chess_move::Move;
use super::castling::Castling;

/*
Macro for creating uniform methods for accessing pieces (with mutable references).
 */
macro_rules! piece_accessors {
    ($(($name:ident, $piece:ident, $color:ident)),* $(,)?) => {
        $(
            pub(crate) fn $name(&mut self) -> &mut PieceInfo {
                self.get_mutable_piece_info(PieceType::$piece, Color::$color)
            }
        )*
    };
}

/*
Macro for creating uniform methods for accessing pieces (with immutable references).
 */
macro_rules! piece_accessors_immutable {
    ($(($name:ident, $piece:ident, $color:ident)),* $(,)?) => {
        $(
            pub(crate) fn $name(&self) -> &PieceInfo {
                self.get_piece_info(PieceType::$piece, Color::$color)
            }
        )*
    };
}

pub(crate) struct ChessBoard {
    pub piece_infos: HashMap<Color, HashMap<PieceType, PieceInfo>>,
    pub active_color: Color,
    pub white_king_side_castle: bool,
    pub white_queen_side_castle: bool,
    pub black_king_side_castle: bool,
    pub black_queen_side_castle: bool,
    pub move_history: Vec<Move>,
}

/// Basic implementation of a bitboard.
impl ChessBoard {

    piece_accessors!(
        (white_pawns, Pawn, White),
        (white_rooks, Rook, White),
        (white_knights, Knight, White),
        (white_bishops, Bishop, White),
        (white_queens, Queen, White),
        (white_kings, King, White),
        (black_pawns, Pawn, Black),
        (black_rooks, Rook, Black),
        (black_knights, Knight, Black),
        (black_bishops, Bishop, Black),
        (black_queens, Queen, Black),
        (black_kings, King, Black),
    );

    piece_accessors_immutable!(
        (white_pawns_immutable, Pawn, White),
        (white_rooks_immutable, Rook, White),
        (white_knights_immutable, Knight, White),
        (white_bishops_immutable, Bishop, White),
        (white_queens_immutable, Queen, White),
        (white_kings_immutable, King, White),
        (black_pawns_immutable, Pawn, Black),
        (black_rooks_immutable, Rook, Black),
        (black_knights_immutable, Knight, Black),
        (black_bishops_immutable, Bishop, Black),
        (black_queens_immutable, Queen, Black),
        (black_kings_immutable, King, Black),
    );

    pub fn new(empty: bool) -> ChessBoard {
        let mut piece_infos = HashMap::new();
        piece_infos.insert(Color::White, initialize_white_pieces(empty));
        piece_infos.insert(Color::Black, initialize_black_pieces(empty));
        // Initialize a new chessboard with the standard starting positions
        ChessBoard {
            piece_infos,
            white_king_side_castle: true,
            white_queen_side_castle: true,
            black_king_side_castle: true,
            black_queen_side_castle: true,
            active_color: Color::White,
            move_history: Vec::new(),
        }
    }

    pub(crate) fn set_square(&mut self, piece_info: SinglePieceInfo) {
        let position: u64 = 1 << (7 - piece_info.position_x) + (piece_info.position_y * 8);
        let mutable = self.piece_infos
            .get_mut(&piece_info.color).unwrap()
            .get_mut(&piece_info.piece_type).unwrap();
            mutable.positions |= position;
    }

    pub(crate) fn set_active_color(&mut self, color: Color) {
        self.active_color = color;
    }
    
    pub(crate) fn set_castling(&mut self, color: Color, castling: Castling) {
        match castling {
            Castling::KingSide => {
                if color == Color::White {
                    self.white_king_side_castle = true;
                } else {
                    self.black_king_side_castle = true;
                }
            },
            Castling::QueenSide => {
                if color == Color::White {
                    self.white_queen_side_castle = true;
                } else {
                    self.black_queen_side_castle = true;
                }
            }
        }
    
    }

    pub fn get_mutable_piece_info(&mut self, piece_type: PieceType, color: Color) -> &mut PieceInfo {
        self.piece_infos
            .get_mut(&color)
            .expect("Color not found in piece_infos")
            .get_mut(&piece_type)
            .expect("PieceType not found in piece_infos for the given color")
    }

    pub fn get_piece_info(&self, piece_type: PieceType, color: Color) -> &PieceInfo {
        self.piece_infos
            .get(&color)
            .expect("Color not found in piece_infos")
            .get(&piece_type)
            .expect("PieceType not found in piece_infos for the given color")
    }
    
}


fn initialize_white_pieces(empty: bool) -> HashMap<PieceType, PieceInfo> {
    let mut white_piece_infos = HashMap::new();
    white_piece_infos.insert(PieceType::Pawn, PieceInfo::new_pawn(empty, Color::White));
    white_piece_infos.insert(PieceType::Rook, PieceInfo::new_rook(empty, Color::White));
    white_piece_infos.insert(PieceType::Knight, PieceInfo::new_knight(empty, Color::White));
    white_piece_infos.insert(PieceType::Bishop, PieceInfo::new_bishop(empty, Color::White));
    white_piece_infos.insert(PieceType::Queen, PieceInfo::new_queen(empty, Color::White));
    white_piece_infos.insert(PieceType::King, PieceInfo::new_king(empty, Color::White));
    white_piece_infos
}

fn initialize_black_pieces(empty: bool) -> HashMap<PieceType, PieceInfo> {
    let mut black_piece_infos = HashMap::new();
    black_piece_infos.insert(PieceType::Pawn, PieceInfo::new_pawn(empty, Color::Black));
    black_piece_infos.insert(PieceType::Rook, PieceInfo::new_rook(empty, Color::Black));
    black_piece_infos.insert(PieceType::Knight, PieceInfo::new_knight(empty, Color::Black));
    black_piece_infos.insert(PieceType::Bishop, PieceInfo::new_bishop(empty, Color::Black));
    black_piece_infos.insert(PieceType::Queen, PieceInfo::new_queen(empty, Color::Black));
    black_piece_infos.insert(PieceType::King, PieceInfo::new_king(empty, Color::Black));
    black_piece_infos
}


/// Takes a bitboard bitstring and a position to check out if it is marked
pub(crate) fn get_square(positions: u64, x: i32, y: i32) -> bool {
    (positions >> (y * 8 + (7 - x))) & 1 == 1
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_empty_squares(chessboard : &mut ChessBoard) -> u64 {
    !(get_black_pieces(chessboard) | get_white_pieces(chessboard))
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_white_pieces(chessboard : &mut ChessBoard) -> u64 {
    chessboard.white_pawns().positions |
    chessboard.white_rooks().positions |
    chessboard.white_knights().positions |
    chessboard.white_bishops().positions |
    chessboard.white_queens().positions |
    chessboard.white_kings().positions
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_black_pieces(chessboard : &mut ChessBoard) -> u64 {
    chessboard.black_pawns().positions |
    chessboard.black_rooks().positions |
    chessboard.black_knights().positions |
    chessboard.black_bishops().positions |
    chessboard.black_queens().positions |
    chessboard.black_kings().positions
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut chessboard = ChessBoard::new(false);
        let result = chessboard.white_pawns().positions |
            chessboard.white_rooks().positions |
            chessboard.white_knights().positions |
            chessboard.white_bishops().positions |
            chessboard.white_queens().positions |
            chessboard.white_kings().positions |
            chessboard.black_pawns().positions |
            chessboard.black_rooks().positions |
            chessboard.black_knights().positions |
            chessboard.black_bishops().positions |
            chessboard.black_queens().positions |
            chessboard.black_kings().positions;

        let expected = 0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_get_empty_squares() {
        let mut chessboard = ChessBoard::new(false);
        let result = get_empty_squares(&mut chessboard);
        let expected = 0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_white_pieces(){
        let mut chessboard = ChessBoard::new(false);
        let result = get_white_pieces(&mut chessboard);
        let expected = 0b0000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_black_pieces(){
        let mut chessboard = ChessBoard::new(false);
        let result = get_black_pieces(&mut chessboard);
        let expected = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_set_square(){
        let mut chessboard = ChessBoard::new(false);
        chessboard.set_square(SinglePieceInfo { 
            piece_type: PieceType::Pawn, 
            color: Color::White, 
            position_x: 1, 
            position_y: 4
        });

        assert_eq!(chessboard.white_pawns().positions, 0b00000000_00000000_00000000_01000000_00000000_00000000_11111111_00000000);
    }
}