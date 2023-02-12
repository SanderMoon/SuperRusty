use crate::movesets::bishop::*;
use crate::movesets::rook::*;

pub(crate) fn queen_move(square : u64, occupancy : u64 ) -> u64{
    bishop_move(square, occupancy) | rook_move(square, occupancy)
}

mod tests {
    use super::*;
    use crate::movesets::magic_bitboards::initialize;

    #[test]
    #[ignore]
    fn test_queen_move_no_blockers() {
        initialize();
        let square = 0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000;
        let occupancy = 0;
        let expected_move = 0b01001001_00101010_00011100_11110111_00011100_00101010_01001001_10001000;
        let actual_move = queen_move(square, occupancy);
        assert_eq!(expected_move, actual_move);
    }
}


