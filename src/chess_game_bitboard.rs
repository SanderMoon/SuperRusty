
struct PieceType {
    // Bitboard representing the positions of all pieces of this type on the board
    positions: u64,
    // Bitboard representing the possible moves for a piece of this type at a given position
    moves: u64,
}

struct ChessBoard {
    white_pawns: PieceType,
    white_knights: PieceType,
    white_bishops: PieceType,
    white_rooks: PieceType,
    white_queen: PieceType,
    white_king: PieceType,
    black_pawns: PieceType,
    black_knights: PieceType,
    black_bishops: PieceType,
    black_rooks: PieceType,
    black_queen: PieceType,
    black_king: PieceType,
}

/// Basic implementation of a bitboard.
impl ChessBoard {
    fn new() -> ChessBoard {
        // Initialize a new chessboard with the standard starting positions
        ChessBoard {
            white_pawns: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
                moves: 0
            },
            white_rooks: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                moves: 0
            },
            white_knights: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                moves: 0
            },
            white_bishops: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                moves: 0
            },
            white_queen: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
                moves: 0
            },
            white_king: PieceType {
                positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                moves: 0
            },
            black_pawns: PieceType {
                positions: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0
            },
            black_rooks: PieceType {
                positions: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0
            },
            black_knights: PieceType {
                positions: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0
            },
            black_bishops: PieceType {
                positions: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0
            },
            black_queen: PieceType {
                positions: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0
            },
            black_king: PieceType {
                positions: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                moves: 0
            },
        }
    }

}


/// Takes a chess board and returns a bit board containing 1's on all places where there is an empty square. 
fn get_empty_squares(chessboard : ChessBoard) -> u64 {
    !(chessboard.white_bishops.positions | chessboard.white_king.positions | chessboard.white_knights.positions | chessboard.white_pawns.positions | chessboard.white_queen.positions | chessboard.white_rooks.positions 
    | chessboard.black_bishops.positions | chessboard.black_king.positions | chessboard.black_knights.positions | chessboard.black_pawns.positions | chessboard.black_queen.positions | chessboard.black_rooks.positions)
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_empty_squares() {
        let chessboard = ChessBoard::new();
        let result = get_empty_squares(chessboard);
        let expected = 0b0000000_000000000_11111111_11111111_11111111_11111111_000000000_0000000;
        assert_eq!(result, expected);
    }
}