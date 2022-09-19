use super::pieces::{Color,Kind};
use super::board::Board;
#[derive(Clone,Copy)]
struct Ply {
    fromx: i32,
    fromy: i32,
    tox: i32,
    toy: i32,
}
pub fn make_move(board: &mut Board) {
    let player_color = board.player_turn;
    let moves = get_moves(board);
    let mut best_move = moves[0];
    let mut best_move_value = 0.0;
    moves.iter().for_each(|ply| {
        if let Some(new_state) = board.perform_move(ply.fromx,ply.fromy,ply.tox,ply.toy) {
            let value = h_minimax(&new_state, 0, &player_color);
            if value > best_move_value {
                best_move_value = value;
                best_move = *ply;
            }
        } else {
            panic!("ai tried illegal move");
        }
    });
    board.selected_tile = (best_move.fromx, best_move.fromy);
    if !board.try_move(best_move.tox, best_move.toy) {
        panic!("ai tried illegal move");
    }
}

fn h_minimax(state: &Board, depth:u32, ai_color: &Color) -> f32 {
    println!("h_minimax depth: {depth}");
    if let Some(color) = state.winner {
        println!("WINNER {:?}", color);
    }
    if depth == 3 || state.winner != None {
        println!("returning");
        evaluate_state(state, ai_color)
    }
    else {
        let mut max_value = 0.0;
        let mut min_value = 1.0;
        for Ply{fromx,fromy,tox,toy} in get_moves(state) {
            if let Some(new_state) = state.perform_move(fromx,fromy,tox,toy) {
                let value = h_minimax(&new_state, depth+1, ai_color);
                if value > max_value { max_value = value; }
                if value < min_value { min_value = value; }
            } else { panic!("ai tried illegal move"); }
        }
        if state.player_turn == *ai_color {
            max_value
        }
        else {
            min_value
        }
    }
}

fn get_moves(state: &Board) -> Vec<Ply> {
    let mut moves = Vec::<Ply>::new();
    let player_color = state.player_turn;
    let player_pieces = state.get_player_pieces(player_color);
    for (_,(fromx,fromy)) in player_pieces {
        for (tox,toy) in state.get_moves(fromx,fromy,false) {
            moves.push(Ply{fromx,fromy,tox,toy});
        }
    }
    moves
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

