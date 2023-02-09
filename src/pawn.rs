use crate::chess_game_bitboard::Color;
use crate::chess_game_bitboard::Move;
use crate::chess_game_bitboard::PieceNames;
use crate::board_utils::{A_FILE, H_FILE, RANK_SEVEN, RANK_FIVE, RANK_FOUR, RANK_TWO};


// Some constants for the directions that pawns can move
const FORWARD: u8 = 8;
const FORWARD_RIGHT: u8 = 7;
const FORWARD_LEFT: u8 = 9;

// ##################################
// # Start of white pawn functions  #
// ##################################

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

fn white_pawn_attacks(pawn_positions: u64, opponent_pieces: u64) -> u64 {
    // Shift the pawn bitboard one square to the left, then one square up.
    // H file is excluded so pawns are not shifted off the board
    let left_attacks = (pawn_positions << FORWARD_RIGHT ) & !A_FILE;

    // Shift the pawn bitboard one square to the right, then one square up.
    // A file is excluded so pawns are not shifted off the board
    let right_attacks = (pawn_positions << FORWARD_LEFT) & !H_FILE;

    // Use the OR operator to combine the left and right attacks.
    (left_attacks | right_attacks) & opponent_pieces
}

fn white_en_passant_calculation(last_move: &Move, pawn_positions: u64) -> u64 {

    let old_rank_check = RANK_SEVEN & last_move.old_position != 0;
    let new_rank_check = RANK_FIVE & last_move.new_position != 0;
    // If pawn did last move and did a double push. Determine en passant
    if last_move.piece_type == PieceNames::Pawn && old_rank_check && new_rank_check{
        // shift double pushed pawn one rank back to get the attack position for white
        let attack_position = (RANK_FIVE & last_move.new_position) << 8;
        return white_pawn_attacks(pawn_positions, attack_position);
    }
    // if last pawn did not do a double pawn push, return no attacks
    0
}

// ##################################
// # Start of black pawn functions  #
// ##################################

// A function to generate single pawn pushes for white pawns
fn black_pawn_single_push(pawn_positions: u64, empty_squares: u64) -> u64 {
    (pawn_positions >> FORWARD) & empty_squares
}

// A function to generate double pawn pushes for white pawns. 
// First do a single push, then again do a single push and check against rank 5
fn black_pawn_double_push(pawns: u64, empty_squares: u64) -> u64 {
    let single_push_squares = black_pawn_single_push(pawns, empty_squares);
    let double_push_sqares = black_pawn_single_push(single_push_squares, empty_squares);
    double_push_sqares & RANK_FIVE
}

fn black_pawn_attacks(pawn_positions: u64, opponent_pieces: u64) -> u64 {
    // Shift the pawn bitboard one square to the right, then one square up.
    // A file is excluded so pawns are not shifted off the board
    let left_attacks = (pawn_positions >> FORWARD_LEFT) & !A_FILE;

    // Shift the pawn bitboard one square to the left, then one square up.
    // H file is excluded so pawns are not shifted off the board
    let right_attacks = (pawn_positions >> FORWARD_RIGHT) & !H_FILE;

    // Use the OR operator to combine the left and right attacks.
    (left_attacks | right_attacks) & opponent_pieces
}

fn black_en_passant_calculation(last_move: &Move, pawn_positions: u64) -> u64 {
    let old_rank_check = RANK_TWO & last_move.old_position != 0;
    let new_rank_check = RANK_FOUR & last_move.new_position != 0;
    // If pawn did last move and did a double push. Determine en passant
    if last_move.piece_type == PieceNames::Pawn && old_rank_check && new_rank_check{
        // shift double pushed pawn one rank back to get the attack position for white
        let attack_position = (RANK_FOUR & last_move.new_position) >> 8;
        return black_pawn_attacks(pawn_positions, attack_position);
    }
    // if last pawn did not do a double pawn push, return no attacks
    0
}

// ###################################
// # Start of general pawn functions #
// ###################################

// A function to generate all moves for pawns depending on color
fn get_pawn_moves(pawn_positions: u64, empty_squares: u64, color: Color) -> u64 {
    let mut single_pushes = 0;
    let mut double_pushes = 0;
    if color == Color::White  {
        single_pushes = white_pawn_single_push(pawn_positions, empty_squares);
        double_pushes = white_pawn_double_push(pawn_positions, empty_squares);
    } else {
        single_pushes = black_pawn_single_push(pawn_positions, empty_squares);
        double_pushes = black_pawn_double_push(pawn_positions, empty_squares);
    }
    single_pushes | double_pushes
}
// get all possible attacks for the pawns
fn get_pawn_attack_set(last_move: &Move, pawn_positions: u64, opponent_pieces: u64, color: Color) -> u64{
    if color == Color::White{
        white_pawn_attacks(pawn_positions, opponent_pieces) | white_en_passant_calculation(last_move, pawn_positions)
    } else{
        black_pawn_attacks(pawn_positions, opponent_pieces) | black_en_passant_calculation(last_move, pawn_positions)
    }
}


mod tests {
    use crate::chess_game_bitboard::PieceNames;

    use super::*;

    // ##################################
    // # Start of white pawn unit tests #
    // ##################################

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

    #[test]
    fn test_white_pawn_attacks(){
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //some opponent pieces are in front of the pawns
        let opponent_pieces = 0b00000000_00000000_00000000_00000000_00000000_10101010_00000000_00000000;
        //all opponent pieces should be capturable
        let expected_result = opponent_pieces;
        let result = white_pawn_attacks(pawn_initial_position, opponent_pieces);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_pawn_attacks_files(){
        //rank 2 has a pawn on the A and H file
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_10000001_00000000;
        //some opponent pieces are in front of the pawns
        let opponent_pieces = 0b00000000_00000000_00000000_00000000_10000001_11000011_00000000_00000000;
        //only the opponent pieces diagonally from the pawns should be capturable
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_01000010_00000000_00000000;
        let result = white_pawn_attacks(pawn_initial_position, opponent_pieces);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_white_en_passant_calculation(){
        // There is one pawn on rank 5
        let pawn_positions = 0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000;
        // Last move was a double push of a pawn
        let last_move = Move {
                piece_type:PieceNames::Pawn,
                old_position: 0b00000000_00010000_00000000_00000000_00000000_00000000_00000000_00000000,
                new_position: 0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000
            };
        // En passant is possible
        let expected_result = 0b00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000000;
        let result = white_en_passant_calculation(&last_move, pawn_positions);
        assert_eq!(expected_result, result);
    }

    // ##################################
    // # Start of black pawn unit tests #
    // ##################################

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

    #[test]
    fn test_black_pawn_attacks(){
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //Some opponent pieces are in front of the pawns
        let opponent_pieces = 0b00000000_00000000_10101010_00000000_00000000_00000000_00000000_00000000;
        //All opponent pieces should capturable
        let expected_result = opponent_pieces;
        let result = black_pawn_attacks(pawn_initial_position, opponent_pieces);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_pawn_attacks_files(){
        //rank 7 has two pawns on the A and H file
        let pawn_initial_position = 0b00000000_10000001_00000000_00000000_00000000_00000000_00000000_00000000;
        //Some opponent pieces are in front of the pawns
        let opponent_pieces = 0b00000000_00000000_11000011_10000001_00000000_00000000_00000000_00000000;
        //Only the diagonal pieces should be allowed for capture
        let expected_result = 0b00000000_00000000_01000010_00000000_00000000_00000000_00000000_00000000;
        let result = black_pawn_attacks(pawn_initial_position, opponent_pieces);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_black_en_passant_calculation(){
        // There is one pawn on rank 5
        let pawn_positions = 0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000;
        // Last move was a double push of a pawn
        let last_move = Move {
                piece_type:PieceNames::Pawn,
                old_position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00010000_00000000,
                new_position: 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000
            };
        // En passant is possible
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000;
        let result = black_en_passant_calculation(&last_move, pawn_positions);
        assert_eq!(expected_result, result);
    }

    // ##################################################
    // # Start of general pawn functionality unit tests #
    // ##################################################
    #[test]
    fn test_get_pawn_moves_white(){
        //play with white
        let color = Color::White;
        //rank 2 is filled with pawns
        let pawn_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //single push and double push should be valid for all pawns
        let expected_result = 0b00000000_00000000_00000000_00000000_11111111_11111111_00000000_00000000;
        let result = get_pawn_moves(pawn_initial_position, empty_squares, color);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_get_pawn_moves_black(){
        let color = Color::Black;
        //rank 7 is filled with pawns
        let pawn_initial_position = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        //everything is empty except for the pawn positions
        let empty_squares = !0 ^ pawn_initial_position;
        //a pawn push should result in a full rank shift
        let expected_result = 0b00000000_00000000_11111111_11111111_00000000_00000000_00000000_00000000;
        let result = get_pawn_moves(pawn_initial_position, empty_squares, color);
        assert_eq!(expected_result, result);
    }

}

