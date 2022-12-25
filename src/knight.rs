
use crate::board_utils::{A_FILE, B_FILE, G_FILE, H_FILE};

fn knight_move_nne(knight_position: u64) -> u64 {
    (knight_position << 17) & !A_FILE
}

fn knight_move_nee(knight_position: u64) -> u64 {
    (knight_position << 10) & !(A_FILE | B_FILE)
}

fn knight_move_see(knight_position: u64) -> u64 {
    (knight_position >> 6) & !(A_FILE | B_FILE)
}

fn knight_move_sse(knight_position: u64) -> u64 {
    (knight_position >> 15) & !A_FILE
}

fn knight_move_nnw(knight_position: u64) -> u64 {
    (knight_position << 15) & !H_FILE
}

fn knight_move_nww(knight_position: u64) -> u64 {
    (knight_position << 6) & !(G_FILE | H_FILE)
}

fn knight_move_sww(knight_position: u64) -> u64 {
    (knight_position >> 10) & !(G_FILE | H_FILE)
}

fn knight_move_ssw(knight_position: u64) -> u64 {
    (knight_position >> 17) & !H_FILE
}
