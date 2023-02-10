
// A function that reads a FEN string and returns a ChessBoard object
// Example of such a string: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
pub fn read_fen(fen: &str) -> ChessBoard{
    // parse the fen notation into a vector of strings
    let fen_vec: Vec<&str> = fen.split_whitespace().collect();
    // create a new ChessBoard object
    let mut chess_board = ChessBoard::new();
    // set the board positions
    chess_board = set_board_positions(&mut chess_board, fen_vec[0]);

}

pub fn set_board_positions(chessboard: &mut ChessBoard, positions: &str){
    // split the positions string into a vector of strings
    let positions_vec: Vec<&str> = positions.split("/").collect();
    // loop through the vector of strings
    for (y, row) in positions_vec.iter().enumerate() {
        // loop through each character in the string
        for (x, character) in row.chars().enumerate() {
            // if the character is a digit, skip that many squares
            if character.is_digit(10) {
                // convert the character to a number
                let number = character.to_digit(10).unwrap();
                // skip that many squares
                for _ in 0..number {
                    // increment the x position
                    x += 1;
                }
            }
            // if the character is a letter, set the square to that piece
            else {
                // convert the character to a piece
                let piece = match character {
                    'p' => Piece::Pawn(PieceColor::Black),
                    'P' => Piece::Pawn(PieceColor::White),
                    'n' => Piece::Knight(PieceColor::Black),
                    'N' => Piece::Knight(PieceColor::White),
                    'b' => Piece::Bishop(PieceColor::Black),
                    'B' => Piece::Bishop(PieceColor::White),
                    'r' => Piece::Rook(PieceColor::Black),
                    'R' => Piece::Rook(PieceColor::White),
                    'q' => Piece::Queen(PieceColor::Black),
                    'Q' => Piece::Queen(PieceColor::White),
                    'k' => Piece::King(PieceColor::Black),
                    'K' => Piece::King(PieceColor::White),
                    _ => panic!("Invalid character in FEN string"),
                };
                // set the square to the piece
                chessboard.set_square(x, y, piece);
            }
        }
    }

}