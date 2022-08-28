use eframe::egui;
use rand::Rng;
use std::thread;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2{x: 800.0, y: 800.0});
    native_options.resizable = false;
    eframe::run_native("Particle Simulation", native_options, Box::new(
            |cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    name: String,
    age: u32,
    x: f32,
    y: f32,
    x_min: f32,
    y_min: f32,
    x_max: f32,
    y_max: f32,
    fc: u32,
    vel: f32,
    rng: rand::rngs::ThreadRng,
    points_vec: Vec<Vec<(f32,f32,f32,f32)>>,
    paused: bool,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            name: "Bob".to_owned(),
            age: 42,
            x: 200.0,
            y: 100.0,
            x_min: 125.0,
            y_min: 5.0,
            x_max: 795.0,
            y_max: 795.0,
            fc: 0,
            vel: 5.0,
            rng: rand::thread_rng(),
            points_vec: vec![vec![(0.0, 0.0, 0.0, 0.0);200];4],
            paused: true
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        let x_min = app.x_min as u32;
        let x_max = app.x_max as u32;
        let y_min = app.y_min as u32;
        let y_max = app.y_max as u32;
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.points_vec[0],x_min,x_max, y_min, y_max);
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.points_vec[1],x_min,x_max, y_min, y_max);
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.points_vec[2],x_min,x_max, y_min, y_max);
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.points_vec[3],x_min,x_max, y_min, y_max);
        app
    }
    fn rand_position(&mut self) -> (f32, f32) {
        let new_x = self.rng.gen_range(self.x_min as u32..self.x_max as u32) as f32;
        let new_y = self.rng.gen_range(self.y_min as u32..self.y_max as u32) as f32;

        (new_x, new_y)
    }
    fn distribute_points_(rnd: &mut rand::rngs::ThreadRng,
                          points: &mut Vec<(f32, f32, f32, f32)>,
                          x_min: u32, x_max: u32, y_min: u32, y_max: u32) {
        for point in points {
            point.0 = rnd.gen_range(x_min..x_max) as f32;
            point.1 = rnd.gen_range(y_min..y_max) as f32;
            point.2 = 0.0;
            point.3 = 0.0;
        }
    }

    fn apply_forces_(&mut self, g: f32, pid1: usize, pid2: usize) {
            for i in 0..self.points_vec[pid1].len() {
                let mut fx = 0.0;
                let mut fy = 0.0;
                for j in 0..self.points_vec[pid2].len() {
                    let dx = self.points_vec[pid1][i].0 - self.points_vec[pid2][j].0;
                    let dy = self.points_vec[pid1][i].1 - self.points_vec[pid2][j].1;
                    let d = (dx*dx + dy*dy).sqrt();
                    if d > 0.0 && d < 180.0 {
                        let f = g * 1.0 / d;
                        fx += f * dx;
                        fy += f * dy;
                    }
                }
                self.points_vec[pid1][i].2 = (self.points_vec[pid1][i].2 + fx)*0.75;
                self.points_vec[pid1][i].3 = (self.points_vec[pid1][i].3 + fy)*0.75;


                //if self.points_vec[pid1][i].0 <= self.x_min
                    //|| self.points_vec[pid1][i].0 >= self.x_max {
                    //self.points_vec[pid1][i].2 *= -1.0;
                //}
                //if self.points_vec[pid1][i].1 <= self.y_min
                    //|| self.points_vec[pid1][i].1 >= self.y_max {
                    //self.points_vec[pid1][i].3 *= -1.0;
                //}

                if self.points_vec[pid1][i].0 < self.x_min {
                    self.points_vec[pid1][i].0 = self.x_min;
                    //self.points_vec[pid1][i].2 *= -1.0;
                }
                if self.points_vec[pid1][i].0 > self.x_max {
                    self.points_vec[pid1][i].0 = self.x_max;
                    //self.points_vec[pid1][i].2 *= -1.0;
                }
                if self.points_vec[pid1][i].1 < self.y_min {
                    self.points_vec[pid1][i].1 = self.y_min;
                    //self.points_vec[pid1][i].3 *= -1.0;
                }
                if self.points_vec[pid1][i].1 > self.y_max {
                    self.points_vec[pid1][i].1 = self.y_max;
                    //self.points_vec[pid1][i].3 *= -1.0;
                }

                self.points_vec[pid1][i].0 += self.points_vec[pid1][i].2;
                self.points_vec[pid1][i].1 += self.points_vec[pid1][i].3;

            }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.fc += 1;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Debug Info");
            ui.label(format!("=D"));
            //ui.label(format!("fc: {}", self.fc));

            //if ctx.input().key_down(egui::Key::ArrowRight) {
                //if self.x < self.x_max { self.x += self.vel; }
            //}
            //if ctx.input().key_down(egui::Key::ArrowLeft) {
                //if self.x > self.x_min { self.x -= self.vel; }
            //}
            //if ctx.input().key_down(egui::Key::ArrowDown) {
                //if self.y < self.y_max { self.y += self.vel; }
            //}
            //if ctx.input().key_down(egui::Key::ArrowUp) {
                //if self.y > self.y_min { self.y -= self.vel; }
            //}
            if ctx.input().key_pressed(egui::Key::R) {
                let x_min = self.x_min as u32;
                let x_max = self.x_max as u32;
                let y_min = self.y_min as u32;
                let y_max = self.y_max as u32;
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.points_vec[0],
                                              x_min, x_max, y_min, y_max);
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.points_vec[1],
                                              x_min, x_max, y_min, y_max);
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.points_vec[2],
                                              x_min, x_max, y_min, y_max);
            }
            if ctx.input().key_pressed(egui::Key::P) {
                self.paused = !self.paused;
            }
            if self.paused { return; }
            // 0: red, 1: blue, 2: green 3: yellow
            self.apply_forces_(-0.08,  0, 0);
            self.apply_forces_( 0.1,  1, 1);
            self.apply_forces_(-0.1,  2, 2);
            self.apply_forces_( 0.1,  3, 3);

            self.apply_forces_(-0.22, 0, 1);
            self.apply_forces_( 0.15, 1, 0);

            self.apply_forces_( 0.15, 0, 2);
            self.apply_forces_( 0.1,  2, 0);

            self.apply_forces_(-0.28, 1, 2);
            self.apply_forces_(-0.1,  2, 1);

            self.apply_forces_( 0.08,  2, 3);
            self.apply_forces_( 0.12,  0, 3);
            self.apply_forces_(-0.08,  3, 0);
            self.apply_forces_( 0.08,  3, 1);
            self.apply_forces_( 0.11,  3, 2);
            self.apply_forces_(-0.18,  1, 3);

            let canvas = egui::Frame::canvas(&ctx.style())
                .fill(egui::Color32::BLACK)
                .paint(egui::Rect{
                    min: egui::Pos2{x: 120.0, y: 0.0},
                    max: egui::Pos2{x: 800.0, y: 800.0} });

            ui.painter().add(canvas);
            for point in &self.points_vec[0] {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::RED,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            for point in &self.points_vec[1] {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::BLUE,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            for point in &self.points_vec[2] {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::GREEN,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            for point in &self.points_vec[3] {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::YELLOW,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            ctx.request_repaint();
        });
    }
}

