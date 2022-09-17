use eframe::egui;
mod chess;
use chess::board::Board;

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
}

impl Default for MyEguiApp {
    fn default() -> Self {
        let width = 1000.0;
        let num_tiles = 8.0;
        Self {
            tile_width: width/num_tiles,
            board: Board::default(),
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
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
                        if self.board.try_move(xpos,ypos) {
                            _frame.set_window_title(self.board.turn_str());
                            self.board.selected_tile = (-1,-1);
                        }
                        else {
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
                        Some(_) => {
                            let image = self.board.get_image(j,i);
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
                                for (x,y) in self.board.get_moves(j,i) {
                                    draw_tile_outline(x as f32*self.tile_width,
                                                      y as f32*self.tile_width,
                                                      self.tile_width,ui);
                                }
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

