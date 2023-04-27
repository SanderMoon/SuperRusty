use super::piece::PieceType;

pub(crate) struct Move {
    pub piece_type:PieceType,
    pub from: u64,
    pub to: u64,
    pub special_move: bool,
    pub promoted_to: Option<PieceType>
}

impl Move {
    pub fn new(piece_type: PieceType, from: u64, to: u64, special_move: bool, promoted_to: Option<PieceType>) -> Move {
        Move {
            piece_type,
            from,
            to,
            special_move,
            promoted_to
        }
    }
}