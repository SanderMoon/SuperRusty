#[derive(PartialEq, Debug)]
pub(crate) enum Color{
    Black,
    White
}

#[derive(PartialEq, Debug)]
pub(crate) enum PieceNames{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq, Debug)]
pub(crate) struct PieceType {
    // Bitboard representing the positions of all pieces of this type on the board
    pub positions: u64,
    // Bitboard representing the possible moves for a piece of this type at a given position
    pub moves: u64,
    pub attacks: u64,
    pub color: Color
}

pub(crate) struct Move {
    pub piece_type:PieceNames,
    pub old_position: u64,
    pub new_position: u64
}

pub(crate) struct ChessBitboard {
    pub white_pawns: PieceType,
    pub white_knights: PieceType,
    pub white_bishops: PieceType,
    pub white_rooks: PieceType,
    pub white_queen: PieceType,
    pub white_king: PieceType,
    pub black_pawns: PieceType,
    pub black_knights: PieceType,
    pub black_bishops: PieceType,
    pub black_rooks: PieceType,
    pub black_queen: PieceType,
    pub black_king: PieceType,
}

/// Basic implementation of a bitboard.
impl ChessBitboard {
    pub fn new() -> ChessBitboard {
        // Initialize a new chessboard with the standard starting positions
        ChessBitboard {
            white_pawns: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
                moves: 0,
                attacks: 0,
                color: Color::White
            },
            white_rooks: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                moves: 0,
                attacks: 0,
                color: Color::White
            },
            white_knights: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                moves: 0,
                attacks: 0,
                color: Color::White
            },
            white_bishops: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                moves: 0,
                attacks: 0,
                color: Color::White
            },
            white_queen: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
                moves: 0,
                attacks: 0,
                color: Color::White
            },
            white_king: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                moves: 0,
                attacks: 0,
                color: Color::White
            },
            black_pawns: PieceType {
                positions: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0,
                attacks: 0,
                color: Color::Black
            },
            black_rooks: PieceType {
                positions: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0,
                attacks: 0,
                color: Color::Black
            },
            black_knights: PieceType {
                positions: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0,
                attacks: 0,
                color: Color::Black
            },
            black_bishops: PieceType {
                positions: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0,
                attacks: 0,
                color: Color::Black
            },
            black_queen: PieceType {
                positions: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0,
                attacks: 0,
                color: Color::Black
            },
            black_king: PieceType {
                positions: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0,
                attacks: 0,
                color: Color::Black
            }
        }
    }

    /// Visualizes the bitboard by returning a Unicode represention
    pub(crate) fn visualize(&self) -> String {
        let mut result = String::new();
        for i in 0..8 {
            for j in 0..8 {
                if j == 0 {
                    result += format!("{} ", 8 - 1 - i + 1).as_str();
                }
                result += format!("{} ", get_square_character(self, 7 - j, 7 - i)).as_str();
            }
            result += "\n";
        }
        
        result += "  ";
        for i in 0..8 {
            result += format!("{} ", char::from_u32('A' as u32 + i).unwrap()).as_str();
        }

        result
    }
}

/// Takes a bitboard object and a position and returns the Unicode representation 
/// of that square
fn get_square_character(bitboard: &ChessBitboard, x: i32, y: i32) -> &'static str {
    // TODO: Improve this by storing piece information in a array, or dictionary with
    // the pawn piece enum as the key value
    if get_square(bitboard.white_pawns.positions, x, y) {
       return "♙";
    }

    if get_square(bitboard.black_pawns.positions, x, y) {
        return "♟︎";
    }
     
    if get_square(bitboard.white_knights.positions, x, y) {
        return "♘";
    }

    if get_square(bitboard.black_knights.positions, x, y) {
        return "♞";
    }

    if get_square(bitboard.white_bishops.positions, x, y) {
        return "♗";
    }

    if get_square(bitboard.black_bishops.positions, x, y) {
        return "♝";
    }

    if get_square(bitboard.white_rooks.positions, x, y) {
        return "♖";
    }

    if get_square(bitboard.black_rooks.positions, x, y) {
        return "♜";
    }

    if get_square(bitboard.white_queen.positions, x, y) {
        return "♕";
    }

    if get_square(bitboard.black_queen.positions, x, y) {
        return "♛";
    }

    if get_square(bitboard.white_king.positions, x, y) {
        return "♔";
    }

    if get_square(bitboard.black_king.positions, x, y) {
        return "♚";
    }

    if (x + y) % 2 == 0 {
        return "□";
    }
    
    "■" 
}

/// Takes a bitboard bitstring and a positioon to check out if it is marked
fn get_square(positions: u64, x: i32, y: i32) -> bool {
    return (positions >> (y * 8 + x)) & 1 == 1
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_empty_squares(chessboard : &ChessBitboard) -> u64 {
    !(get_black_pieces(chessboard) | get_white_pieces(chessboard))
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_white_pieces(chessboard : &ChessBitboard) -> u64 {
    chessboard.white_bishops.positions | chessboard.white_king.positions | chessboard.white_knights.positions | chessboard.white_pawns.positions | chessboard.white_queen.positions | chessboard.white_rooks.positions
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_black_pieces(chessboard : &ChessBitboard) -> u64 {
    chessboard.black_bishops.positions | chessboard.black_king.positions | chessboard.black_knights.positions | chessboard.black_pawns.positions | chessboard.black_queen.positions | chessboard.black_rooks.positions
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let chessboard = ChessBitboard::new();
        let result = chessboard.white_bishops.positions | chessboard.white_king.positions | chessboard.white_knights.positions | chessboard.white_pawns.positions | chessboard.white_queen.positions | chessboard.white_rooks.positions 
            | chessboard.black_bishops.positions | chessboard.black_king.positions | chessboard.black_knights.positions | chessboard.black_pawns.positions | chessboard.black_queen.positions | chessboard.black_rooks.positions;       
        let expected = 0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_get_empty_squares() {
        let chessboard = ChessBitboard::new();
        let result = get_empty_squares(&chessboard);
        let expected = 0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_white_pieces(){
        let chessboard = ChessBitboard::new();
        let result = get_white_pieces(&chessboard);
        let expected = 0b0000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_black_pieces(){
        let chessboard = ChessBitboard::new();
        let result = get_black_pieces(&chessboard);
        let expected = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_visualize(){
        let chessboard = ChessBitboard::new();
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