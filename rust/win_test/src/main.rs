use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui demo",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}

#[derive(Default)]
struct MyApp {
    label: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, egui!");
            ui.text_edit_singleline(&mut self.label);
            if ui.button("Click me").clicked() {
                println!("Button clicked!");
            }
        });
    }
}