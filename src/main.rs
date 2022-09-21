use eframe::egui;
use chess::ai;
use chess::ai::Ply;
use std::collections::HashMap;
use egui_extras::image::RetainedImage;
use std::path::Path;
mod chess;
use chess::board::{Board,enemy_color};
use chess::pieces::{Color,Kind,ChessPiece};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2{x: 1000.0, y: 1000.0});
    native_options.resizable = false;
    eframe::run_native("Chess Game", native_options, Box::new(
            |cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    tile_width: f32,
    board: Board,
    image_map: HashMap<(Kind, Color), RetainedImage>,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        let width = 1000.0;
        let num_tiles = 8.0;
        Self {
            tile_width: width/num_tiles,
            board: Board::default(),
            image_map: make_image_map(),
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    pub fn get_image(&self, color: Color, kind: Kind) -> &RetainedImage {
        self.image_map.get(&(kind, color)).unwrap()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        _frame.set_window_title(self.board.turn_str());
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input().pointer.any_click() {
                match ctx.input().pointer.interact_pos() {
                    Some(egui::Pos2{x, y}) => {
                        let xpos = (x / self.tile_width).floor() as i32;
                        let ypos = (y / self.tile_width).floor() as i32;
                        if let Some(_) = self.board.winner { return; }
                        let available_moves = self.board.get_moves_2(self.board.player_turn);
                        if available_moves.is_empty() {
                            println!("no available moves");
                            _frame.set_window_title(self.board.turn_str());
                            return;
                        }
                        let mut made_move = false;
                        for ply@Ply{fromx,fromy,tox,toy} in self.board.get_moves_2(self.board.player_turn) {
                            if self.board.selected_tile.0 == fromx
                                && self.board.selected_tile.1 == fromy
                                && xpos == tox
                                && ypos == toy {
                                    self.board.perform_move_2(ply); // which function should process this?
                                    _frame.set_window_title(self.board.turn_str());
                                    self.board.selected_tile = (-1,-1);
                                    made_move = true;
                                    // now let ai have a turn
                                    ai::make_move(&mut self.board);
                                    _frame.set_window_title(self.board.turn_str());
                                    self.board.selected_tile = (-1,-1);
                            }
                        }
                        if !made_move {
                            self.board.selected_tile = (xpos,ypos);
                        }
                    },
                    None => println!("no mouse pos"),
                }
            }

            for i in 0..8 {
                for j in 0..8 {
                    let color = if (i + j) % 2 == 0 { egui::Color32::LIGHT_BLUE }
                    else { egui::Color32::KHAKI };
                    let x1 = i as f32 * self.tile_width;
                    let y1 = j as f32 * self.tile_width;
                    let x2 = x1 + self.tile_width;
                    let y2 = y1 + self.tile_width;
                    ui.painter().rect_filled(
                        egui::Rect{
                        min: egui::Pos2{x: x1, y: y1},
                        max: egui::Pos2{x: x2, y: y2},
                    },
                        egui::Rounding::none(),
                        color,
                    );
                }
            }
            for i in 0..8 {
                for j in 0..8 {
                    let xpos = self.tile_width * j as f32 +0.0;
                    let ypos = self.tile_width * i as f32 +0.0;
                    match self.board.get_piece(j,i) {
                        Some(ChessPiece{color,kind}) => {
                            let image = self.get_image(*color,*kind);
                            ui.put(
                                egui::Rect{min: egui::Pos2{x: xpos, y: ypos},
                                           max: egui::Pos2{x: xpos + self.tile_width,
                                                           y: ypos + self.tile_width }},
                                egui::Image::new(
                                    image.texture_id(ctx),
                                    egui::Vec2{x:image.width() as f32,y:image.height() as f32})
                            );
                            if self.board.selected_tile == (j,i)
                                && self.board.turn_piece_selected() {
                                draw_tile_outline(xpos,ypos,self.tile_width,ui);
                                //for (x,y) in self.board.get_moves(j,i,false) {
                                    //draw_tile_outline(x as f32*self.tile_width,
                                                      //y as f32*self.tile_width,
                                                      //self.tile_width,ui);
                                //}
                            }
                        },
                        None => (),
                    }
                }
            }
        });
    }
}
fn draw_tile_outline(xpos:f32,ypos:f32,tile_width:f32,ui: &mut egui::Ui) {
    ui.painter().rect_stroke(
        egui::Rect{
            min: egui::Pos2{x: xpos, y: ypos},
            max: egui::Pos2{x: xpos + tile_width,
                            y: ypos + tile_width},
        },
        egui::Rounding::same(2.0),
        egui::Stroke{
            width: 5.0,
            color: egui::Color32::BLUE,
        },
    );
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
