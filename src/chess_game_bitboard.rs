
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

struct ChessBoard {
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
impl ChessBoard {
    fn new() -> ChessBoard {
        // Initialize a new chessboard with the standard starting positions
        ChessBoard {
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
            },
        }
    }

}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_empty_squares(chessboard : &ChessBoard) -> u64 {
    !(get_black_pieces(chessboard) | get_white_pieces(chessboard))
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_white_pieces(chessboard : &ChessBoard) -> u64 {
    chessboard.white_bishops.positions | chessboard.white_king.positions | chessboard.white_knights.positions | chessboard.white_pawns.positions | chessboard.white_queen.positions | chessboard.white_rooks.positions
}

/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_black_pieces(chessboard : &ChessBoard) -> u64 {
    chessboard.black_bishops.positions | chessboard.black_king.positions | chessboard.black_knights.positions | chessboard.black_pawns.positions | chessboard.black_queen.positions | chessboard.black_rooks.positions
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let chessboard = ChessBoard::new();
        let result = chessboard.white_bishops.positions | chessboard.white_king.positions | chessboard.white_knights.positions | chessboard.white_pawns.positions | chessboard.white_queen.positions | chessboard.white_rooks.positions 
            | chessboard.black_bishops.positions | chessboard.black_king.positions | chessboard.black_knights.positions | chessboard.black_pawns.positions | chessboard.black_queen.positions | chessboard.black_rooks.positions;       
        let expected = 0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_get_empty_squares() {
        let chessboard = ChessBoard::new();
        let result = get_empty_squares(&chessboard);
        let expected = 0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_white_pieces(){
        let chessboard = ChessBoard::new();
        let result = get_white_pieces(&chessboard);
        let expected = 0b0000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_black_pieces(){
        let chessboard = ChessBoard::new();
        let result = get_black_pieces(&chessboard);
        let expected = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(result, expected);
    }
}