use eframe::{egui, IconData}; // Imports the egui crate into scope

fn main() {
    //let native_options = eframe::NativeOptions::default();

    /* If i were to add a icon
    let icon = Some(IconData {
        rgba: vec![40,48,42,255,100,48,42,255,40,48,42,255,100,48,42,255],
        width: 2,
        height: 2,
     });
    */

    
    let native_options = eframe::NativeOptions {
        // This i understand

        //icon_data: icon,
        vsync: false,
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "MyEguiApp",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );

}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
        });
    }
}
