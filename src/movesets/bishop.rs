
use lazy_static::lazy_static;
use crate::{chess_game_bitboard::{PieceNames}};
use crate::movesets::magic_bitboards::*;

lazy_static! {
    static ref BLOCKERMASKS_BISHOP: [u64; 64] = generate_all_blockermasks(PieceNames::Bishop);
    static ref BLOCKERBOARDS_BISHOP: Vec<Vec<u64>> = generate_all_blockerboards(&BLOCKERMASKS_BISHOP);
    static ref MOVEBOARDS_BISHOP: Vec<Vec<u64>> = generate_all_moveboards(&BLOCKERBOARDS_BISHOP, PieceNames::Bishop);
    static ref MAGIC_TUPLE_BISHOP : ([u64; 64], Vec<Vec<Option<u64>>>)  = generate_magic_numbers(&BLOCKERBOARDS_BISHOP ,&MOVEBOARDS_BISHOP, &BLOCKERMASKS_BISHOP);
}

pub(crate) fn bishop_move(square : u64, occupancy : u64 ) -> u64{
    let index = square.trailing_zeros() as usize;
    let blockermask = BLOCKERMASKS_BISHOP[index];
    let blockerboard = occupancy & blockermask;
    let bits = blockermask.count_ones();
    let magic_number = MAGIC_TUPLE_BISHOP.0[index];
    let magic_index: u64 = (blockerboard.wrapping_mul(magic_number)) >> (64 - bits);
    let magic_move = MAGIC_TUPLE_BISHOP.1[index][magic_index as usize].unwrap();
    magic_move
}

mod tests {
    use super::*;

    #[test]
    fn test_bishop_move_no_blockers() {
        let square = 0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000;
        let occupancy = 0;
        let expected_move = 0b01000001_00100010_00010100_00000000_00010100_00100010_01000001_10000000;
        let actual_move = bishop_move(square, occupancy);
        assert_eq!(expected_move, actual_move);
    }
}