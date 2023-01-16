use eframe::{
    egui,
    epaint::{Color32, Stroke},
};

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Snapis's Calculator",
        native_options,
        Box::new(|_| {
            Box::new(MyApp {
                display_text: "Hello, World!".to_string(),
                num1: None,
                num2: None,
                operation: None
            })
        }),
    );
}

fn division(dividend: f64, divisor: f64) -> f64 {
    // handle divisor = 0

    dividend / divisor
}

fn multiplication(multiplier: f64, multiplicand: f64) -> f64 {
    multiplier * multiplicand
}

fn subtraction(minuend: f64, subtrahend: f64) -> f64 {
    minuend - subtrahend
}

fn addition(addend_1: f64, addend_2: f64) -> f64 {
    addend_1 + addend_2
}

fn to_percent(num: f64) -> f64 {
    num / 100_f64
}

enum ButtonType {
    Button,
    Operation,
    Function,
}

fn button(size: i8, sort: ButtonType, name: String) -> eframe::egui::Button {
    let vec2 = egui::Vec2 {
        x: (f32::from(size) * 100.0),
        y: 100.0,
    };

    egui::Button::new(egui::widget_text::RichText::new(name).size(40.0))
        .min_size(vec2)
        .fill(match sort {
            ButtonType::Button => Color32::from_gray(60),
            ButtonType::Operation => Color32::from_rgb(255, 190, 30),
            ButtonType::Function => Color32::from_gray(50),
        })
}

enum Operation {
    Divide,
    Multiply,
    Minus,
    Plus,
    Blank
}

struct MyApp {
    display_text: String,
    num1: Option<f64>,
    num2: Option<f64>,
    operation: Option<Operation>,
}

impl MyApp {
    fn execute(&mut self, value: u8, operation: Operation) {

        // Take value, store it and display it

        match operation {
            Operation::Blank => todo!(),
            Operation::Divide => todo!(),
            Operation::Multiply => todo!(),
            Operation::Minus => todo!(),
            Operation::Plus => todo!(),
        };


    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_window_size(egui::vec2(415.0, 555.0));

        // Modifies widget visuals
        ctx.set_visuals(egui::style::Visuals {
            widgets: egui::style::Widgets {
                inactive: egui::style::WidgetVisuals {
                    rounding: egui::Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: 0.0,
                        se: 0.0,
                    },
                    bg_fill: Color32::from_gray(60),
                    bg_stroke: Stroke::new(0.5, Color32::from_gray(50)),
                    fg_stroke: Stroke::new(1.0, Color32::BLACK),
                    expansion: 0.0,
                },
                hovered: egui::style::WidgetVisuals {
                    rounding: egui::Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: 0.0,
                        se: 0.0,
                    },
                    bg_fill: Color32::from_gray(60),
                    bg_stroke: Stroke::new(0.5, Color32::from_gray(170)),
                    fg_stroke: Stroke::new(1.0, Color32::BLACK),
                    expansion: 0.0,
                },
                active: egui::style::WidgetVisuals {
                    rounding: egui::Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: 0.0,
                        se: 0.0,
                    },
                    bg_fill: Color32::from_gray(50),
                    bg_stroke: Stroke::new(0.5, Color32::from_gray(50)),
                    fg_stroke: Stroke::new(1.0, Color32::BLACK),
                    expansion: 0.0,
                },
                ..Default::default()
            },
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.display_text)
                        .font(egui::FontId {
                            size: 40.0,
                            ..Default::default()
                        })
                        .interactive(false)
                        .text_color(Color32::WHITE)
                        .desired_width(f32::INFINITY)
                        .margin(egui::vec2(0.0, 0.0)),
                );
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(2, ButtonType::Button, "C".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Function, "%".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Operation, "÷".to_string()))
                    .clicked()
                {
                    println!("test");
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "7".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, "8".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, "9".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Operation, "×".to_string()))
                    .clicked()
                {
                    println!("test");
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "4".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, "5".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, "6".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Operation, "–".to_string()))
                    .clicked()
                {
                    println!("test");
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "1".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, "2".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, "3".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Operation, "+".to_string()))
                    .clicked()
                {
                    println!("test");
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(2, ButtonType::Button, "0".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Button, ",".to_string()))
                    .clicked()
                {
                    println!("test");
                }

                if ui
                    .add(button(1, ButtonType::Operation, "=".to_string()))
                    .clicked()
                {
                    println!("test");
                }
            });
        });
    }
}
