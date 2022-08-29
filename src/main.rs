use eframe::egui;
use rand::Rng;
use std::thread;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2{x: 1000.0, y: 800.0});
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
    red: Vec<(f32,f32,f32,f32)>,
    blue: Vec<(f32,f32,f32,f32)>,
    green: Vec<(f32,f32,f32,f32)>,
    yellow: Vec<(f32,f32,f32,f32)>,
    paused: bool,

    force_locality: i32,
    force_multiplier: f32,

    yty: f32,
    btb: f32,
    gtg: f32,
    rtr: f32,

    rtb: f32,
    gtb: f32,
    ytb: f32,

    rtg: f32,
    ytg: f32,
    btg: f32,

    ytr: f32,
    gtr: f32,
    btr: f32,

    rty: f32,
    gty: f32,
    bty: f32,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            name: "Bob".to_owned(),
            age: 42,
            x: 200.0,
            y: 100.0,
            x_min: 205.0,
            y_min: 5.0,
            x_max: 995.0,
            y_max: 795.0,
            fc: 0,
            vel: 5.0,
            rng: rand::thread_rng(),
            red: vec![(0.0,0.0,0.0,0.0);200],
            blue: vec![(0.0,0.0,0.0,0.0);200],
            green: vec![(0.0,0.0,0.0,0.0);200],
            yellow: vec![(0.0,0.0,0.0,0.0);200],
            paused: true,

            force_locality: 100,
            force_multiplier: 0.5,

            yty: 0.0,
            btb: 0.0,
            rtr: 0.0,
            gtg: 0.0,

            rtb: 0.0,
            ytb: 0.0,
            gtb: 0.0,

            rtg: 0.0,
            ytg: 0.0,
            btg: 0.0,

            ytr: 0.0,
            gtr: 0.0,
            btr: 0.0,

            rty: 0.0,
            gty: 0.0,
            bty: 0.0,
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
                                      &mut app.red,x_min,x_max, y_min, y_max);
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.blue,x_min,x_max, y_min, y_max);
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.green,x_min,x_max, y_min, y_max);
        MyEguiApp::distribute_points_(&mut app.rng,
                                      &mut app.yellow,x_min,x_max, y_min, y_max);
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

    fn apply_forces_to_self(g: f32, v1: &mut Vec<(f32,f32,f32,f32)>,
                      x_min: f32, x_max: f32, y_min: f32, y_max: f32,
                      force_locality: i32, force_multiplier: f32) {
        for i in 0..v1.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;
            for j in 0..v1.len() {
                let dx = v1[i].0 - v1[j].0;
                let dy = v1[i].1 - v1[j].1;
                let d = (dx*dx + dy*dy).sqrt();
                if d > 0.0 && d < force_locality as f32 {
                    let f = g * 1.0 / d;
                    fx += f * dx;
                    fy += f * dy;
                }
            }
            v1[i].2 = (v1[i].2 + fx)*force_multiplier;
            v1[i].3 = (v1[i].3 + fy)*force_multiplier;

            if v1[i].0 < x_min {
                v1[i].0 = x_min;
                v1[i].2 *= -1.0;
            }
            if v1[i].0 > x_max {
                v1[i].0 = x_max;
                v1[i].2 *= -1.0;
            }
            if v1[i].1 < y_min {
                v1[i].1 = y_min;
                v1[i].3 *= -1.0;
            }
            if v1[i].1 > y_max {
                v1[i].1 = y_max;
                v1[i].3 *= -1.0;
            }
            v1[i].0 += v1[i].2;
            v1[i].1 += v1[i].3;
        }
    }
    fn apply_forces_2(g: f32, v1: &mut Vec<(f32,f32,f32,f32)>, v2: &Vec<(f32,f32,f32,f32)>,
                      x_min: f32, x_max: f32, y_min: f32, y_max: f32,
                      force_locality: i32, force_multiplier: f32
                      ) 
    {
        for i in 0..v1.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;
            for j in 0..v2.len() {
                let dx = v1[i].0 - v2[j].0;
                let dy = v1[i].1 - v2[j].1;
                let d = (dx*dx + dy*dy).sqrt();
                if d > 0.0 && d < force_locality as f32 {
                    let f = g * 1.0 / d;
                    fx += f * dx;
                    fy += f * dy;
                }
            }
            v1[i].2 = (v1[i].2 + fx)*force_multiplier;
            v1[i].3 = (v1[i].3 + fy)*force_multiplier;

            if v1[i].0 < x_min {
                v1[i].0 = x_min;
                v1[i].2 *= -1.0;
            }
            if v1[i].0 > x_max {
                v1[i].0 = x_max;
                v1[i].2 *= -1.0;
            }
            if v1[i].1 < y_min {
                v1[i].1 = y_min;
                v1[i].3 *= -1.0;
            }
            if v1[i].1 > y_max {
                v1[i].1 = y_max;
                v1[i].3 *= -1.0;
            }
            v1[i].0 += v1[i].2;
            v1[i].1 += v1[i].3;
        }

    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.fc += 1;
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label(format!("yellow <-> yellow"));
            ui.add(egui::Slider::new(&mut self.yty, -1.0..=1.0));

            ui.label(format!("red <-> red"));
            ui.add(egui::Slider::new(&mut self.rtr, -1.0..=1.0));

            ui.label(format!("green <-> green"));
            ui.add(egui::Slider::new(&mut self.gtg, -1.0..=1.0));

            ui.label(format!("blue <-> blue"));
            ui.add(egui::Slider::new(&mut self.btb, -1.0..=1.0));

            ui.label(format!("red -> blue"));
            ui.add(egui::Slider::new(&mut self.rtb, -1.0..=1.0));
            ui.label(format!("green -> blue"));
            ui.add(egui::Slider::new(&mut self.gtb, -1.0..=1.0));
            ui.label(format!("yellow -> blue"));
            ui.add(egui::Slider::new(&mut self.ytb, -1.0..=1.0));

            ui.label(format!("blue -> red"));
            ui.add(egui::Slider::new(&mut self.btr, -1.0..=1.0));
            ui.label(format!("green -> red"));
            ui.add(egui::Slider::new(&mut self.gtr, -1.0..=1.0));
            ui.label(format!("yellow -> red"));
            ui.add(egui::Slider::new(&mut self.ytr, -1.0..=1.0));

            ui.label(format!("blue -> yellow"));
            ui.add(egui::Slider::new(&mut self.bty, -1.0..=1.0));
            ui.label(format!("green -> yellow"));
            ui.add(egui::Slider::new(&mut self.gty, -1.0..=1.0));
            ui.label(format!("red -> yellow"));
            ui.add(egui::Slider::new(&mut self.rty, -1.0..=1.0));

            ui.label(format!("blue -> green"));
            ui.add(egui::Slider::new(&mut self.btg, -1.0..=1.0));
            ui.label(format!("yellow -> green"));
            ui.add(egui::Slider::new(&mut self.ytg, -1.0..=1.0));
            ui.label(format!("red -> green"));
            ui.add(egui::Slider::new(&mut self.rtg, -1.0..=1.0));

            ui.label(format!("force locality"));
            ui.add(egui::Slider::new(&mut self.force_locality, 0..=500));

            ui.label(format!("force multiplier"));
            ui.add(egui::Slider::new(&mut self.force_multiplier, 0.0..=1.0));

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
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.red,
                                              x_min, x_max, y_min, y_max);
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.blue,
                                              x_min, x_max, y_min, y_max);
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.green,
                                              x_min, x_max, y_min, y_max);
                MyEguiApp::distribute_points_(&mut self.rng,&mut self.yellow,
                                              x_min, x_max, y_min, y_max);
            }
            if ctx.input().key_pressed(egui::Key::P) {
                self.paused = !self.paused;
            }
            if self.paused { return; }
            thread::scope(|scope| {
                scope.spawn(|| {
                    MyEguiApp::apply_forces_to_self(self.rtr,
                        &mut self.red,
                        self.x_min, self.x_max,
                        self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier);
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_to_self(self.btb,
                        &mut self.blue,
                        self.x_min, self.x_max,
                        self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier);
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_to_self(self.gtg,
                        &mut self.green,
                        self.x_min, self.x_max,
                        self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier);
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_to_self(self.yty,
                        &mut self.yellow,
                        self.x_min, self.x_max,
                        self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier);
                });
            });
            thread::scope(|scope| {
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.rtb,
                                              &mut self.red,
                                              &self.blue,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.gtb,
                                              &mut self.green,
                                              &self.blue,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.ytb,
                                              &mut self.yellow,
                                              &self.blue,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
            });
            thread::scope(|scope| {
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.rtg,
                                              &mut self.red,
                                              &self.green,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.btg,
                                              &mut self.blue,
                                              &self.green,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.ytg,
                                              &mut self.yellow,
                                              &self.green,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
            });
            thread::scope(|scope| {
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.gty,
                                              &mut self.green,
                                              &self.yellow,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.rty,
                                              &mut self.red,
                                              &self.yellow,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.bty,
                                              &mut self.blue,
                                              &self.yellow,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
            });
            thread::scope(|scope| {
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.btr,
                                              &mut self.blue,
                                              &self.red,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.gtr,
                                              &mut self.green,
                                              &self.red,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
                scope.spawn(|| {
                    MyEguiApp::apply_forces_2(self.ytr,
                                              &mut self.yellow,
                                              &self.red,
                                              self.x_min, self.x_max,
                                              self.y_min, self.y_max,
                        self.force_locality, self.force_multiplier)
                });
            });

            let canvas = egui::Frame::canvas(&ctx.style())
                .fill(egui::Color32::BLACK)
                .paint(egui::Rect{
                    min: egui::Pos2{x: 200.0, y: 0.0},
                    max: egui::Pos2{x: 1000.0, y: 800.0} });

            ui.painter().add(canvas);
            for point in &self.red {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::RED,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            for point in &self.blue {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::BLUE,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            for point in &self.green {
                let circle = egui::epaint::CircleShape{
                    center: egui::Pos2{ x: point.0, y: point.1 },
                    radius: 1.0,
                    fill: egui::Color32::GREEN,
                    stroke: egui::Stroke::none(),
                };
                ui.painter().add(circle);
            }
            for point in &self.yellow {
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

