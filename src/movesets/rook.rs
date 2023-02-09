use lazy_static::lazy_static;
use crate::{chess_game_bitboard::{PieceNames}};
use crate::movesets::magic_bitboards::*;use lazy_static::lazy_static;

// This class generate plain magic numbers and look-up tables for the Rooks and Bishops. 

lazy_static! {
    static ref BLOCKERMASKS_ROOK: [u64; 64] = generate_all_blockermasks(PieceNames::Rook);

    static ref BLOCKERBOARDS_ROOK: Vec<Vec<u64>> = generate_all_blockerboards(&BLOCKERMASKS_ROOK);

    static ref MOVEBOARDS_ROOK: Vec<Vec<u64>>= generate_all_moveboards(&BLOCKERBOARDS_ROOK, PieceNames::Rook);

    static ref MAGIC_TUPLE_ROOK : ([u64; 64], Vec<Vec<Option<u64>>>)  = generate_magic_numbers(&BLOCKERBOARDS_ROOK ,&MOVEBOARDS_ROOK, &BLOCKERMASKS_ROOK);

}