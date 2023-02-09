
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
    let blockerboard = occupancy & BLOCKERMASKS_BISHOP[index];
    let bits = blockerboard.count_ones();
    let magic_number = MAGIC_TUPLE_BISHOP.0[index];
    let magic_index = ((blockerboard & BLOCKERMASKS_BISHOP[index]).wrapping_mul(magic_number) >> (64 - bits)) as usize;
    let magic_move = MAGIC_TUPLE_BISHOP.1[index][magic_index].unwrap();
    magic_move
}