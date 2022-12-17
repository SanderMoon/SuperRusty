#[derive(Debug, Copy, Clone)]enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug, Copy, Clone)]
enum Color{
    Black,
    White
}
#[derive(Debug, Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color,
    value: i16,
    position: (u8, u8)
}

type ChessBoard = [[Option<Piece>; 8]; 8];


#[derive(Debug, Copy, Clone)]
pub(crate) struct ChessGame {
    board: ChessBoard
}

impl ChessGame{
    #[doc = "this function return a new ChessGame with a standard chess board configuration"]
    pub fn new() -> ChessGame {
        let mut board: ChessBoard =  [[None;8]; 8];

        // Initialize white pawns
        for i in 0..8 {
            board[1][i] = Some(Piece{piece_type: PieceType::Pawn,color: Color::White, value: 1, position: (1,i as u8)});
        }

        // Initialize white pieces
        board[0][0] = Some(Piece{piece_type: PieceType::Rook,color: Color::White, value: 5, position: (0,0 as u8)});
        board[0][1] = Some(Piece{piece_type: PieceType::Knight,color: Color::White, value: 3, position: (0,1 as u8)});
        board[0][2] = Some(Piece{piece_type: PieceType::Bishop,color: Color::White, value: 3, position: (0,2 as u8)});
        board[0][3] = Some(Piece{piece_type: PieceType::Queen,color: Color::White, value: 9, position: (0,3 as u8)});
        board[0][4] = Some(Piece{piece_type: PieceType::King,color: Color::White, value: 200, position: (0,4 as u8)});
        board[0][5] = Some(Piece{piece_type: PieceType::Bishop,color: Color::White, value: 3, position: (0,5 as u8)});
        board[0][6] = Some(Piece{piece_type: PieceType::Knight,color: Color::White, value: 3, position: (0,6 as u8)});
        board[0][7] = Some(Piece{piece_type: PieceType::Rook,color: Color::White, value: 5, position: (0,7 as u8)});

        //Initialize black pawns
        for i in 0..8 {
            board[6][i] = Some(Piece{piece_type: PieceType::Pawn,color: Color::Black, value: 1, position: (6,i as u8)});
        }

        //Initialize black pieces
        board[7][0] = Some(Piece{piece_type: PieceType::Rook,color: Color::Black, value: 5, position: (7,0 as u8)});
        board[7][1] = Some(Piece{piece_type: PieceType::Knight,color: Color::Black, value: 3, position: (7,1 as u8)});
        board[7][2] = Some(Piece{piece_type: PieceType::Bishop,color: Color::Black, value: 3, position: (7,2 as u8)});
        board[7][3] = Some(Piece{piece_type: PieceType::Queen,color: Color::Black, value: 9, position: (7,3 as u8)});
        board[7][4] = Some(Piece{piece_type: PieceType::King,color: Color::Black, value: 200, position: (7,4 as u8)});
        board[7][5] = Some(Piece{piece_type: PieceType::Bishop,color: Color::Black, value: 3, position: (7,5 as u8)});
        board[7][6] = Some(Piece{piece_type: PieceType::Knight,color: Color::Black, value: 3, position: (7,6 as u8)});
        board[7][7] = Some(Piece{piece_type: PieceType::Rook,color: Color::Black, value: 5, position: (7,7 as u8)});
        
        ChessGame{board: board}

    }
}