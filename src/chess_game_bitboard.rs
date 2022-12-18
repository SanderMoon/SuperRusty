use std::u64;

struct PieceType {
    // Bitboard representing the positions of all pieces of this type on the board
    positions: u64,
    // Bitboard representing the possible moves for a piece of this type at a given position
    moves: [u64; 64],
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
                positions: u64::from(0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000),
                moves: [0; 64],
            },
            white_rooks: PieceType {
                positions: u64::from(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001),
                moves: [0; 64],
            },
            white_knights: PieceType {
                positions: u64::from(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010),
                moves: [0; 64],
            },
            white_bishops: PieceType {
                positions: u64::from(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100),
                moves: [0; 64],
            },
            white_queen: PieceType {
                positions: u64::from(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000),
                moves: [0; 64],
            },
            white_king: PieceType {
                positions: u64::from(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000),
                moves: [0; 64],
            },
            black_pawns: PieceType {
                positions: u64::from(0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000),
                moves: [0; 64],
            },
            black_rooks: PieceType {
                positions: u64::from(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000)
            },
            black_knights: PieceType {
                positions: u64::from(0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
                moves: [0; 64],
            },
            black_bishops: PieceType {
                positions: u64::from(0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
                moves: [0; 64],
            },
            black_queen: PieceType {
                positions: u64::from(0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
                moves: [0; 64],
            },
            black_king: PieceType {
                positions: u64::from(0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
                moves: [0; 64],
            },
        }
    }
}
