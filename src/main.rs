use eframe::egui::{self, Color32, RichText};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_cc| {
            Box::new(MyEguiApp {
                display_text: "test text".to_string(),
            })
        }),
    );
}

#[derive(Default)]
struct MyEguiApp {
    display_text: String,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_window_size(egui::vec2(410.0, 550.0));

        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size_square = egui::Vec2 { x: 100.0, y: 100.0 };

            let button_size_rectangle = egui::Vec2 { x: 200.0, y: 100.0 };

            let operation_color = Color32::from_rgb(255, 190, 30);

            ui.spacing_mut().item_spacing = egui::vec2(0.1, 0.1);

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.display_text)
                        .font(egui::FontId {
                            size: 40.0,
                            ..Default::default()
                        })
                        .interactive(false)
                        .desired_width(f32::INFINITY)
                        .margin(egui::vec2(0.0, 0.0)),
                );
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Button::new(RichText::new("C").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_rectangle),
                );
                ui.add(
                    egui::Button::new(RichText::new("%").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("÷").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square)
                        .fill(operation_color),
                );
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Button::new(RichText::new("7").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("8").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("9").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("×").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square)
                        .fill(operation_color),
                );
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Button::new(RichText::new("4").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("5").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("6").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("–").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square)
                        .fill(operation_color),
                );
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Button::new(RichText::new("1").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("2").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("3").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("+").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square)
                        .fill(operation_color),
                );
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Button::new(RichText::new("0").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_rectangle),
                );
                ui.add(
                    egui::Button::new(RichText::new(",").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square),
                );
                ui.add(
                    egui::Button::new(RichText::new("=").color(Color32::BLACK).size(40.0))
                        .min_size(button_size_square)
                        .fill(operation_color),
                );
            });
        });
    }
}
