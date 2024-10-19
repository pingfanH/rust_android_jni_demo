
#[derive(Default)]
pub struct MyApp {
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