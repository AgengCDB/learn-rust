use eframe::egui;

struct MyApp {
    counter: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Rust GUI!");
            ui.label(format!("Counter: {}", self.counter));

            if ui.button("Increment").clicked() {
                self.counter += 1;
            }

            if ui.button("Decrement").clicked() {
                self.counter -= 1;
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Simple Counter App", native_options, Box::new(|_| Ok(Box::new(app))))
}
