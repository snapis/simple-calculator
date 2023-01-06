use eframe::{egui, IconData, Renderer}; // Imports the egui crate into scope

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

        // icon_data: icon,
        // renderer is by defualt Glow
        vsync: false,
        ..Default::default()
    };

    // type AppCreator = Box<dyn FnOnce(&CreationContext<'_>) -> Box<dyn App>>;

    eframe::run_native(
        "MyEguiApp",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
        // First is basically like a name less function (cc) is passed {box::new(MyEguiApp::new(cc))}
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

/*

run_native(app_name: &str, native_options: NativeOptions, app_creator: AppCreator)

native::run::run_glow(app_name, native_options, app_creator);


GlowWinitApp::new(event_loop, app_name, native_options, app_creator);



impl GlowWinitApp {
        fn new(
            event_loop: &EventLoop<UserEvent>,
            app_name: &str,
            native_options: epi::NativeOptions,
            app_creator: epi::AppCreator,
        ) -> Self {
            Self {
                repaint_proxy: Arc::new(egui::mutex::Mutex::new(event_loop.create_proxy())),
                app_name: app_name.to_owned(),
                native_options,
                running: None,
                app_creator: Some(app_creator),
                is_focused: true,
                frame_nr: 0,
            }
        } */
