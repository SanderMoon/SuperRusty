/// This class is mostly used for practicing the Rust language. 
/// It is my intention to switch to bitboards ASAP

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
    #[doc = "This function return a new ChessGame with a standard chess board configuration."]
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

    #[doc = "This function returns a string visualizing the chessboard"]
    pub fn visualize(&self) -> String {
        let mut result = String::new();
        for i in 0..8 {
            for j in 0..8 {
                if j == 0 {
                    result += format!("{} ", 8 - 1 - i + 1).as_str();
                }
                result += format!("{} ", visualize_square(self.board, j, 7 - i)).as_str();
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

#[doc = "This function returns a string visualizing the a square on the board"]
fn visualize_square(board: ChessBoard, x: i32, y: i32) -> &'static str {
    match board[y as usize][x as usize] {
        Some(piece) => {
            visualize_filled_squares(piece)
        },
        None => {
            visualize_empty_square(x, y)
        }
    }
}

#[doc = "This function returns a string visualizing the a square on the board that has a piece on it"]
fn visualize_filled_squares(piece: Piece) -> &'static str {
    match piece.piece_type {
        PieceType::Pawn => match piece.color {
            Color::White => "♙",
            Color::Black => "♟︎"
        }
        PieceType::Knight => match piece.color {
            Color::White => "♘",
            Color::Black => "♞"
        }
        PieceType::Bishop => match piece.color {
            Color::White => "♗",
            Color::Black => "♝"
        }
        PieceType::Rook => match piece.color {
            Color::White => "♖",
            Color::Black => "♜"
        }
        PieceType::Queen => match piece.color {
            Color::White => "♕",
            Color::Black => "♛"
        }
        PieceType::King => match piece.color {
            Color::White => "♔",
            Color::Black => "♚"
        }
    }
}

#[doc = "This function returns a string visualizing the an empty square on the board"]
fn visualize_empty_square(x: i32, y: i32) -> &'static str {
    if (x + y) % 2 == 0 {
        "■"
    } else {
        "□"
    }
}