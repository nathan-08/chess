use super::pieces::{ChessPiece, Color, Kind};
use super::ai::Ply;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Board {
    state: [[Option<ChessPiece>; 8]; 8],
    pub selected_tile: (i32,i32),
    pub player_turn: Color,
    pub winner: Option<Color>,
    pub king_attackers: HashMap<Color,Vec<(ChessPiece,(i32,i32))>>
}
impl Board {
    pub fn default() -> Self {
        let mut king_attackers = HashMap::<Color,Vec<(ChessPiece,(i32,i32))>>::new();
        king_attackers.insert(
            Color::WHITE, Vec::<(ChessPiece,(i32,i32))>::new(),
            );
        king_attackers.insert(
            Color::BLACK, Vec::<(ChessPiece,(i32,i32))>::new(),
            );
        Self {
            state: get_initial_state(),
            selected_tile: (-1,-1),
            player_turn: Color::WHITE,
            winner: None,
            king_attackers,
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
    pub fn perform_move_copy(&self, ply:Ply) -> Self {
        let mut new_state = self.clone();
        new_state.perform_move_2(ply);
        new_state
    }
    pub fn perform_move_2(&mut self, ply:Ply) {
        let Ply{fromx,fromy,tox,toy} = ply;
        self.state[toy as usize][tox as usize] = self.state[fromy as usize][fromx as usize];
        self.state[fromy as usize][fromx as usize] = None;
        let enemy_clr = enemy_color(&self.player_turn);
        self.king_attackers.insert(enemy_clr, self.get_king_attackers(enemy_clr));
        self.player_turn = enemy_clr;
        // determine king attackers for (self.player_turn)
    }
    pub fn turn_piece_selected(&self) -> bool {
        match self.get_piece(self.selected_tile.0,self.selected_tile.1) {
            Some(ChessPiece{color,..}) => color == &self.player_turn,
            None => false
        }
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
    pub fn get_moves_2(&self,player_color:Color) -> Vec::<Ply> {
        let mut moves = Vec::<Ply>::new();
        for (piece,(fromx,fromy)) in self.get_player_pieces(player_color) {
            match piece {
                ChessPiece{kind:Kind::KING,..} => {
                    for (tox,toy) in self.get_moveto_tiles(fromx,fromy,Kind::KING,player_color) {
                        if self.tile_under_attack(tox,toy,&player_color,fromx,fromy).is_empty()
                            && !self.tile_occupied_by_ally(tox,toy,&player_color) {
                            moves.push(Ply{fromx,fromy,tox,toy});
                        }
                    }
                },
                ChessPiece{kind,..} => {
                    for (tox,toy) in self.get_moveto_tiles(fromx,fromy,*kind,player_color) {
                        // does move expose king?
                        if self.tile_occupied_by_ally(tox,toy,&player_color) {
                            continue;
                        }
                        if self.does_move_expose_king(fromx,fromy,tox,toy,player_color) {
                            continue;
                        }
                        let mut nullifies_attackers = true;
                        let (kingx,kingy) = self.find_piece(player_color, Kind::KING).unwrap();
                        for (attacker,(ax,ay)) in self.king_attackers.get(&player_color).unwrap() {
                            let ax = *ax; let ay = *ay;
                            if attacker.kind == Kind::KNIGHT || attacker.kind == Kind::PAWN {
                                if !(ax == tox && ay == toy) {
                                    nullifies_attackers = false;
                                }
                            }
                            // straight line
                            if attacker.kind == Kind::ROOK || attacker.kind == Kind::QUEEN {
                                if ax == kingx {
                                    if ax < kingx {
                                        if !(ax <= tox && tox < kingx) { nullifies_attackers = false; }
                                    }
                                    else if ax > kingx {
                                        if !(ax <= tox && tox > kingx) { nullifies_attackers = false; }
                                    }
                                } else if ay == kingy {
                                    if ay < kingy {
                                        if !(ay <= toy && toy < kingy) { nullifies_attackers = false; }
                                    }
                                    else if ay > kingy {
                                        if !(ay <= toy && toy > kingy) { nullifies_attackers = false; }
                                    }
                                }
                            }
                            // diagonal
                            if attacker.kind == Kind::BISHOP || attacker.kind == Kind::QUEEN {
                                let dx = if kingx > ax { -1 } else { 1 };
                                let dy = if kingy > ay { -1 } else { 1 };
                                for i in 0..=7 {
                                    if kingx+i*dx == tox && kingy+i*dy == toy {
                                        break;
                                    }
                                    if kingx+i*dx == ax && kingy+i*dy == ay {
                                        nullifies_attackers = false;
                                        break;
                                    }
                                }
                            }
                        }
                        if nullifies_attackers {
                            moves.push(Ply{fromx,fromy,tox,toy});
                        }
                    }
                }
            }
        }
        moves
    }
    fn tile_under_attack(&self,x:i32,y:i32,color:&Color,fromx:i32,fromy:i32) -> Vec<(&ChessPiece,(i32,i32))> {
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
        if let Some(p)=self.is_attacked_line(x,y, 1, 0, color,fromx,fromy) {attackers.push(p);}
        if let Some(p)=self.is_attacked_line(x,y,-1, 0, color,fromx,fromy) {attackers.push(p);}
        if let Some(p)=self.is_attacked_line(x,y, 0, 1, color,fromx,fromy) {attackers.push(p);}
        if let Some(p)=self.is_attacked_line(x,y, 0,-1, color,fromx,fromy) {attackers.push(p);}
        // check for bishops,queens
        if let Some(p)=self.is_attacked_diag(x,y, 1, 1, color,fromx,fromy) {attackers.push(p);}
        if let Some(p)=self.is_attacked_diag(x,y, 1,-1, color,fromx,fromy) {attackers.push(p);}
        if let Some(p)=self.is_attacked_diag(x,y,-1, 1, color,fromx,fromy) {attackers.push(p);}
        if let Some(p)=self.is_attacked_diag(x,y,-1,-1, color,fromx,fromy) {attackers.push(p);}
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

    fn is_attacked_diag(&self,x:i32,y:i32,dx:i32,dy:i32,color:&Color,xignore:i32,yignore:i32) -> Option<(&ChessPiece,(i32,i32))> {
        for i in 1..=7 {
            let xi = x+i*dx;
            let yi = y+i*dy;
            if !tile_on_board(xi,yi) { break; }
            if xi == xignore && yi == yignore { continue; }
            if self.tile_occupied_by_ally(xi,yi,color) { break; }
            if self.tile_occupied_by_enemy(xi,yi,color) {
                match self.get_piece(xi,yi) {
                    Some(p@ChessPiece{kind:Kind::BISHOP,..}) => return Some((p,(xi,yi))),
                    Some(p@ChessPiece{kind:Kind::QUEEN,..}) => return Some((p,(xi,yi))),
                    _ => break,
                }
            }
        }
        None
    }
    fn is_attacked_line(&self,x:i32,y:i32,dx:i32,dy:i32,color:&Color,xignore:i32,yignore:i32) -> Option<(&ChessPiece,(i32,i32))> {
        for i in 1..=7 {
            let xi = x+i*dx;
            let yi = y+i*dy;
            if xi == xignore && yi == yignore { continue; }
            if !tile_on_board(xi,yi) { break; }
            if self.tile_occupied_by_ally(xi,yi,color) { break; }
            if self.tile_occupied_by_enemy(xi,yi,color) {
                match self.get_piece(xi,yi) {
                    Some(p@ChessPiece{kind:Kind::ROOK,..}) => return Some((p,(xi,yi))),
                    Some(p@ChessPiece{kind:Kind::QUEEN,..}) => return Some((p,(xi,yi))),
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
    fn get_king_attackers(&self, king_color:Color) -> Vec<(ChessPiece,(i32,i32))> {
        let mut attackers = Vec::<(ChessPiece,(i32,i32))>::new();
        let (kingx, kingy) = self.find_piece(king_color,Kind::KING).expect("~king not found");
        let dir = y_direction(&king_color);
        let enemy_clr = enemy_color(&king_color);
        // check for knights
        self.add_if_kind(kingx+2,kingy+1,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx+2,kingy-1,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx-2,kingy+1,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx-2,kingy-1,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx+1,kingy+2,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx+1,kingy-2,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx-1,kingy+2,enemy_clr,Kind::KNIGHT,&mut attackers);
        self.add_if_kind(kingx-1,kingy-2,enemy_clr,Kind::KNIGHT,&mut attackers);
        // check for pawns
        self.add_if_kind(kingx+1,kingy+dir,enemy_clr,Kind::PAWN,&mut attackers);
        self.add_if_kind(kingx-1,kingy+dir,enemy_clr,Kind::PAWN,&mut attackers);
        // check outward rays
        // x   x   x
        //   x x x  
        // x x K x x
        //   x x x  
        // x   x   x
        // straight line
        self.add_if_in_line(kingx,kingy, 1, 0,&[Kind::ROOK,Kind::QUEEN],enemy_clr,&mut attackers);
        self.add_if_in_line(kingx,kingy,-1, 0,&[Kind::ROOK,Kind::QUEEN],enemy_clr,&mut attackers);
        self.add_if_in_line(kingx,kingy, 0, 1,&[Kind::ROOK,Kind::QUEEN],enemy_clr,&mut attackers);
        self.add_if_in_line(kingx,kingy, 0,-1,&[Kind::ROOK,Kind::QUEEN],enemy_clr,&mut attackers);
        // diagonal
        self.add_if_in_line(kingx,kingy, 1, 1,&[Kind::BISHOP,Kind::QUEEN],enemy_clr,&mut attackers);
        self.add_if_in_line(kingx,kingy, 1,-1,&[Kind::BISHOP,Kind::QUEEN],enemy_clr,&mut attackers);
        self.add_if_in_line(kingx,kingy,-1, 1,&[Kind::BISHOP,Kind::QUEEN],enemy_clr,&mut attackers);
        self.add_if_in_line(kingx,kingy,-1,-1,&[Kind::BISHOP,Kind::QUEEN],enemy_clr,&mut attackers);
        attackers
    }
    fn add_if_in_line(&self,x:i32,y:i32,dx:i32,dy:i32,kinds:&[Kind],color_to_match:Color,v:&mut Vec<(ChessPiece,(i32,i32))>) {
        for i in 1..=7 {
            let xi = x+i*dx;
            let yi = y+i*dy;
            match self.get_piece(xi,yi) {
                Some(p@ChessPiece{kind,color}) =>
                    if color == &color_to_match && kinds.contains(kind) {
                        v.push((*p,(xi,yi)))
                    }
                    else { break }
                _ => (),
            }
        }
    }
    fn add_if_kind(&self,x:i32,y:i32,color_a:Color,kind_a:Kind,v:&mut Vec<(ChessPiece,(i32,i32))>) {
        match self.get_piece(x,y) {
            Some(p@ChessPiece{kind:kind_b,color:color_b}) =>
                if &kind_a == kind_b && &color_a == color_b {v.push((*p,(x,y)))} else {},
            _ => (),
        }
    }
    fn does_move_expose_king(&self,fromx:i32,fromy:i32,tox:i32,toy:i32,color:Color) -> bool {
        // this function is to be used for pieces other than the king.
        // check outward rays
        // x   x   x
        //   x x x  
        // x x K x x
        //   x x x  
        // x   x   x
        // is piece moving out of ray?
        let (kingx,kingy) = self.find_piece(color,Kind::KING).unwrap();
        if fromy == kingy && toy != kingy {
            let dx = if kingx < fromx { 1 } else { -1 };
            match self.get_first_piece_in_dir(kingx,kingy, 0, dx) {
                Some(ChessPiece{kind,color:clr}) => {
                    if clr != color && [Kind::ROOK, Kind::QUEEN].contains(&kind) { return true; }
                },
                _ => (),
            }
        }
        if fromx == kingx && tox != kingx {
            let dy = if kingy < fromy { 1 } else { -1 };
            match self.get_first_piece_in_dir(kingx,kingy, dy, 0) {
                Some(ChessPiece{kind,color:clr}) => {
                    if clr != color && [Kind::ROOK, Kind::QUEEN].contains(&kind) { return true; }
                },
                _ => (),
            }
        }
        false
    }
    fn get_first_piece_in_dir(&self,x:i32,y:i32,dx:i32,dy:i32) -> Option<ChessPiece> {
        for i in 1..=7 {
            if !tile_on_board(x+i*dx,y+i*dy) {
                return None;
            }
            if let some@Some(_) = self.get_piece(x+i*dx,y+i*dy) {
                return *some;
            }
        }
        None
    }
    fn get_moveto_tiles(&self,x:i32,y:i32,kind:Kind,color:Color) -> Vec<(i32,i32)> {
        let mut tiles = Vec::<(i32,i32)>::new();
        match kind {
            Kind::PAWN => {
                let dir = y_direction(&color);
                if !pawn_has_moved(y,&color) {
                    if !self.tile_occupied_by_enemy(x,y+2*dir,&color) {
                        add_if_on_board(x,y+2*dir,&mut tiles);
                    }
                }
                if !self.tile_occupied_by_enemy(x,y+dir,&color) {
                    tiles.push((x,y+dir));
                }
                if self.tile_occupied_by_enemy(x+1,y+dir,&color) {
                    add_if_on_board(x+1,y+dir,&mut tiles);
                }
                if self.tile_occupied_by_enemy(x-1,y+dir,&color) {
                    add_if_on_board(x-1,y+dir,&mut tiles);
                }
            },
            Kind::KNIGHT => {
                add_if_on_board(x+2,y+1,&mut tiles);
                add_if_on_board(x+2,y-1,&mut tiles);
                add_if_on_board(x-2,y+1,&mut tiles);
                add_if_on_board(x-2,y-1,&mut tiles);
                add_if_on_board(x+1,y+2,&mut tiles);
                add_if_on_board(x+1,y-2,&mut tiles);
                add_if_on_board(x-1,y+2,&mut tiles);
                add_if_on_board(x-1,y-2,&mut tiles);
            },
            Kind::ROOK => {
                self.add_until_occupied(x,y, 1, 0,&mut tiles);
                self.add_until_occupied(x,y,-1, 0,&mut tiles);
                self.add_until_occupied(x,y, 0, 1,&mut tiles);
                self.add_until_occupied(x,y, 0,-1,&mut tiles);
            },
            Kind::BISHOP => {
                self.add_until_occupied(x,y, 1, 1,&mut tiles);
                self.add_until_occupied(x,y, 1,-1,&mut tiles);
                self.add_until_occupied(x,y,-1, 1,&mut tiles);
                self.add_until_occupied(x,y,-1,-1,&mut tiles);
            },
            Kind::QUEEN => {
                self.add_until_occupied(x,y, 1, 0,&mut tiles);
                self.add_until_occupied(x,y,-1, 0,&mut tiles);
                self.add_until_occupied(x,y, 0, 1,&mut tiles);
                self.add_until_occupied(x,y, 0,-1,&mut tiles);

                self.add_until_occupied(x,y, 1, 1,&mut tiles);
                self.add_until_occupied(x,y, 1,-1,&mut tiles);
                self.add_until_occupied(x,y,-1, 1,&mut tiles);
                self.add_until_occupied(x,y,-1,-1,&mut tiles);
            },
            Kind::KING => {
                add_if_on_board(x+1,y+1,&mut tiles);
                add_if_on_board(x+1,y  ,&mut tiles);
                add_if_on_board(x+1,y-1,&mut tiles);
                add_if_on_board(x-1,y+1,&mut tiles);
                add_if_on_board(x-1,y  ,&mut tiles);
                add_if_on_board(x-1,y-1,&mut tiles);
                add_if_on_board(x  ,y+1,&mut tiles);
                add_if_on_board(x  ,y-1,&mut tiles);
            },
        }
        tiles
    }
    fn add_until_occupied(&self,x:i32,y:i32,dx:i32,dy:i32,v:&mut Vec<(i32,i32)>) {
        for i in 1..=7 {
            let xi = x+i*dx;
            let yi = y+i*dy;
            if !tile_on_board(xi,yi) {
                break;
            }
            if self.tile_occupied(xi,yi) {
                v.push((xi,yi));
                break;
            }
            v.push((xi,yi));
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
pub fn enemy_color(color:&Color) -> Color {
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
