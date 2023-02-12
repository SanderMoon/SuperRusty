use crate::chess_game_bitboard::{ChessBoard, SinglePieceInfo, PieceType, Color, Castling};

// A function that reads a FEN string and returns a ChessBoard object
// Example of such a string: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
pub(crate) fn read_fen(fen: &str) -> ChessBoard{
    // parse the fen notation into a vector of strings
    let fen_vec: Vec<&str> = fen.split_whitespace().collect();
    // create a new ChessBoard object
    let mut chess_board = ChessBoard::new(true);
    // set the board positions
    set_board_positions(&mut chess_board, fen_vec[0]);
    // set the active color
    set_active_color(&mut chess_board, fen_vec[1]);
    // set the castling rights
    set_castling_rights(&mut chess_board, fen_vec[2]);
    chess_board

}

fn set_castling_rights(chess_board: &mut ChessBoard, castling_string: &str) {
    // if the castling rights are "-", do nothing
    if castling_string == "-" {
        return;
    }
    // loop through the castling rights string
    for character in castling_string.chars() {
        // set the castling rights for each character
        match character {
            'K' => {
                chess_board.set_castling(Color::White, Castling::KingSide);
            },
            'Q' => {
                chess_board.set_castling(Color::White, Castling::QueenSide);
            },
            'k' => {
                chess_board.set_castling(Color::Black, Castling::KingSide);
            },
            'q' => {
                chess_board.set_castling(Color::Black, Castling::KingSide);
            },
            _ => panic!("Invalid character in FEN string"),
        }
    }
}

fn set_active_color(chess_board: &mut ChessBoard, active_color: &str) {
    chess_board.set_active_color(match active_color {
        "w" => Color::White,
        "b" => Color::Black,
        _ => panic!("Invalid color in FEN string"),
    });
}

fn set_board_positions(chessboard: &mut ChessBoard, positions: &str){
    // split the positions string into a vector of strings
    let positions_vec: Vec<&str> = positions.split("/").collect();
    // loop through the vector of strings
    for (y, row) in positions_vec.iter().enumerate() {
        // loop through each character in the string
        let mut x = 0;
        for (_, character) in row.chars().enumerate() {
            // if the character is a digit, skip that many squares
            if character.is_digit(10) {
                // convert the character to a number
                let number = character.to_digit(10).unwrap();
                // skip that many squares
                x += number as usize;
            }
            // if the character is a letter, set the square to that piece
            else {
                // convert the character to a piece
                let mut piece_type: PieceType = PieceType::Pawn;
                let mut color: Color = Color::Black;
                let piece = match character {
                    'p' => { },
                    'P' => {
                        color = Color::White;
                    },
                    'n' => {
                        piece_type = PieceType::Knight;
                    },
                    'N' => {
                        piece_type = PieceType::Knight;
                        color = Color::White;
                    },
                    'b' => {
                        piece_type = PieceType::Bishop;
                    },
                    'B' => {
                        piece_type = PieceType::Bishop;
                        color = Color::White;
                    },
                    'r' => {
                        piece_type = PieceType::Rook;
                    },
                    'R' => {
                        piece_type = PieceType::Rook;
                        color = Color::White;
                    },
                    'q' => {
                        piece_type = PieceType::Queen;
                    },
                    'Q' => {
                        piece_type = PieceType::Queen;
                        color = Color::White;
                    },
                    'k' => {
                        piece_type = PieceType::King;
                    },
                    'K' => {
                        piece_type = PieceType::King;
                        color = Color::White;
                    },
                    _ => panic!("Invalid character in FEN string"),
                };

                // set the square to the piece
                chessboard.set_square(SinglePieceInfo {
                    piece_type,
                    color,
                    position_x: x,
                    position_y: 7 - y,
                });

                x += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let mut chess_board = read_fen(fen);
        let mut start_board = ChessBoard::new(false);

        assert_eq!(chess_board.white_pawns().positions, 0b00001000_00000000_11110111_00000000);
        assert_eq!(chess_board.white_knights().positions, start_board.white_knights().positions);
        assert_eq!(chess_board.white_bishops().positions, start_board.white_bishops().positions);
        assert_eq!(chess_board.white_rooks().positions, start_board.white_rooks().positions );
        assert_eq!(chess_board.white_queens().positions, start_board.white_queens().positions);
        assert_eq!(chess_board.white_kings().positions, start_board.white_kings().positions);
        assert_eq!(chess_board.black_pawns().positions, start_board.black_pawns().positions);
        assert_eq!(chess_board.black_knights().positions, start_board.black_knights().positions);
        assert_eq!(chess_board.black_bishops().positions, start_board.black_bishops().positions);
        assert_eq!(chess_board.black_rooks().positions, start_board.black_rooks().positions);
        assert_eq!(chess_board.black_queens().positions, start_board.black_queens().positions);
        assert_eq!(chess_board.black_kings().positions, start_board.black_kings().positions);
        assert_eq!(Color::Black, chess_board.active_color);
        assert_eq!(true, chess_board.white_king_side_castle);
        assert_eq!(true, chess_board.white_queen_side_castle);
        assert_eq!(true, chess_board.black_king_side_castle);
        assert_eq!(true, chess_board.black_queen_side_castle);

    }
}