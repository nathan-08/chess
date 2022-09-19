use super::pieces::{ChessPiece, Color, Kind};

#[derive(Clone)]
pub struct Board {
    state: [[Option<ChessPiece>; 8]; 8],
    pub selected_tile: (i32,i32),
    pub player_turn: Color,
    pub winner: Option<Color>,
}
impl Board {
    pub fn default() -> Self {
        Self {
            state: get_initial_state(),
            selected_tile: (-1,-1),
            player_turn: Color::WHITE,
            winner: None,
        }
    }
    pub fn turn_str(&self) -> &str {
        if let Some(color) = self.winner {
            match color {
                Color::WHITE => &"White wins!",
                Color::BLACK => &"Black wins!",
            }
        }
        else {
            match self.player_turn {
                Color::WHITE => &"White's turn",
                Color::BLACK => &"Black's turn",
            }
        }
    }
    pub fn get_piece(&self, x:i32, y:i32) -> &Option<ChessPiece> {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            &None
        } else {
            &self.state[y as usize][x as usize]
        }
    }
    pub fn get_pieces(&self) -> Vec<(&ChessPiece,(i32,i32))> {
        let mut pieces = Vec::<(&ChessPiece,(i32,i32))>::new();
        for x in 0..=7 {
            for y in 0..=7 {
                if let Some(piece) = self.get_piece(x,y) {
                    pieces.push((piece,(x,y)));
                }
            }
        }
        pieces
    }
    pub fn get_player_pieces(&self,color:Color) -> Vec<(&ChessPiece,(i32,i32))> {
        self.get_pieces().iter().map(|m| *m).filter(|piece| piece.0.color == color).collect()
    }
    pub fn try_move(&mut self,x:i32,y:i32) -> bool {
        if let Some(_) = self.winner { return false; }
        let xcur = self.selected_tile.0;
        let ycur = self.selected_tile.1;
        if let Some(ChessPiece{color,..}) = self.get_piece(xcur,ycur) {
            if color == &self.player_turn {
                match self.get_piece(x,y) {
                    Some(ChessPiece{kind,..}) => println!("taking {:?}", kind),
                    _ => (),
                }
                self.state[y as usize][x as usize] = self.state[ycur as usize][xcur as usize];
                self.state[ycur as usize][xcur as usize] = None;
                // check for win condition
                if self.checkmated(enemy_color(&self.player_turn)) {
                    println!("checkmate!");
                    self.winner = Some(self.player_turn);
                    self.player_turn = enemy_color(&self.player_turn);
                }
                else {
                    println!("no checkmate");
                    self.player_turn = enemy_color(&self.player_turn);
                }
                return true;
            }
        }
        return false;
    }
    pub fn perform_move(&self,fromx:i32,fromy:i32,tox:i32,toy:i32) -> Option<Board> {
        let mut new_board = self.clone();
        if let Some(piece) = new_board.get_piece(fromx,fromy) {
            if piece.color == new_board.player_turn {
                new_board.selected_tile = (fromx, fromy);
                if new_board.try_move(tox,toy) {
                    return Some(new_board);
                }
            }
        }
        None
    }
    pub fn turn_piece_selected(&self) -> bool {
        match self.get_piece(self.selected_tile.0,self.selected_tile.1) {
            Some(ChessPiece{color,..}) => color == &self.player_turn,
            None => false
        }
    }
    pub fn get_moves(&self, x:i32, y:i32, consider_allies:bool) -> Vec<(i32,i32)> {
        let mut moves = Vec::<(i32,i32)>::new();
        let player_color: Option<Color>;
        match self.get_piece(x,y) {
            Some(ChessPiece{ kind:Kind::PAWN, color }) => {
                let dir = y_direction(&color);
                if !self.tile_occupied(x,y+dir) {
                    moves.push((x,y + dir));
                    if !pawn_has_moved(y,&color)
                        && !self.tile_occupied(x,y+2*dir) {
                        moves.push((x,y + 2*dir));
                    }
                }
                if self.tile_occupied_by_enemy(x+1,y+dir,&color) || (self.tile_occupied_by_ally(x+1,y+dir,&color) && consider_allies) {
                    moves.push((x+1,y+dir));
                }
                if self.tile_occupied_by_enemy(x-1,y+dir,&color) || (self.tile_occupied_by_ally(x-1,y+dir,&color) && consider_allies){
                    moves.push((x-1,y+dir));
                };
                player_color = Some(*color);
            },
            Some(ChessPiece{kind:Kind::KNIGHT,color}) => {
                if !self.tile_occupied_by_ally(x+1,y+2,color) || consider_allies {
                    add_if_on_board(x+1,y+2,&mut moves); }
                if !self.tile_occupied_by_ally(x-1,y+2,color) || consider_allies {
                    add_if_on_board(x-1,y+2,&mut moves); }
                if !self.tile_occupied_by_ally(x+1,y-2,color) || consider_allies {
                    add_if_on_board(x+1,y-2,&mut moves); }
                if !self.tile_occupied_by_ally(x-1,y-2,color) || consider_allies {
                    add_if_on_board(x-1,y-2,&mut moves); }
                if !self.tile_occupied_by_ally(x+2,y+1,color) || consider_allies {
                    add_if_on_board(x+2,y+1,&mut moves); }
                if !self.tile_occupied_by_ally(x+2,y-1,color) || consider_allies {
                    add_if_on_board(x+2,y-1,&mut moves); }
                if !self.tile_occupied_by_ally(x-2,y+1,color) || consider_allies {
                    add_if_on_board(x-2,y+1,&mut moves); }
                if !self.tile_occupied_by_ally(x-2,y-1,color) || consider_allies {
                    add_if_on_board(x-2,y-1,&mut moves); }
                player_color = Some(*color);
            },
            Some(ChessPiece{kind:Kind::ROOK,color}) => {
                self.evaluate_ray(x,y, 1, 0,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y,-1, 0,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 0, 1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 0,-1,&color,&mut moves,consider_allies);
                player_color = Some(*color);
            },
            Some(ChessPiece{kind:Kind::BISHOP,color}) => {
                self.evaluate_ray(x,y, 1, 1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 1,-1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y,-1, 1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y,-1,-1,&color,&mut moves,consider_allies);
                player_color = Some(*color);
            },
            Some(ChessPiece{kind:Kind::QUEEN,color}) => {
                self.evaluate_ray(x,y, 1, 0,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y,-1, 0,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 0, 1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 0,-1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 1, 1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y, 1,-1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y,-1, 1,&color,&mut moves,consider_allies);
                self.evaluate_ray(x,y,-1,-1,&color,&mut moves,consider_allies);
                player_color = Some(*color);
            },
            Some(ChessPiece{kind:Kind::KING,color}) => {
                self.evaluate_king_move(x+1,y+1,color,&mut moves,consider_allies);
                self.evaluate_king_move(x+1,y  ,color,&mut moves,consider_allies);
                self.evaluate_king_move(x+1,y-1,color,&mut moves,consider_allies);
                self.evaluate_king_move(x-1,y+1,color,&mut moves,consider_allies);
                self.evaluate_king_move(x-1,y  ,color,&mut moves,consider_allies);
                self.evaluate_king_move(x-1,y-1,color,&mut moves,consider_allies);
                self.evaluate_king_move(x  ,y+1,color,&mut moves,consider_allies);
                self.evaluate_king_move(x  ,y-1,color,&mut moves,consider_allies);
                player_color = Some(*color);
            }
            None => player_color = None,
        };
        // do not allow moves that leave king in check
        match player_color {
            Some(color) => {
                moves
                    .iter()
                    .map(|m| *m)
                    .filter(|(tox,toy)| !self.move_exposes_king_to_attack(x,y,*tox,*toy,&color))
                    .collect()
            },
            None => moves,
        }
    }
    fn move_exposes_king_to_attack(&self,
                                   fromx:i32,fromy:i32,tox:i32,toy:i32,color:&Color) -> bool {
        if let Some(hypothetical_state) = self.perform_move(fromx,fromy,tox,toy) {
            let (kingx, kingy) = hypothetical_state.find_piece(*color,Kind::KING).unwrap();
            return !hypothetical_state.tile_under_attack(kingx,kingy,color).is_empty();
        }
        return false;
    }
    fn evaluate_king_move(&self,x:i32,y:i32,color:&Color,moves:&mut Vec<(i32,i32)>,consider_allies:bool) {
        if tile_on_board(x,y) && self.tile_under_attack(x,y,color).is_empty()
                && (!self.tile_occupied_by_ally(x,y,color) || consider_allies) {
            moves.push((x,y));
        }
    }
    fn checkmated(&self,color:Color) -> bool {
        println!("checkmated? {:?}", color);
        let (kingx,kingy) = self.find_piece(color,Kind::KING).expect("no king found");
        let attackers = self.tile_under_attack(kingx,kingy,&color);
        !attackers.is_empty()
            && self.get_moves(kingx,kingy,false).is_empty()
            && attackers.iter().all(|(_,(x,y))| self.tile_under_attack(*x,*y,&enemy_color(&color)).is_empty())
    }
    fn find_piece(&self,color:Color,kind:Kind) -> Option<(i32,i32)> {
        for x in 0..=7 {
            for y in 0..=7 {
                if let Some(ChessPiece{color:c,kind:k}) = self.get_piece(x,y) {
                    if c == &color && k == &kind {
                        return Some((x,y));
                    }
                }
            }
        }
        None
    }
    //fn get_attackers(&self,x:i32,y:i32,color:&Color) -> Vec<(i32,i32)> {
        //let mut attackers = Vec::<(i32,i32)>::new();
        //for xi in 0..=7 {
            //for yi in 0..=7 {
                //if self.tile_occupied_by_enemy(xi,yi,color) {
                    //match self.get_piece(xi,yi) {
                        //Some(ChessPiece{kind:Kind::PAWN,color:pawn_color}) => {
                            //let dir = y_direction(pawn_color);
                            //if y == yi + dir && (x == xi + 1 || x == xi - 1) {
                                //attackers.push((xi,yi));
                            //}
                        //},
                        //Some(ChessPiece{kind:Kind::KING,..}) => {
                            //if x == xi && y == yi { continue; }
                            //if x == xi + 1 || x == xi || x == xi - 1 {
                                //if y == yi || y == yi + 1 || y == yi -1 {
                                    //attackers.push((xi,yi));
                                //}
                            //}
                        //},
                        //_ => {
                            //if self.get_moves(xi,yi,true).contains(&(x,y)) {
                                //attackers.push((xi,yi));
                            //}
                        //}
                    //}
                //}
            //}
        //}
        //attackers
    //}
    fn tile_under_attack(&self,x:i32,y:i32,color:&Color) -> Vec<(&ChessPiece,(i32,i32))> {
        let enemy_clr = &enemy_color(color);
        let mut attackers = Vec::<(&ChessPiece,(i32,i32))>::new();
        // check for knights
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x+2,y+1) {
                                    if color==enemy_clr{attackers.push((knight,(x+2,y+1)));}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x+2,y-1) {
                                    if color==enemy_clr{attackers.push((knight,(x+2,y-1)))}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x-2,y+1) {
                                    if color==enemy_clr{attackers.push((knight,(x-2,y+1)))}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x-2,y-1) {
                                    if color==enemy_clr{attackers.push((knight,(x-2,y-1)))}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x+1,y+2) {
                                    if color==enemy_clr{attackers.push((knight,(x+1,y+2)))}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x+1,y-2) {
                                    if color==enemy_clr{attackers.push((knight,(x+1,y-2)))}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x-1,y+2) {
                                    if color==enemy_clr{attackers.push((knight,(x-1,y+2)))}
        }
        if let Some(knight@ChessPiece{kind:Kind::KNIGHT,color}) = self.get_piece(x-1,y-2) {
                                    if color==enemy_clr{attackers.push((knight,(x-1,y-2)))}
        }
        // check for rooks,queen straight lines
        if let Some(p)=self.is_attacked_line(x,y, 1, 0, color) {attackers.push(p);}
        if let Some(p)=self.is_attacked_line(x,y,-1, 0, color) {attackers.push(p);}
        if let Some(p)=self.is_attacked_line(x,y, 0, 1, color) {attackers.push(p);}
        if let Some(p)=self.is_attacked_line(x,y, 0,-1, color) {attackers.push(p);}
        // check for bishops,queens
        if let Some(p)=self.is_attacked_diag(x,y, 1, 1, color) {attackers.push(p);}
        if let Some(p)=self.is_attacked_diag(x,y, 1,-1, color) {attackers.push(p);}
        if let Some(p)=self.is_attacked_diag(x,y,-1, 1, color) {attackers.push(p);}
        if let Some(p)=self.is_attacked_diag(x,y,-1,-1, color) {attackers.push(p);}
        // check for pawns
        let dir = y_direction(color);
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x-1,y+dir){if color==enemy_clr{attackers.push((p,(x-1,y+dir)))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x+1,y+dir){if color==enemy_clr{attackers.push((p,(x+1,y+dir)))}}
        // check for kings
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x+1,y+1){if color==enemy_clr{attackers.push((p,(x+1,y+1)))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x+1,y  ){if color==enemy_clr{attackers.push((p,(x+1,y  )))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x+1,y-1){if color==enemy_clr{attackers.push((p,(x+1,y-1)))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x-1,y+1){if color==enemy_clr{attackers.push((p,(x-1,y+1)))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x-1,y  ){if color==enemy_clr{attackers.push((p,(x-1,y  )))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x-1,y-1){if color==enemy_clr{attackers.push((p,(x-1,y-1)))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x  ,y+1){if color==enemy_clr{attackers.push((p,(x  ,y+1)))}}
        if let Some(p@ChessPiece{kind:Kind::PAWN,color})=self.get_piece(x  ,y-1){if color==enemy_clr{attackers.push((p,(x  ,y-1)))}}
        attackers
    }

    fn is_attacked_diag(&self,x:i32,y:i32,dx:i32,dy:i32,color:&Color) -> Option<(&ChessPiece,(i32,i32))> {
        for i in 0..=7 {
            if !tile_on_board(x+i*dx,y+i*dy) { break; }
            if self.tile_occupied_by_ally(x+i*dx,y+i*dy,color) { break; }
            if self.tile_occupied_by_enemy(x+i*dx,y+i*dy,color) {
                match self.get_piece(x+i*dx,y+i*dy) {
                    Some(p@ChessPiece{kind:Kind::BISHOP,..}) => return Some((p,(x+i*dx,y+i*dy))),
                    Some(p@ChessPiece{kind:Kind::QUEEN,..}) => return Some((p,(x+i*dx,y+i*dy))),
                    _ => break,
                }
            }
        }
        None
    }
    fn is_attacked_line(&self,x:i32,y:i32,dx:i32,dy:i32,color:&Color) -> Option<(&ChessPiece,(i32,i32))> {
        for i in 0..=7 {
            if !tile_on_board(x+i*dx,y+i*dy) { break; }
            if self.tile_occupied_by_ally(x+i*dx,y+i*dy,color) { break; }
            if self.tile_occupied_by_enemy(x+i*dx,y+i*dy,color) {
                match self.get_piece(x+i*dx,y+i*dy) {
                    Some(p@ChessPiece{kind:Kind::ROOK,..}) => return Some((p,(x+i*dx,y+i*dy))),
                    Some(p@ChessPiece{kind:Kind::QUEEN,..}) => return Some((p,(x+i*dx,y+i*dy))),
                    _ => break,
                }
            }
        }
        None
    }
    fn evaluate_ray(&self,x:i32,y:i32,dx:i32,dy:i32,color:&Color,moves:&mut Vec<(i32,i32)>,consider_allies:bool) {
        for i in 1..=7 {
            let xi = x+i*dx;
            let yi = y+i*dy;
            if !tile_on_board(xi,yi) { break; }
            if !self.tile_occupied(xi,yi) {
                moves.push((xi,yi));
            }
            else if self.tile_occupied_by_enemy(xi,yi,color) || (self.tile_occupied_by_ally(xi,yi,color) && consider_allies) {
                moves.push((xi,yi));
                break;
            }
            else { break; }
        }
    }
    pub fn tile_occupied(&self, x: i32, y: i32) -> bool {
        match self.get_piece(x,y) {
            Some(_) => true,
            None => false
        }
    }
    pub fn tile_occupied_by_enemy(&self,x:i32,y:i32,color:&Color) -> bool {
        match self.get_piece(x,y) {
            Some(ChessPiece{color:other_color,..}) => other_color == &enemy_color(color),
            None => false
        }
    }
    pub fn tile_occupied_by_ally(&self,x:i32,y:i32,color:&Color) -> bool {
        match self.get_piece(x,y) {
            Some(ChessPiece{color:target_color,..}) => target_color == color,
            None => false
        }
    }
}
fn y_direction(color: &Color) -> i32 { // returns 1 or -1
    match color {
        Color::BLACK => -1,
        Color::WHITE => 1,
    }
}
fn pawn_has_moved(y:i32, color: &Color) -> bool {
    match color {
        Color::BLACK => y != 6,
        Color::WHITE => y != 1,
    }
}
fn add_if_on_board(x:i32, y:i32, v: &mut Vec<(i32,i32)>) {
    if tile_on_board(x,y) {
        v.push((x,y))
    }
}
fn tile_on_board(x:i32,y:i32) -> bool {
    0 <= x && x <= 7 && 0 <= y && y <= 7
}
fn enemy_color(color:&Color) -> Color {
    match color {
        Color::BLACK => Color::WHITE,
        Color::WHITE => Color::BLACK,
    }
}
fn get_initial_state() -> [[Option<ChessPiece>; 8]; 8] {
    let mut state:[[Option<ChessPiece>; 8]; 8] = Default::default();
    state[0][0] = Some(ChessPiece::new(Color::WHITE, Kind::ROOK));
    state[0][1] = Some(ChessPiece::new(Color::WHITE, Kind::KNIGHT));
    state[0][2] = Some(ChessPiece::new(Color::WHITE, Kind::BISHOP));
    state[0][3] = Some(ChessPiece::new(Color::WHITE, Kind::QUEEN));
    state[0][4] = Some(ChessPiece::new(Color::WHITE, Kind::KING));
    state[0][5] = Some(ChessPiece::new(Color::WHITE, Kind::BISHOP));
    state[0][6] = Some(ChessPiece::new(Color::WHITE, Kind::KNIGHT));
    state[0][7] = Some(ChessPiece::new(Color::WHITE, Kind::ROOK));
    state[1][0] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][1] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][2] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][3] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][4] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][5] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][6] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));
    state[1][7] = Some(ChessPiece::new(Color::WHITE, Kind::PAWN));

    state[7][0] = Some(ChessPiece::new(Color::BLACK, Kind::ROOK));
    state[7][1] = Some(ChessPiece::new(Color::BLACK, Kind::KNIGHT));
    state[7][2] = Some(ChessPiece::new(Color::BLACK, Kind::BISHOP));
    state[7][3] = Some(ChessPiece::new(Color::BLACK, Kind::QUEEN));
    state[7][4] = Some(ChessPiece::new(Color::BLACK, Kind::KING));
    state[7][5] = Some(ChessPiece::new(Color::BLACK, Kind::BISHOP));
    state[7][6] = Some(ChessPiece::new(Color::BLACK, Kind::KNIGHT));
    state[7][7] = Some(ChessPiece::new(Color::BLACK, Kind::ROOK));
    state[6][0] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][1] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][2] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][3] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][4] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][5] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][6] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state[6][7] = Some(ChessPiece::new(Color::BLACK, Kind::PAWN));
    state
}
