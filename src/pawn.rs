use std::convert::TryInto;

// Some constants for the directions that pawns can move
const FORWARD: u8 = 8;
const FORWARD_LEFT: u8 = 7;
const FORWARD_RIGHT: u8 = 9;
const RANK_FIVE: u64 = 0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
const RANK_FOUR: u64 = 0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;


/// Start white pawn functions

// A function to generate single pawn pushes for white pawns
fn white_pawn_single_push(pawn_positions: u64, empty_squares: u64) -> u64 {
    (pawn_positions << FORWARD) & empty_squares
}

// A function to generate double pawn pushes for white pawns. 
// First do a single push, then again do a single push and check against rank 4
fn white_pawn_double_push(pawns: u64, empty_squares: u64) -> u64 {
    let single_push_squares = white_pawn_single_push(pawns, empty_squares);
    let double_push_sqares = white_pawn_single_push(single_push_squares, empty_squares);
    double_push_sqares & RANK_FOUR
}

/// Start BLACK pawn functions

// A function to generate single pawn pushes for white pawns
fn black_pawn_single_push(pawn_positions: u64, empty_squares: u64) -> u64 {
    (pawn_positions >> FORWARD) & empty_squares
}

// A function to generate double pawn pushes for white pawns. 
// First do a single push, then again do a single push and check against rank 4
fn black_pawn_double_push(pawns: u64, empty_squares: u64) -> u64 {
    let single_push_squares = black_pawn_single_push(pawns, empty_squares);
    let double_push_sqares = black_pawn_single_push(single_push_squares, empty_squares);
    double_push_sqares & RANK_FIVE
}


mod tests {
    use super::*;

    /// ##################################
    /// # Start of white pawn unit tests #
    /// ##################################

    #[test]
    fn test_white_pawn_single_push_initial(){
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a pawn push should result in a full rank shift
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000;
        let result = white_pawn_single_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_single_push_missing(){
        //rank 2 is filled with some pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11011011_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a pawn push should result in a full rank shift
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_11011011_00000000_00000000;
        let result = white_pawn_single_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_single_push_blocked(){
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //some squares in front of the pawns are occupied
        let empty_squares = 0b11111111_11111111_11111111_11111111_11111111_10101010_00000000_11111111;
        //a pawn push should result in a full rank shift except for the blocked pawns
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_10101010_00000000_00000000;
        let result = white_pawn_single_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_double_push_initial(){
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a double pawn push should result in two full rank shifts
        let expected_result = 0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
        let result = white_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_double_push_missing(){
        //rank 2 is filled with some pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11011011_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a double pawn push should result in two full rank shifts
        let expected_result = 0b00000000_00000000_00000000_00000000_11011011_00000000_00000000_00000000;
        let result = white_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_double_push_blocked_one_sq(){
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //everything is empty except for the pawn positions and some blockades one rank in front
        let empty_squares = 0b11111111_11111111_11111111_11111111_11111111_10101010_00000000_11111111;
        //a double pawn push should result in two full rank shifts except for the blocked pawns
        let expected_result = 0b00000000_00000000_00000000_00000000_10101010_00000000_00000000_00000000;
        let result = white_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_double_push_blocked_two_sq(){
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //everything is empty except for the pawn positions and some blockades two ranks in front
        let empty_squares = 0b11111111_11111111_11111111_11111111_10101010_11111111_00000000_11111111;
        //a double pawn push should result in two full rank shifts except for the blocked pawns
        let expected_result = 0b00000000_00000000_00000000_00000000_10101010_00000000_00000000_00000000;
        let result = white_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    /// ##################################
    /// # Start of black pawn unit tests #
    /// ##################################

    #[test]
    fn test_black_pawn_single_push_initial(){
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a pawn push should result in a full rank shift
        let expected_result = 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;
        let result = black_pawn_single_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_single_push_missing(){
        //rank 7 is filled with some pawns
        let pawn_initial_position = 0b00000000_11011011_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a pawn push should result in a full rank shift
        let expected_result = 0b00000000_00000000_11011011_00000000_00000000_00000000_00000000_00000000;
        let result = black_pawn_single_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_single_push_blocked(){
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //some squares in front of the pawns are occupied
        let empty_squares = 0b11111111_00000000_10101010_11111111_11111111_11111111_11111111_11111111;
        //a pawn push should result in a full rank shift except for the blocked pawns
        let expected_result = 0b00000000_00000000_10101010_00000000_00000000_00000000_00000000_00000000;
        let result = black_pawn_single_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_double_push_initial(){
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a double pawn push should result in two full rank shifts
        let expected_result = 0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
        let result = black_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_double_push_missing(){
        //rank 7 is filled with some pawns
        let pawn_initial_position = 0b00000000_11011011_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a double pawn push should result in two full rank shifts
        let expected_result = 0b00000000_00000000_00000000_11011011_00000000_00000000_00000000_00000000;
        let result = black_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_double_push_blocked_one_sq(){
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions and some blockades one rank in front
        let empty_squares = 0b11111111_00000000_10101010_11111111_11111111_11111111_11111111_11111111;
        //a double pawn push should result in two full rank shifts except for the blocked pawns
        let expected_result = 0b00000000_00000000_00000000_10101010_00000000_00000000_00000000_00000000;
        let result = black_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_double_push_blocked_two_sq(){
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions and some blockades two ranks in front
        let empty_squares = 0b11111111_00000000_11111111_10101010_11111111_11111111_11111111_11111111;
        //a double pawn push should result in two full rank shifts except for the blocked pawns
        let expected_result = 0b00000000_00000000_00000000_10101010_00000000_00000000_00000000_00000000;
        let result = black_pawn_double_push(pawn_initial_position, empty_squares);
        assert_eq!(expected_result, result);
    }

}
