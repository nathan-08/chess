use super::pieces::{ChessPiece, Color, Kind};
use std::collections::HashMap;
use egui_extras::image::RetainedImage;
use std::path::Path;
use eframe::egui;

pub struct Board {
    state: [[Option<ChessPiece>; 8]; 8],
    image_map: HashMap<(Kind, Color), RetainedImage>,
    pub selected_tile: (i32,i32),
}
impl Board {
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
    pub fn default() -> Self {
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
        Self {
            state,
            image_map: Board::make_image_map(),
            selected_tile: (-1,-1),
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
    pub fn try_move(&mut self,x:i32,y:i32) {
        let xcur = self.selected_tile.0;
        let ycur = self.selected_tile.1;
        if self.get_moves(xcur,ycur).contains(&(x,y)) {
            self.state[y as usize][x as usize] = self.state[ycur as usize][xcur as usize];
            self.state[ycur as usize][xcur as usize] = None;
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
                for xi in (0..x).rev() {
                    if !self.tile_occupied(xi,y) {
                        moves.push((xi,y));
                    }
                    else if self.tile_occupied_by_enemy(xi,y,color) {
                        moves.push((xi,y));
                        break;
                    }
                    else { break; }
                }
                for xi in x+1..=7 {
                    if !self.tile_occupied(xi,y) {
                        moves.push((xi,y));
                    }
                    else if self.tile_occupied_by_enemy(xi,y,color) {
                        moves.push((xi,y));
                        break;
                    }
                    else { break; }
                }
                for yi in (0..y).rev() {
                    if !self.tile_occupied(x,yi) {
                        moves.push((x,yi));
                    }
                    else if self.tile_occupied_by_enemy(x,yi,color) {
                        moves.push((x,yi));
                        break;
                    }
                    else { break; }
                }
                for yi in y+1..=7 {
                    if !self.tile_occupied(x,yi) {
                        moves.push((x,yi));
                    }
                    else if self.tile_occupied_by_enemy(x,yi,color) {
                        moves.push((x,yi));
                        break;
                    }
                    else { break; }
                }
            },
            Some(_) => (),
            None => (),
        };
        moves
    }
    pub fn tile_occupied(&self, x: i32, y: i32) -> bool {
        match self.get_piece(x,y) {
            Some(_) => true,
            None => false
        }
    }
    pub fn tile_occupied_by_enemy(&self,x:i32,y:i32,color:&Color) -> bool {
        let enemy_color = match color {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };
        match self.get_piece(x,y) {
            Some(ChessPiece{color,..}) => color == &enemy_color,
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
    if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
        v.push((x,y))
    }
}
