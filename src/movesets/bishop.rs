
use crate::movesets::magic_bitboards::*;

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
    #[ignore]
    fn test_bishop_move_no_blockers() {
        lazy_static::initialize(&MAGIC_TUPLE_BISHOP);
        let square = 0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000;
        let occupancy = 0;
        let expected_move = 0b01000001_00100010_00010100_00000000_00010100_00100010_01000001_10000000;
        let actual_move = bishop_move(square, occupancy);
        assert_eq!(expected_move, actual_move);
    }
}