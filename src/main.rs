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
                result: None,
            })
        }),
    );
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
}

enum Function {
    Equal,
}

struct MyApp {
    display_text: String,
    num: Vec<Option<f64>>,
    ops: Option<Operation>,
    result: Option<f64>,
}

impl MyApp {
    fn input(&mut self, value: char) {
        let index = if let Option::None = self.ops { 0 } else { 1 }; //Based of the logic that if a operator has been specified, it also means which number we're at

        if let Option::Some(_) = self.result {
            self.num[index] = Some(f64::from(
                value
                    .to_digit(10)
                    .expect("Will always be a integer between 0-9"),
            ));
            self.result = None;
            self.display_text(false);
            return;
        }

        if let Option::Some(saved_num) = self.num[index] {
            // If true, pushes the new number, if not just sets it as equal

            let mut new_number = saved_num.to_string();
            new_number.push(value);
            self.num[index] = Some(new_number.parse().expect("Should always be valid f64"));
        } else {
            self.num[index] = Some(f64::from(
                value
                    .to_digit(10)
                    .expect("Will always be a integer between 0-9"),
            ));
        }

        self.display_text(false);

        println!("First Value: {}", self.num[0].unwrap_or(0.0));
        println!("Second Value: {}", self.num[1].unwrap_or(0.0));
        println!("Result Value: {}", self.result.unwrap_or(0.0));
        if let Option::Some(_) = self.ops {
            println!("There is a op: True");
        } else {
            println!("There is a op: False");
        }
    }

    fn operation(&mut self, operation: Operation) {
        if let Option::None = self.ops {
            self.ops = Some(operation);
            self.display_text(false);
        } else {
            
            self.display_text = "Err".to_string();
            self.num = vec![None, None];
            self.ops = None;
            self.result = None;

        }
    }

    fn function(&mut self, function: Function) {
        match function {
            Function::Equal => {
                let num1 = self.num[0].unwrap_or_else(|| {
                    todo!("Handle Error, missing num 1");
                });

                let num2 = self.num[1].unwrap_or_else(|| {
                    todo!("Handle Error, missing num 2");
                });

                match self.ops.as_mut().unwrap() {
                    Operation::Divide => {
                        if num2 == 0.0 {
                            todo!("Make code display Err, maybe...")
                        }

                        self.result = Some(num1 / num2);
                    }

                    Operation::Multiply => {
                        self.result = Some(num1 * num2);
                    }

                    Operation::Minus => {
                        self.result = Some(num1 - num2);
                    }

                    Operation::Plus => {
                        self.result = Some(num1 - num2);
                    }
                }

                self.display_text(true);
                self.num[0] = Some(self.result.unwrap());
                self.num[1] = None;
                self.ops = None;
            }
        }
    }

    fn display_text(&mut self, result: bool) {
        if result {
            self.display_text = self.result.unwrap().to_string();
        } else {
            let mut string_buf = "".to_string();

            if let Option::Some(num) = self.num[0] {
                string_buf.push_str(&num.to_string());
            }

            if let Option::Some(ops) = &self.ops {
                match ops {
                    Operation::Divide => string_buf.push_str(" ÷ "),
                    Operation::Multiply => string_buf.push_str(" × "),
                    Operation::Minus => string_buf.push_str(" – "),
                    Operation::Plus => string_buf.push_str(" + "),
                }
            }

            if let Option::Some(num) = self.num[1] {
                string_buf.push_str(&num.to_string());
            }

            self.display_text = string_buf;
        }
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
                {}

                if ui
                    .add(button(1, ButtonType::Function, "%".to_string()))
                    .clicked()
                {}

                if ui
                    .add(button(1, ButtonType::Operation, "÷".to_string()))
                    .clicked()
                {
                    self.operation(Operation::Divide);
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "7".to_string()))
                    .clicked()
                {
                    self.input('7');
                }

                if ui
                    .add(button(1, ButtonType::Button, "8".to_string()))
                    .clicked()
                {
                    self.input('8');
                }

                if ui
                    .add(button(1, ButtonType::Button, "9".to_string()))
                    .clicked()
                {
                    self.input('9');
                }

                if ui
                    .add(button(1, ButtonType::Operation, "×".to_string()))
                    .clicked()
                {
                    self.operation(Operation::Multiply);
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "4".to_string()))
                    .clicked()
                {
                    self.input('4');
                }

                if ui
                    .add(button(1, ButtonType::Button, "5".to_string()))
                    .clicked()
                {
                    self.input('5');
                }

                if ui
                    .add(button(1, ButtonType::Button, "6".to_string()))
                    .clicked()
                {
                    self.input('6');
                }

                if ui
                    .add(button(1, ButtonType::Operation, "–".to_string()))
                    .clicked()
                {
                    self.operation(Operation::Minus);
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(1, ButtonType::Button, "1".to_string()))
                    .clicked()
                {
                    self.input('1');
                }

                if ui
                    .add(button(1, ButtonType::Button, "2".to_string()))
                    .clicked()
                {
                    self.input('2');
                }

                if ui
                    .add(button(1, ButtonType::Button, "3".to_string()))
                    .clicked()
                {
                    self.input('3');
                }

                if ui
                    .add(button(1, ButtonType::Operation, "+".to_string()))
                    .clicked()
                {
                    self.operation(Operation::Plus);
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add(button(2, ButtonType::Button, "0".to_string()))
                    .clicked()
                {
                    self.input('0');
                }

                if ui
                    .add(button(1, ButtonType::Button, ",".to_string()))
                    .clicked()
                {}

                if ui
                    .add(button(1, ButtonType::Operation, "=".to_string()))
                    .clicked()
                {
                    self.function(Function::Equal);
                }
            });
        });
    }
}
