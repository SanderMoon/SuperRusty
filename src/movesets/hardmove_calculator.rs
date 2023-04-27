

fn simulate_move(board: &ChessBoard, piece: PieceType, source_square: u64, target_square: u64, color: Color) -> ChessBoard {
    let mut new_board = board.clone();
    let piece_info = new_board.get_mutable_piece_info(piece, color);

    // Remove the piece from the source square
    piece_info.positions &= !source_square;

    // Place the piece on the target square
    piece_info.positions |= target_square;

    // Update the opponent's pieces to remove any captured piece (if any)
    let opponent_color = color.opposite();
    for opponent_piece_type in &[PieceType::Pawn, PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen, PieceType::King] {
        let opponent_piece_info = new_board.get_mutable_piece_info(*opponent_piece_type, opponent_color);
        opponent_piece_info.positions &= !target_square;
    }

    new_board
}


fn is_move_legal(board: &ChessBoard, piece: PieceType, source_square: u64, target_square: u64, color: Color) -> bool {
    let new_board = simulate_move(board, piece, source_square, target_square, color);

    // Update the moves and attacks for all pieces on the new board
    // ...
    // (Note: You'll need to modify your existing functions to take a mutable reference to a ChessBoard)

    // Check if the king is in check
    let king_info = new_board.get_piece_info(PieceType::King, color);
    let opponent_color = color.opposite();
    let king_square = king_info.positions;

    for opponent_piece_type in &[PieceType::Pawn, PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen, PieceType::King] {
        let opponent_piece_info = new_board.get_piece_info(*opponent_piece_type, opponent_color);
        if opponent_piece_info.attacks & king_square != 0 {
            // The king is in check after the move, so the move is not legal
            return false;
        }
    }

    // The king is not in check after the move, so the move is legal
    true
}


fn get_legal_moves(board: &ChessBoard, piece: PieceInfo) -> Vec<u64> {
    let mut legal_moves = Vec::new();
    let color = piece.color;
    let positions = piece.positions;
    let piece_type = piece.piece_type;
    let attacks = piece.attacks;
    let moves = piece.moves;

    // loop over all moves and check if they are legal
    for square in 0..64 {
        let square_mask = 1 << square;
        if moves & square_mask != 0 {
            if is_move_legal(board, piece_type, piece.positions, square_mask, color) {
                legal_moves.push(square_mask);
            }
        }
    }

}