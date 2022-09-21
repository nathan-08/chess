use super::pieces::{Color,Kind};
use super::board::Board;
#[derive(Clone,Copy,Debug)]
pub struct Ply {
    pub fromx: i32,
    pub fromy: i32,
    pub tox: i32,
    pub toy: i32,
}
pub fn make_move(board: &mut Board) {
    let player_color = &board.player_turn;
    let moves = board.get_moves_2(*player_color);
    let mut best_move = moves[0];
    let mut best_move_value = 0.0;
    moves.iter().for_each(|ply| {
        let new_state = board.perform_move_copy(*ply);
        let value = h_minimax(&new_state, 0, player_color);
        if value > best_move_value {
            best_move_value = value;
            best_move = *ply;
        }
    });
    board.selected_tile = (best_move.fromx, best_move.fromy);
    println!("best move: {:?}", best_move);
    board.perform_move_2(best_move);
}

fn h_minimax(state: &Board, depth:u32, ai_color: &Color) -> f32 {
    println!("h_minimax depth: {depth}");
    if depth == 3 || state.winner != None {
        evaluate_state(state, ai_color)
    }
    else {
        let moves = state.get_moves_2(state.player_turn);
        if moves.is_empty() {
            return evaluate_state(state, ai_color);
        }
        let mut max_value = 0.0;
        let mut min_value = 1.0;
        for ply@Ply{fromx,fromy,tox,toy} in moves {
            let new_state = state.perform_move_copy(ply);
            let value = h_minimax(&new_state, depth+1, ai_color);
            if value > max_value { max_value = value; }
            if value < min_value { min_value = value; }
        }
        if state.player_turn == *ai_color {
            max_value
        }
        else {
            min_value
        }
    }
}

fn evaluate_state(board: &Board, color: &Color) -> f32 {
    if let Some(winner_color) = board.winner {
        return if winner_color == *color { 1.0 } else { 0.0 }
    }
    let mut value = 39;
    for piece in board.get_pieces() {
        let sign = if piece.0.color == *color { 1 } else { -1 };
        value += sign * piece_value(piece.0.kind);
    }
    value as f32 / 78.0
}

fn piece_value(kind: Kind) -> i32 {
    match kind {
        Kind::PAWN => 1,
        Kind::KNIGHT => 3,
        Kind::BISHOP => 3,
        Kind::ROOK => 5,
        Kind::QUEEN => 9,
        Kind::KING => 0,
    }
}

