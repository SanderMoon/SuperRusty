use crate::movesets::magic_bitboards::*;

pub(crate) fn rook_move(square : u64, occupancy : u64 ) -> u64{
    let index = square.trailing_zeros() as usize;
    let blockermask = BLOCKERMASKS_ROOK[index];
    let blockerboard = occupancy & blockermask;
    let bits = blockermask.count_ones();
    let magic_number = MAGIC_TUPLE_ROOK.0[index];
    let magic_index: u64 = (blockerboard.wrapping_mul(magic_number)) >> (64 - bits);
    let magic_move = MAGIC_TUPLE_ROOK.1[index][magic_index as usize].unwrap();
    magic_move
}

mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_rook_move_no_blockers() {
        lazy_static::initialize(&MAGIC_TUPLE_ROOK);
        let square = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        let occupancy = 0;
        let expected_move = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_11111110;
        let actual_move = rook_move(square, occupancy);
        assert_eq!(expected_move, actual_move);
    }
}
