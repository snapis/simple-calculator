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
                display_text: "".to_string(),
                num: vec![None, None],
                ops: None,
                auto_clear: false,
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
    Value(char),
    Divide,
    Equal,
}

struct MyApp {
    display_text: String,
    num: Vec<Option<f64>>,
    ops: Option<Operation>,

    // if true, when value entered overwrite num 0
    auto_clear: bool,
}

impl MyApp {
    fn input(&mut self, operation: Operation) {
        match operation {
            Operation::Value(x) => {
                let current_num = if let Option::None = self.ops { 0 } else { 1 };

                if let Option::Some(stored_num) = self.num[current_num] {
                    let mut stored_num = stored_num.to_string();

                    stored_num.push(x);

                    let stored_num: f64 = stored_num.parse().expect("Always a valid integer");

                    self.num[current_num] = Option::Some(stored_num);
                } else {
                    self.num[current_num] =
                        Option::Some(f64::from(x.to_digit(10).expect("It's a integer from 0-9")));
                }

                if self.auto_clear {
                    self.display_text = x.to_string();
                    self.num[current_num] = Option::Some(f64::from(x.to_digit(10).expect("msg")));
                    self.auto_clear = false;
                } else {
                    println!("LOG: {}", self.display_text);
                    self.display_text.push(x);
                }

                println!("num1:{:?} and num2: {:?}", self.num[0], self.num[1]);
                if let Option::None = self.ops {
                    println!("Ops is None");
                } else {
                    println!("Ops is Something");
                }
                println!("current num is {}", current_num);
            }
            Operation::Divide => {
                if let Option::None = self.ops {
                    self.ops = Option::Some(Operation::Divide);
                    self.display_text.push_str(" ÷ ");
                } else {
                    // in case of double operator calling it just resets
                    self.display_text = "".to_string();
                    self.num = vec![None, None];
                    self.ops = None;
                }
            }
            Operation::Equal => {
                if let Some(Operation::Divide) = self.ops {
                    let result = division(self.num[0].unwrap(), self.num[1].unwrap());
                    self.num = vec![Some(result), None];
                    self.display_text = result.to_string();
                    self.ops = None;
                    self.auto_clear = true;
                }
            }
        }
    }

    fn display_text(num1: Option<f64>, num2: Option<f64>, arg: Operation) {
        //match arg {
        //  Operation::Value() => {

        //}
        //}
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
                    self.input(Operation::Divide);
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "7".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('7'));
                }

                if ui
                    .add(button(1, ButtonType::Button, "8".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('8'));
                }

                if ui
                    .add(button(1, ButtonType::Button, "9".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('9'));
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
                    self.input(Operation::Value('4'));
                }

                if ui
                    .add(button(1, ButtonType::Button, "5".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('5'));
                }

                if ui
                    .add(button(1, ButtonType::Button, "6".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('6'));
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
                    self.input(Operation::Value('1'));
                }

                if ui
                    .add(button(1, ButtonType::Button, "2".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('2'));
                }

                if ui
                    .add(button(1, ButtonType::Button, "3".to_string()))
                    .clicked()
                {
                    self.input(Operation::Value('3'));
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
                    self.input(Operation::Value('0'));
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
                    self.input(Operation::Equal);
                }
            });
        });
    }
}
