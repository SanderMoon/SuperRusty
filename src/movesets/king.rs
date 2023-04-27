use crate::chess::chess_move::Move;
use crate::utils::board_utils::{A_FILE, H_FILE, RANK_EIGHT, RANK_ONE};
use crate::chess::piece::{PieceInfo, PieceType};



fn king_move_east(king_position : u64) -> u64{
    // shift one square east
    (king_position & !H_FILE) >> 1
}

fn king_move_west(king_position : u64) -> u64{
    // shift one square west
    (king_position & !A_FILE) << 1
}

fn king_moves_north(king_position_ew : u64) -> u64{
    // take the king position with east and west attack and shift north
    (king_position_ew & !RANK_EIGHT) << 8
}

fn king_moves_south(king_position_ew : u64) -> u64{
    // take the king position with east and west attack and shift south
    (king_position_ew & !RANK_ONE) >> 8
}

fn all_king_moves(king_position : u64) -> u64{
    // Calculate king with east and west positions
    let king_position_ew = king_position | king_move_east(king_position) | king_move_west(king_position);
    king_position ^ (king_position_ew | king_moves_north(king_position_ew) | king_moves_south(king_position_ew))
}

fn get_king_soft_moves(all_king_moves: u64, empty_squares: u64) -> u64{
    all_king_moves & empty_squares
}

fn get_king_attacks(all_king_moves: u64, opponent_pieces: u64) -> u64{
    all_king_moves & opponent_pieces
}

fn set_moves_attacks_kings(king: &mut PieceInfo, opponent_pieces: u64, empty_squares: u64){
    let all_king_moves = all_king_moves(king.positions);
    king.moves = get_king_soft_moves(all_king_moves, empty_squares);
    king.attacks = get_king_attacks(all_king_moves, opponent_pieces);
}

pub(crate) fn calculate_individual_king_moves(king: &mut PieceInfo, opponent_pieces: u64, empty_squares: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    set_moves_attacks_kings(king, opponent_pieces, empty_squares);
    let mut king_moves = king.moves | king.attacks;
    while king_moves != 0 {
        let king_move = king_moves & (!king_moves + 1);
        moves.push(Move::new(PieceType::King ,king.positions, king_move, false, None));
        king_moves ^= king_move;
    }
    moves
}

mod tests{
    use super::*;
    use crate::chess::piece::{PieceInfo, Color, PieceType};

    #[test]
    fn test_king_move_east_correct(){
        let king_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100;
        let result = king_move_east(king_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_move_east_incorrect_rank(){
        let king_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        let expected_result =       0;
        let result = king_move_east(king_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_move_west_correct(){
        let king_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        let result = king_move_west(king_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_move_west_incorrect_rank(){
        let king_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000;
        let expected_result =       0;
        let result = king_move_west(king_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_moves_north_correct(){
        let king_moves_ew =   0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00011100;
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_00000000_00011100_00000000;
        let result = king_moves_north(king_moves_ew);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_moves_north_incorrect_rank(){
        let king_moves_ew =   0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let expected_result = 0;
        let result = king_moves_north(king_moves_ew);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_moves_south_correct(){
        let king_moves_ew =   0b00000000_00000000_00000000_00000000_00000000_00000000_00011100_00000000;
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00011100;
        let result = king_moves_south(king_moves_ew);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_king_moves_south_incorrect_rank(){
        let king_moves_ew =   0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
        let expected_result = 0;
        let result = king_moves_south(king_moves_ew);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_all_king_moves(){
        let king_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00011100_00010100_00011100;
        let result = all_king_moves(king_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_all_king_moves_corner(){
        let king_initial_position = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        let expected_result =       0b00000000_00000000_00000000_00000000_00000000_00000000_00000011_00000010;
        let result = all_king_moves(king_initial_position);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_get_king_soft_moves(){

        let all_king_moves  =  0b00000000_00000000_00000000_00000000_00000000_00011100_00010100_00011100;
        let empty_squares   = !0b00000000_00000000_00000000_00000000_00000000_00000000_00010100_00011100;
        let expected_result = 0b00000000_00000000_00000000_00000000_00000000_00011100_00000000_00000000;
        let result = get_king_soft_moves(all_king_moves, empty_squares);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_get_king_attacks(){

        let all_king_moves  =  0b00000000_00000000_00000000_00000000_00000000_00011100_00010100_00011100;
        let opponent_pieces   = 0b00000000_00000000_00000000_00000000_00000000_00011100_00000000_00000000;
        let expected_result =  0b00000000_00000000_00000000_00000000_00000000_00011100_00000000_00000000;
        let result = get_king_attacks(all_king_moves, opponent_pieces);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_set_moves_attacks_kings(){
        let mut king = PieceInfo{
            positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            color: Color::White,
            moves: 0,
            attacks: 0,
            piece_type: PieceType::King
        };

        let opponent_pieces : u64 =   0b00000000_00000000_00000000_00000000_00000000_00000000_00011100_00000000;
        let empty_squares : u64 =     !opponent_pieces;

        set_moves_attacks_kings(&mut king, opponent_pieces, empty_squares);

        let expected: PieceInfo = PieceInfo{
            positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            color: Color::White,
            moves: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010100,
            attacks: 0b00000000_00000000_00000000_00000000_00000000_00000000_00011100_00000000,
            piece_type: PieceType::King
        };

        assert_eq!(expected, king);
        
    }

    #[test]
    fn test_calculate_individual_king_moves(){
        let mut king = PieceInfo{
            positions: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            color: Color::White,
            moves: 0,
            attacks: 0,
            piece_type: PieceType::King
        };

        let opponent_pieces : u64 =   0b00000000_00000000_00000000_00000000_00000000_00000000_00011100_00000000;
        let empty_squares : u64 =     !opponent_pieces;

        let result = calculate_individual_king_moves(&mut king, opponent_pieces, empty_squares);
        assert_eq!(5, result.len());
        
    }


}