use crate::{board_utils::{RANKS, FILES}, chess_game_bitboard::{PieceNames}};

use lazy_static::lazy_static;

lazy_static! {
    static ref BLOCKERMASKS_ROOK: [u64; 64] = generate_all_blockermasks(PieceNames::Rook);
    static ref BLOCKERMASKS_BISHOP: [u64; 64] = generate_all_blockermasks(PieceNames::Bishop);

    static ref BLOCKERBOARDS_ROOK: Vec<Vec<u64>> = generate_all_blockerboards(&BLOCKERMASKS_ROOK);
    static ref BLOCKERBOARDS_BISHOP: Vec<Vec<u64>> = generate_all_blockerboards(&BLOCKERMASKS_BISHOP);

    static ref MOVEBOARDS_ROOK: Vec<Vec<u64>>= generate_all_moveboards(&BLOCKERBOARDS_ROOK, PieceNames::Rook);
    static ref MOVEBOARDS_BISHOP: Vec<Vec<u64>> = generate_all_moveboards(&BLOCKERBOARDS_BISHOP, PieceNames::Bishop);
}

pub(crate) fn blockermask_rook (square: u64) -> u64 {
    // Find the index of the least significant 1 in the bitboard
    let index = square.trailing_zeros() as u8;
    let row = index / 8;
    let col = index % 8;

    let pattern = generate_rook_move_pattern(row as i8, col as i8);
    // Remove the first and last columns and rows if the piece is not on there. 
    let blocker_mask = remove_edges(row, pattern, col);

    //remove the piece itself form the blocker mask and return
    blocker_mask ^ square
}

fn generate_rook_move_pattern(row: i8, col: i8) -> u64 {
    // Calculate the row and column of the index
    let mut pattern = 0;
    let row_mask = RANKS[row as usize];
    let col_mask = FILES[col as usize];
    pattern |= row_mask;
    pattern |= col_mask;
    pattern
}

pub(crate) fn blockermask_bishop (square: u64) -> u64 {
    // Find the index of the least significant 1 in the bitboard
    let index:u8 = square.trailing_zeros() as u8;

    // Calculate the row and column of the index
    let row: u8 = index / 8;
    let col: u8 = index % 8;

    let pattern = generate_bishop_move_pattern(row as i8, col as i8);

    // Remove the edges if piece is not positioned on the edge
    let blocker_mask = remove_edges(row, pattern, col);

    // Remove the piece itself from the blocker mask and return
    blocker_mask ^ square
}

fn remove_edges(row: u8, all_moves: u64, col: u8) -> u64 {
    let mut blocker_mask = all_moves;
    if row != 0 {
        blocker_mask &= !RANKS[0];
    }
    if row != 7 {
        blocker_mask &= !RANKS[7];
    }
    if col != 0 {
        blocker_mask &= !FILES[0];
    }
    if col != 7 {
        blocker_mask &= !FILES[7];
    }
    blocker_mask
}

fn generate_bishop_move_pattern(row: i8, col: i8) -> u64 {
    let mut pattern = 0;
    // find diagonal going SE from bishop position
    let mut i: i8 = row;
    let mut j: i8 = col;
    
    while i >= 0 && j >= 0 {
        pattern |= 1 << (8 * i + j);
        i -= 1;
        j -= 1;
    }

    // find diagonal going NW from bishop position
    i = row;
    j = col;
    while i < 8 && j < 8 {
        pattern |= 1 << (8 * i + j);
        i += 1;
        j += 1;
    }

    // find diagonal going NE from bishop position
    i = row;
    j = col;
    while i >= 0 && j < 8 {
        pattern |= 1 << (8 * i + j);
        i -= 1;
        j += 1;
    }

    // find diagonal going SW from bishop position
    i = row;
    j = col;
    while i < 8 && j >= 0 {
        pattern |= 1 << (8 * i + j);
        i += 1;
        j -= 1;
    }
    pattern
}

pub(crate) fn generate_all_blockermasks(piece_name: PieceNames) -> [u64; 64]{
    let mut blockermasks: [u64; 64] = [0; 64];
    for i in 0..64{
        let square = 1 << i;
        if piece_name == PieceNames::Rook {
            blockermasks[i] = blockermask_rook(square);
        } else {
            blockermasks[i] = blockermask_bishop(square);
        }
    }
    blockermasks
}

fn generate_blockerboard(index: u32, blockermask: u64) -> u64 {
    let mut blockerboard = blockermask;
    let mut bitindex: u8 = 0;

    for i in 0..64 {
        // if there is a 1 in the blockermask on index i
        if blockermask & (1 << i) != 0 {
            // If there is no 1 on the specified position in the index bit mask
            if (index & (1 << bitindex)) == 0 {
                // clear the bit from the blockerboard
                blockerboard &= !(1 << i);
            }
            bitindex += 1;
        }
    }
    blockerboard
}

fn generate_blockerboards_for_square(square: u8, blockermask: &[u64; 64], blockerboards: &mut Vec<Vec<u64>>) {
    let bits = blockermask[square as usize].count_ones();
    
    // Generate all possible combinations of bits in the blockermask.
    // For example, if the blockermask has 10 bits set, this loop will generate
    // all possible combinations of the positions and number of ones in the blockermask,
    // ranging from 0 to 2^10 = 1024.
    // On a normal chess board the maximum number of bits set in a blockermask is 12, so there are 4096 boards possible per mask. 
    for i in 0..(1 << bits) {
        blockerboards[square as usize][i as usize] = generate_blockerboard(i, blockermask[square as usize]);
    }
}

fn generate_all_blockerboards(blockermask: &[u64; 64]) -> Vec<Vec<u64>>{
    let mut blockerboards = vec![vec![0u64; 4096]; 64];
    for i in 0..64{
        generate_blockerboards_for_square(i, blockermask, &mut blockerboards);
    }
    blockerboards
}


fn generate_moveboard_for_square(square: u64, move_pattern : u64, blockerboard : u64) -> u64 {
    let index = square.trailing_zeros() as i8;
    let row: i8 = index / 8;
    let col: i8 = index % 8;
    let mut moveboard = move_pattern;

    let mut clear_switch = false;
    // find the southernmost bit and clear the ranks more south
    for y in (0..row).rev() {
        clear_axes(&mut clear_switch, &mut moveboard, y, col, y, blockerboard, RANKS);
    }

    clear_switch = false;
    //find the northermost bit and clear the rank more north
    for y in row..8 {
        clear_axes(&mut clear_switch, &mut moveboard, y, col, y, blockerboard, RANKS);
    }

    clear_switch = false;
    // find the westernmost bit and clear the files more west
    for x in col..8 {
        clear_axes(&mut clear_switch, &mut moveboard, row, x, x, blockerboard, FILES);
    }

    clear_switch = false;
    // find the easternmost bit and clear the files more eastern
    for x in (0..col).rev() {
        clear_axes(&mut clear_switch, &mut moveboard, row, x, x, blockerboard, FILES);
    }

    // clear the square itself to obtain the moveboard
    moveboard ^ square

}

fn clear_axes(clear_switch: &mut bool, moveboard: &mut u64, row: i8, col: i8, axes_to_clear: i8, blockerboard: u64, axis: [u64;8]) {
    // if the switch is active, clear the row
    if *clear_switch{
        *moveboard &= !axis[axes_to_clear as usize];
    }
    // isolate bit
    let next_bit = 1 << (8 * row + col);
    // The first encountered 1 will set the switch for clearing the rest of the move_pattern
    if next_bit & blockerboard != 0 {
        *clear_switch = true;
    }
}

fn generate_all_moveboards(blockerboards: &Vec<Vec<u64>>, piece_name: PieceNames) -> Vec<Vec<u64>>{
    // Q1 Does the move pattern match the blockerboard by index?
    //let move_pattern = generate_all_move_patterns(piece_name);
    let mut moveboards = vec![vec![0u64; 4096]; 64];
    for i in 0..64{
        let square = 1 << i;
        let move_pattern ;
        if piece_name == PieceNames::Rook {
            move_pattern = generate_rook_move_pattern(i / 8 as i8, i % 8 as i8);
        } else {
            move_pattern = generate_bishop_move_pattern(i / 8 as i8, i % 8 as i8);
        }
        
        for j in 0..4096{
            moveboards[i as usize][j] = generate_moveboard_for_square(square, move_pattern, blockerboards[i as usize][j]);
        }
    }
    moveboards
}

fn generate_all_move_patterns(piece_name: PieceNames) -> [u64; 64] {
    let mut move_pattern: [u64; 64] = [0; 64];
    for y in 0..8{
        for x in 0..8{
            if piece_name == PieceNames::Rook {
                move_pattern[8 * y + x] = generate_bishop_move_pattern(y as i8, x as i8);
            } else {
                move_pattern[8 * y + x] = generate_rook_move_pattern(y as i8, x as i8);
            }
        }
    }
    move_pattern
}

fn generate_magic_numbers(blockerboards: &Vec<Vec<u64>>, moveboards: &Vec<Vec<u64>>, blockermask: &[u64; 64]) -> [u64; 64] {
    let mut magic_numbers: [u64; 64] = [0; 64];
    for i in 0..64{
        let bits = blockermask[i].count_ones();
        let mut magic_number = 0;
        let mut found_magic_number = false;
        while !found_magic_number {
            magic_number += 1;
            let mut magic_number_found = true;
            for j in 0..4096{
                let index = (blockerboards[i][j] * magic_number) >> (64 - bits);
                if moveboards[i][index as usize] != moveboards[i][j] {
                    magic_number_found = false;
                    break;
                }
            }
            if magic_number_found {
                found_magic_number = true;
            }
        }
        magic_numbers[i] = magic_number;
    }
    magic_numbers
}


mod tests{

    use super::*;
    #[test]
    fn test_blockermask_rook(){
        let input =             0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =   0b00000000_00010000_00010000_00010000_01101110_00010000_00010000_00000000;
        let result = blockermask_rook(input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_generate_rook_move_pattern(){
        let square:u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000;
        let index = square.trailing_zeros() as i8;
        let row = index / 8;
        let col = index % 8;
        let result = generate_rook_move_pattern(row, col);
        let expected_result = 0b00000010_00000010_00000010_00000010_00000010_00000010_11111111_00000010;
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_blockermask_rook_corner(){
        let input =             0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        let expected_result =   0b00000000_00000001_00000001_00000001_00000001_00000001_00000001_01111110;
        let result = blockermask_rook(input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_generate_bishop_move_pattern(){
        let square:u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000;
        let index = square.trailing_zeros() as u8;
        let row = index / 8;
        let col = index % 8;
        let result = generate_bishop_move_pattern(row as i8, col as i8);
        let expected_result = 0b10000000_01000000_00100000_00010000_00001000_00000101_00000010_00000101;
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_remove_edges(){
        let row: u8 = 1;
        let mut blocker_mask: u64 = 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;
        let  col:u8 = 1;
        let expected_result:u64 =   0;
        let actual = remove_edges(row, blocker_mask, col);
        assert_eq!(expected_result, actual);
    }

    #[test]
    fn test_remove_edges_corner(){
        let row: u8 = 0;
        let mut blocker_mask: u64 = 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;
        let  col:u8 = 0;
        let expected_result:u64 =   0b00000000_00000001_00000001_00000001_00000001_00000001_00000001_01111111;
        let actual = remove_edges(row, blocker_mask, col);
        assert_eq!(expected_result, actual);
    }

    #[test]
    fn test_blockermask_bishop(){
        let input =             0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000;
        let expected_result =   0b00000000_00000010_01000100_00101000_00000000_00101000_01000100_00000000;
        let result = blockermask_bishop(input);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_blockermask_bishop_corner(){
        let input =             0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        let expected_result =   0b00000000_01000000_00100000_00010000_00001000_00000100_00000010_00000000;
        let result = blockermask_bishop(input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_generate_all_blockermasks_rook_no_null(){
        let result = generate_all_blockermasks(PieceNames::Rook);
        assert!(result.iter().all(|&x| x != 0));
    }

    #[test]
    fn test_generate_all_blockermasks_bishop_no_null(){
        let result = generate_all_blockermasks(PieceNames::Bishop);
        assert!(result.iter().all(|&x| x != 0));
    }

    #[test]
    fn test_generate_moveboard_for_square(){
        let square: u64 =           0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000;
        let move_pattern : u64 =    0b00010000_00010000_00010000_11111111_00010000_00010000_00010000_00010000; 
        let blockerboard : u64 =    0b00010000_00010000_00000000_10000100_00010000_00010000_00000000_00010000; 
        let expected_result : u64 = 0b00000000_00010000_00010000_11101100_00010000_00000000_00000000_00000000;
        let actual_result = generate_moveboard_for_square(square, move_pattern, blockerboard);
        assert_eq!(expected_result, actual_result);
        
    }

    #[test]
    fn test_generate_all_move_patterns_rook_not_empty(){
        let result = generate_all_move_patterns(PieceNames::Rook);
        assert!(result.iter().all(|&x| x != 0));
    }

    #[test]
    fn test_generate_all_move_patterns_bishop_not_empty(){
        let result = generate_all_move_patterns(PieceNames::Bishop);
        assert!(result.iter().all(|&x| x != 0));
    }

    #[test]
    fn test_generate_all_blockermasks_rook_not_empty(){
        let result = generate_all_blockermasks(PieceNames::Rook);
        assert!(result.iter().all(|&x| x != 0));
    }

    #[test]
    fn test_generate_all_blockermasks_bishop_not_empty(){
        let result = generate_all_blockermasks(PieceNames::Bishop);
        assert!(result.iter().all(|&x| x != 0));
    }

    // #[test]
    // fn test_generate_all_blockerboards(){
    //     let piece_name = PieceNames::Rook;
    //     let blockermasks = generate_all_blockermasks(piece_name);
    //     let mut blockerboards = generate_all_blockerboards(&blockermasks);
    //     let moveboards = generate_all_moveboards(&blockerboards, PieceNames::Rook);
    //     assert!(blockermasks.iter().all(|&x| x != 0));
    // }

    // #[test]
    // fn test_generate_magic_number(){
    //     let piece_name = PieceNames::Rook;
    //     let blockermasks = generate_all_blockermasks(piece_name);
    //     let blockerboards = generate_all_blockerboards(&blockermasks);
    //     let moveboards = generate_all_moveboards(&blockerboards, PieceNames::Rook);
    //     let magic_numbers = generate_magic_numbers(&blockerboards, &moveboards, &blockermasks);
    //     assert!(magic_numbers.iter().all(|&x| x != 0));
    // }

}