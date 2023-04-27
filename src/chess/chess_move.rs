use super::piece::PieceType;

pub(crate) struct Move {
    pub piece_type:PieceType,
    pub old_position: u64,
    pub new_position: u64
}