use super::pieces::{ChessPiece, Color, Kind};
use std::collections::HashMap;
use egui_extras::image::RetainedImage;
use std::path::Path;
use eframe::egui;

pub struct Board {
    state: [[Option<ChessPiece>; 8]; 8],
    image_map: HashMap<(Kind, Color), RetainedImage>,
    pub selected_tile: (i32,i32),
    player_turn: Color,
    winner: Option<Color>,
}
impl Board {
    pub fn default() -> Self {
        Self {
            state: get_initial_state(),
            image_map: make_image_map(),
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
    pub fn get_image(&self, x:i32, y:i32) -> &RetainedImage {
        match self.get_piece(x,y) {
            Some(ChessPiece{color, kind}) => {
                self.image_map.get(&(*kind, *color)).unwrap()
            },
            None => panic!("CANNOT GET IMAGE FOR EMPTY TILE"),
        }
    }
    pub fn get_piece(&self, x:i32, y:i32) -> &Option<ChessPiece> {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            &None
        } else {
            &self.state[y as usize][x as usize]
        }
    }
    pub fn try_move(&mut self,x:i32,y:i32) -> bool {
        if let Some(_) = self.winner { return false; }
        let xcur = self.selected_tile.0;
        let ycur = self.selected_tile.1;
        if let Some(ChessPiece{color,..}) = self.get_piece(xcur,ycur) {
            if color == &self.player_turn {
                if self.get_moves(xcur,ycur).contains(&(x,y)) {
                    self.state[y as usize][x as usize] = self.state[ycur as usize][xcur as usize];
                    self.state[ycur as usize][xcur as usize] = None;
                    // check for win condition
                    if self.checkmated(enemy_color(&self.player_turn)) {
                        self.winner = Some(self.player_turn);
                    }
                    else {
                        self.player_turn = enemy_color(&self.player_turn);
                    }
                    return true;
                }
            }
        }
        return false;
    }
    pub fn turn_piece_selected(&self) -> bool {
        match self.get_piece(self.selected_tile.0,self.selected_tile.1) {
            Some(ChessPiece{color,..}) => color == &self.player_turn,
            None => false
        }
    }
    pub fn get_moves(&self, x:i32, y:i32) -> Vec<(i32,i32)> {
        let mut moves = Vec::<(i32,i32)>::new();
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
                if self.tile_occupied_by_enemy(x+1,y+dir,&color) {
                    moves.push((x+1,y+dir));
                }
                if self.tile_occupied_by_enemy(x-1,y+dir,&color) {
                    moves.push((x-1,y+dir));
                };
            },
            Some(ChessPiece{kind:Kind::KNIGHT,color}) => {
                if !self.tile_occupied_by_ally(x+1,y+2,color) {
                    add_if_on_board(x+1,y+2,&mut moves); }
                if !self.tile_occupied_by_ally(x-1,y+2,color) {
                    add_if_on_board(x-1,y+2,&mut moves); }
                if !self.tile_occupied_by_ally(x+1,y-2,color) {
                    add_if_on_board(x+1,y-2,&mut moves); }
                if !self.tile_occupied_by_ally(x-1,y-2,color) {
                    add_if_on_board(x-1,y-2,&mut moves); }
                if !self.tile_occupied_by_ally(x+2,y+1,color) {
                    add_if_on_board(x+2,y+1,&mut moves); }
                if !self.tile_occupied_by_ally(x+2,y-1,color) {
                    add_if_on_board(x+2,y-1,&mut moves); }
                if !self.tile_occupied_by_ally(x-2,y+1,color) {
                    add_if_on_board(x-2,y+1,&mut moves); }
                if !self.tile_occupied_by_ally(x-2,y-1,color) {
                    add_if_on_board(x-2,y-1,&mut moves); }
            },
            Some(ChessPiece{kind:Kind::ROOK,color}) => {
                self.evaluate_ray(x,y, 1, 0,&color,&mut moves);
                self.evaluate_ray(x,y,-1, 0,&color,&mut moves);
                self.evaluate_ray(x,y, 0, 1,&color,&mut moves);
                self.evaluate_ray(x,y, 0,-1,&color,&mut moves);
            },
            Some(ChessPiece{kind:Kind::BISHOP,color}) => {
                self.evaluate_ray(x,y, 1, 1,&color,&mut moves);
                self.evaluate_ray(x,y, 1,-1,&color,&mut moves);
                self.evaluate_ray(x,y,-1, 1,&color,&mut moves);
                self.evaluate_ray(x,y,-1,-1,&color,&mut moves);
            },
            Some(ChessPiece{kind:Kind::QUEEN,color}) => {
                self.evaluate_ray(x,y, 1, 0,&color,&mut moves);
                self.evaluate_ray(x,y,-1, 0,&color,&mut moves);
                self.evaluate_ray(x,y, 0, 1,&color,&mut moves);
                self.evaluate_ray(x,y, 0,-1,&color,&mut moves);
                self.evaluate_ray(x,y, 1, 1,&color,&mut moves);
                self.evaluate_ray(x,y, 1,-1,&color,&mut moves);
                self.evaluate_ray(x,y,-1, 1,&color,&mut moves);
                self.evaluate_ray(x,y,-1,-1,&color,&mut moves);
            },
            Some(ChessPiece{kind:Kind::KING,color}) => {
                self.evaluate_king_move(x+1,y+1,color,&mut moves);
                self.evaluate_king_move(x+1,y  ,color,&mut moves);
                self.evaluate_king_move(x+1,y-1,color,&mut moves);
                self.evaluate_king_move(x-1,y+1,color,&mut moves);
                self.evaluate_king_move(x-1,y  ,color,&mut moves);
                self.evaluate_king_move(x-1,y-1,color,&mut moves);
                self.evaluate_king_move(x  ,y+1,color,&mut moves);
                self.evaluate_king_move(x  ,y-1,color,&mut moves);
            }
            None => (),
        };
        moves
    }
    fn evaluate_king_move(&self,x:i32,y:i32,color:&Color,moves:&mut Vec<(i32,i32)>) {
        if tile_on_board(x,y) && !self.tile_under_attack(x,y,color)
                && !self.tile_occupied_by_ally(x,y,color) {
            moves.push((x,y));
        }
    }
    fn checkmated(&self,color:Color) -> bool {
        let (kingx,kingy) = self.find_piece(color,Kind::KING).expect("no king found");
        self.tile_under_attack(kingx,kingy,&color) && self.get_moves(kingx,kingy).is_empty()
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
        println!("no piece found");
        None
    }
    fn switch_color(&self,x:i32,y:i32) {
        let piece = &mut self.state[y as usize][x as usize].unwrap();
        piece.color = enemy_color(&piece.color);
    }
    fn tile_under_attack(&self,x:i32,y:i32,color:&Color) -> bool {
        // TODO this is hacky :( .. find better solution
        // tile_under_attack utilized get_moves, which excludes tiles occupied by
        // an ally. however, for the case of tile_under_attack, we need to consider
        // the case that the ally occupying that tile may be taken.
        let mut switched = false;
        if self.tile_occupied_by_enemy(x,y,color) {
            switched = true;
            self.switch_color(x,y);
        }
        for xi in 0..7 {
            for yi in 0..7 {
                if self.tile_occupied_by_enemy(xi,yi,color) {
                    match self.get_piece(xi,yi) {
                        Some(ChessPiece{kind:Kind::PAWN,color:pawn_color}) => {
                            let dir = y_direction(pawn_color);
                            if y == yi + dir && (x == xi + 1 || x == xi - 1) {
                                if switched { self.switch_color(x,y); }
                                return true;
                            }
                        },
                        Some(ChessPiece{kind:Kind::KING,..}) => {
                            if x == xi && y == yi { continue; }
                            if x == xi + 1 || x == xi || x == xi - 1 {
                                if y == yi || y == yi + 1 || y == yi -1 {
                                    if switched { self.switch_color(x,y); }
                                    return true;
                                }
                            }
                        }
                        _ => {
                            if self.get_moves(xi,yi).contains(&(x,y)) {
                                if switched { self.switch_color(x,y); }
                                return true;
                            }
                        }
                    }
                }
            }
        }
        if switched { self.switch_color(x,y); }
        return false;
    }
    fn evaluate_ray(&self,x:i32,y:i32,dx:i32,dy:i32,color:&Color,moves:&mut Vec<(i32,i32)>) {
        for i in 1..=7 {
            let xi = x+i*dx;
            let yi = y+i*dy;
            if !tile_on_board(xi,yi) { break; }
            if !self.tile_occupied(xi,yi) {
                moves.push((xi,yi));
            }
            else if self.tile_occupied_by_enemy(xi,yi,color) {
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
fn get_image(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
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
fn make_image_map() -> HashMap<(Kind, Color), RetainedImage> {
    let white_pawn = get_image(Path::new("./src/images/pawn_white.png")).unwrap();
    let black_pawn = get_image(Path::new("./src/images/pawn_black.png")).unwrap();
    let white_rook = get_image(Path::new("./src/images/rook_white.png")).unwrap();
    let black_rook = get_image(Path::new("./src/images/rook_black.png")).unwrap();
    let white_knight = get_image(Path::new("./src/images/knight_white.png")).unwrap();
    let black_knight = get_image(Path::new("./src/images/knight_black.png")).unwrap();
    let white_bishop = get_image(Path::new("./src/images/bishop_white.png")).unwrap();
    let black_bishop = get_image(Path::new("./src/images/bishop_black.png")).unwrap();
    let white_queen = get_image(Path::new("./src/images/queen_white.png")).unwrap();
    let black_queen = get_image(Path::new("./src/images/queen_black.png")).unwrap();
    let white_king = get_image(Path::new("./src/images/king_white.png")).unwrap();
    let black_king = get_image(Path::new("./src/images/king_black.png")).unwrap();
    HashMap::from([
      ((Kind::PAWN, Color::WHITE),RetainedImage::from_color_image("w_pawn", white_pawn)),
      ((Kind::PAWN, Color::BLACK),RetainedImage::from_color_image("b_pawn", black_pawn)),
      ((Kind::ROOK, Color::WHITE),RetainedImage::from_color_image("w_rook", white_rook)),
      ((Kind::ROOK, Color::BLACK),RetainedImage::from_color_image("b_rook", black_rook)),
      ((Kind::KNIGHT, Color::WHITE),RetainedImage::from_color_image("w_knight", white_knight)),
      ((Kind::KNIGHT, Color::BLACK),RetainedImage::from_color_image("b_knight", black_knight)),
      ((Kind::BISHOP, Color::WHITE),RetainedImage::from_color_image("w_bishop", white_bishop)),
      ((Kind::BISHOP, Color::BLACK),RetainedImage::from_color_image("b_bishop", black_bishop)),
      ((Kind::QUEEN, Color::WHITE),RetainedImage::from_color_image("w_queen", white_queen)),
      ((Kind::QUEEN, Color::BLACK),RetainedImage::from_color_image("b_queen", black_queen)),
      ((Kind::KING, Color::WHITE),RetainedImage::from_color_image("w_king", white_king)),
      ((Kind::KING, Color::BLACK),RetainedImage::from_color_image("b_king", black_king)),
    ])
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
