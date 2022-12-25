
use crate::board_utils::{A_FILE, B_FILE, G_FILE, H_FILE, RANK_EIGHT, RANK_SEVEN, RANK_ONE, RANK_TWO};

fn knight_move_nnw(knight_position: u64) -> u64 {
    // If the knight is on file A, rank eight or rank seven, it should not be possible to move NNW. 
    (knight_position & !A_FILE & !(RANK_EIGHT | RANK_SEVEN )) << 17
}

fn knight_move_nww(knight_position: u64) -> u64 {
    // If the knight is on file A or B or rank 8, it should not be possible to move NWW. 
    (knight_position & !(A_FILE | B_FILE) & !RANK_EIGHT) << 10 
}

fn knight_move_nne(knight_position: u64) -> u64 {
    // If the knight is on file H, rank 8 or rank 7 it should not be possible to move NNE. 
    (knight_position & !H_FILE & !(RANK_EIGHT | RANK_SEVEN )) << 15 
}

fn knight_move_nee(knight_position: u64) -> u64 {
    // If the knight is on file G, H or rank 8 it should not be possible to move NEE. 
    (knight_position & !(G_FILE | H_FILE) & !RANK_EIGHT) << 6
}

fn knight_move_sww(knight_position: u64) -> u64 {
    // If the knight is on file A, B or rank 1 it should not be possible to move SWW. 
    (knight_position & !(A_FILE | B_FILE) & !RANK_ONE) >> 6 
}

fn knight_move_ssw(knight_position: u64) -> u64 {
    // If the knight is on file A, rank 1 or rank 2 it should not be possible to move SSW. 
    (knight_position & !A_FILE & !(RANK_ONE | RANK_TWO)) >> 15 
}

fn knight_move_see(knight_position: u64) -> u64 {
    // If the knight is on file G, H or rank 1 it should not be possible to move SEE. 
    (knight_position & !(G_FILE | H_FILE) & !RANK_ONE) >> 10
}

fn knight_move_sse(knight_position: u64) -> u64 {
    // If the knight is on file H, rank 1 or tank 2 it should not be possible to move SSE. 
    (knight_position & !H_FILE & !(RANK_ONE | RANK_TWO)) >> 17
}

mod tests{
    use super::*;

    #[test]
    fn test_knight_move_nne_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00001000_00000000_00000000_00000000_00000000_00000000;
        let result = knight_move_nne(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nne_invalid_ranks(){
        let knight_initial_position = 0b00100000_00100000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nne(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nne_invalid_file(){
        let knight_initial_position = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nne(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_nne_invalid_rank_and_file(){
        let knight_initial_position = 0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nne(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nee_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00000000_00000100_00000000_00000000_00000000_00000000;
        let result = knight_move_nee(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nee_invalid_ranks(){
        let knight_initial_position = 0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nee(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nee_invalid_file(){
        let knight_initial_position = 0b00000000_00000001_00000001_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nee(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_nee_invalid_rank_and_file(){
        let knight_initial_position = 0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nee(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_nww_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000;
        let result = knight_move_nww(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nww_invalid_rank(){
        let knight_initial_position = 0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nww(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nww_invalid_file(){
        let knight_initial_position = 0b00000000_0000000_11000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nww(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_nww_invalid_rank_and_file(){
        let knight_initial_position = 0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nww(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nnw_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00100000_00000000_00000000_00000000_00000000_00000000;
        let result = knight_move_nnw(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nnw_invalid_rank(){
        let knight_initial_position = 0b00100000_00100000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nnw(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_nnw_invalid_file(){
        let knight_initial_position = 0b00000000_0000000_10000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nnw(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_nnw_invalid_rank_and_file(){
        let knight_initial_position = 0b10000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_nnw(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_sse_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000;
        let result = knight_move_sse(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_sse_invalid_ranks(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00100000;
        let expected_result =       0;
        let result = knight_move_sse(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_sse_invalid_file(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_000000001_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_sse(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_sse_invalid_rank_and_file(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000;
        let expected_result =       0;
        let result = knight_move_sse(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_see_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000;
        let result = knight_move_see(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_see_invalid_ranks(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100;
        let expected_result =       0;
        let result = knight_move_see(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_see_invalid_file(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000011_00000011_00000000;
        let expected_result =       0;
        let result = knight_move_see(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_see_invalid_rank_and_file(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011;
        let expected_result =       0;
        let result = knight_move_see(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_sww_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000;
        let result = knight_move_sww(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_sww_invalid_rank(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000;
        let expected_result =       0;
        let result = knight_move_sww(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_sww_invalid_file(){
        let knight_initial_position = 0b00000000_0000000_00000000_00000000_00000000_11000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_sww(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_sww_invalid_rank_and_file(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000000;
        let expected_result =       0;
        let result = knight_move_sww(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_ssw_correct(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000;
        let result = knight_move_ssw(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_ssw_invalid_rank(){
        let knight_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00100000;
        let expected_result =       0;
        let result = knight_move_ssw(knight_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_knight_move_ssw_invalid_file(){
        let knight_initial_position = 0b00000000_0000000_00000000_00000000_00000000_10000000_00000000_00000000;
        let expected_result =       0;
        let result = knight_move_ssw(knight_initial_position);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_knight_move_ssw_invalid_rank_and_file(){
        let knight_initial_position = 0b10000000_00000000_00000000_00000000_00000000_00000000_10000000_00000000;
        let expected_result =       0;
        let result = knight_move_ssw(knight_initial_position);
        assert_eq!(expected_result, result);
    }
}

