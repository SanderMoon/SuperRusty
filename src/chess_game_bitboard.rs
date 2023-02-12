use std::collections::HashMap;

#[derive(PartialEq, Debug, Eq, Hash)]
pub  enum Color{
    Black,
    White
}

#[derive(PartialEq, Debug, Eq, Hash)]
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
    pub color: Color
}

#[derive(PartialEq, Debug)]
pub(crate) struct SinglePieceInfo {
    pub piece_type: PieceType,
    pub color: Color,
    pub position_x: usize,
    pub position_y: usize,
}

pub(crate) struct Move {
    pub piece_type:PieceType,
    pub old_position: u64,
    pub new_position: u64
}

pub(crate) struct ChessBoard {
    pub piece_infos: HashMap<Color, HashMap<PieceType, PieceInfo>>,
}

/// Basic implementation of a bitboard.
impl ChessBoard {

    // A function that sets a piece to a square on the board based on x, y coordinates
    // it uses PieceNames to identify the piece type
    // It uses the color of the piece to determine which bitboard to use
    // It uses the x, y coordinates to determine which bit to set
    // pub fn set_square(&mut self, x: usize, y: usize, piece: PieceNames, color: Color) {
    //     // convert the x, y coordinates to a bitboard position
    //     let position = 1 << (x + (y * 8));
    //     // set the bitboard position to 1
    //     match piece {
    //         PieceNames::Pawn => {
    //             match color {
    //                 Color::White => self.white_pawns.positions |= position,
    //                 Color::Black => self.black_pawns.positions |= position,
    //             }
    //         },
    //         PieceNames::Knight => {
    //             match color {
    //                 Color::White => self.white_knights.positions |= position,
    //                 Color::Black => self.black_knights.positions |= position,
    //             }
    //         },
    //         PieceNames::Bishop => {
    //             match color {
    //                 Color::White => self.white_bishops.positions |= position,
    //                 Color::Black => self.black_bishops.positions |= position,
    //             }
    //         },
    //         PieceNames::Rook => {
    //             match color {
    //                 Color::White => self.white_rooks.positions |= position,
    //                 Color::Black => self.black_rooks.positions |= position,
    //             }
    //         },
    //         PieceNames::Queen => {
    //             match color {
    //                 Color::White => self.white_queen.positions |= position,
    //                 Color::Black => self.black_queen.positions |= position,
    //             }
    //         },
    //         PieceNames::King => {
    //             match color {
    //                 Color::White => self.white_king.positions |= position,
    //                 Color::Black => self.black_king.positions |= position,
    //             }
    //         },
    //     }
    // }

    pub fn new(empty: bool) -> ChessBoard {
        let mut piece_infos = HashMap::new();
        let mut white_piece_infos = HashMap::new();
        let mut black_piece_infos = HashMap::new();

        let white_pawns = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::White
        };
        white_piece_infos.insert(PieceType::Pawn, white_pawns);

        let white_rooks = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001 },
            moves: 0,
            attacks: 0,
            color: Color::White
        };
        white_piece_infos.insert(PieceType::Rook, white_rooks);

        let white_knights = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010 },
            moves: 0,
            attacks: 0,
            color: Color::White
        };
        white_piece_infos.insert(PieceType::Knight, white_knights);

        let white_bishops = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100 },
            moves: 0,
            attacks: 0,
            color: Color::White
        };
        white_piece_infos.insert(PieceType::Bishop, white_bishops);

        let white_queen = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000 },
            moves: 0,
            attacks: 0,
            color: Color::White
        };
        white_piece_infos.insert(PieceType::Queen, white_queen);

        let white_king = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000 },
            moves: 0,
            attacks: 0,
            color: Color::White
        };
        white_piece_infos.insert(PieceType::King, white_king);

        let black_pawns = PieceInfo {
            positions: if empty { 0 } else { 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::Black
        };
        black_piece_infos.insert(PieceType::Pawn, black_pawns);

        let black_rooks = PieceInfo {
            positions: if empty { 0 } else { 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::Black
        };
        black_piece_infos.insert(PieceType::Rook, black_rooks);

        let black_knights = PieceInfo {
            positions: if empty { 0 } else { 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::Black
        };
        black_piece_infos.insert(PieceType::Knight, black_knights);

        let black_bishops = PieceInfo {
            positions: if empty { 0 } else { 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::Black
        };
        black_piece_infos.insert(PieceType::Bishop, black_bishops);

        let black_queen = PieceInfo {
            positions: if empty { 0 } else { 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::Black
        };
        black_piece_infos.insert(PieceType::Queen, black_queen);

        let black_king = PieceInfo {
            positions: if empty { 0 } else { 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 },
            moves: 0,
            attacks: 0,
            color: Color::Black
        };
        black_piece_infos.insert(PieceType::King, black_king);

        piece_infos.insert(Color::White, white_piece_infos);
        piece_infos.insert(Color::Black, black_piece_infos);
        
        // Initialize a new chessboard with the standard starting positions
        ChessBoard {
            piece_infos: piece_infos
        }
    }

    pub(crate) fn white_pawns(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Pawn, Color::White)
    }

    pub(crate) fn white_rooks(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Rook, Color::White)
    }

    pub(crate) fn white_knights(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Knight, Color::White)
    }

    pub(crate) fn white_bishops(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Bishop, Color::White)
    }

    pub(crate) fn white_queens(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Queen, Color::White)
    }

    pub(crate) fn white_kings(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::King, Color::White)
    }

    pub(crate) fn black_pawns(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Pawn, Color::Black)
    }

    pub(crate) fn black_rooks(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Rook, Color::Black)
    }

    pub(crate) fn black_knights(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Knight, Color::Black)
    }

    pub(crate) fn black_bishops(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Bishop, Color::Black)
    }

    pub(crate) fn black_queens(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::Queen, Color::Black)
    }

    pub(crate) fn black_kings(&mut self) -> &PieceInfo {
        self.get_mutable_piece_info(PieceType::King, Color::Black)
    }

    fn get_mutable_piece_info(&mut self, piece_type: PieceType, color: Color) -> &mut PieceInfo {
        self.piece_infos.get_mut(&color).unwrap().get_mut(&piece_type).unwrap()
    }

    /// Visualizes the bitboard by returning a Unicode represention
    pub(crate) fn visualize(&mut self) -> String {
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

    pub(crate) fn set_square(&mut self, piece_info: SinglePieceInfo) {
        let position: u64 = 1 << (7 - piece_info.position_x) + (piece_info.position_y * 8);
        let mutable = self.piece_infos
            .get_mut(&piece_info.color).unwrap()
            .get_mut(&piece_info.piece_type).unwrap();
            mutable.positions |= position;
    }
}

/// Takes a bitboard object and a position and returns the Unicode representation 
/// of that square
fn get_square_character(bitboard: &mut ChessBoard, x: i32, y: i32) -> &'static str {
    // TODO: Improve this by storing piece information in a array, or dictionary with
    // the pawn piece enum as the key value
    if get_square(bitboard.white_pawns().positions, x, y) {
       return "♙";
    }

    if get_square(bitboard.black_pawns().positions, x, y) {
        return "♟︎";
    }

    if get_square(bitboard.white_rooks().positions, x, y) {
        return "♖";
    }

    if get_square(bitboard.black_rooks().positions, x, y) {
        return "♜";
    }

    if get_square(bitboard.white_knights().positions, x, y) {
        return "♘";
    }

    if get_square(bitboard.black_knights().positions, x, y) {
        return "♞";
    }

    if get_square(bitboard.white_bishops().positions, x, y) {
        return "♗";
    }

    if get_square(bitboard.black_bishops().positions, x, y) {
        return "♝";
    }

    if get_square(bitboard.white_queens().positions, x, y) {
        return "♕";
    }

    if get_square(bitboard.black_queens().positions, x, y) {
        return "♛";
    }

    if get_square(bitboard.white_kings().positions, x, y) {
        return "♔";
    }

    if get_square(bitboard.black_kings().positions, x, y) {
        return "♚";
    }

    if (x + y) % 2 == 0 {
        return "■";
    }
    
    "□"  
}

/// Takes a bitboard bitstring and a position to check out if it is marked
fn get_square(positions: u64, x: i32, y: i32) -> bool {
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