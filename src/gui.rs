use crate::recoil::{IS_ACTIVE, RECOIL_STRENGTH};
use crate::operators::{Operator, load_operators, save_operators};
use eframe::egui;
use std::sync::atomic::Ordering;
const OPERATORS_FILE: &str = "operators.json";

pub struct MyApp {
    pub is_active: bool,
    pub operators: Vec<Operator>,
    pub selected_index: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let path = OPERATORS_FILE.to_string();
        let operators = load_operators(&path).unwrap();

        let strength = operators.get(0).map_or(0, |op| op.current_strength as i32);
        RECOIL_STRENGTH.store(strength, Ordering::SeqCst);

        MyApp {
            is_active: false,
            operators,
            selected_index: 0
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Synchronize GUI state with the global IS_ACTIVE flag
        self.is_active = IS_ACTIVE.load(Ordering::SeqCst);

        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(10.0, 10.0); // Increase button padding
        style.spacing.item_spacing = egui::vec2(20.0, 20.0); // Adjust spacing between elements
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::proportional(40.0)), // Larger heading
            (egui::TextStyle::Body, egui::FontId::proportional(30.0)),    // Larger body text
            (egui::TextStyle::Button, egui::FontId::proportional(30.0)), // Define Button text style
        ]
        .into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Recoil Control");

                ui.vertical_centered(|ui| {
                    let button_height = 40.0;
                    let button_width = ui.available_width() / 2.0;

                    // Add a spacer to ensure alignment
                    ui.add_space(1.0);

                    if ui
                        .add_sized([button_width, button_height], egui::Button::new("On"))
                        .clicked()
                    {
                        IS_ACTIVE.store(true, Ordering::SeqCst);
                        println!("Recoil control activated");
                    }

                    if ui
                        .add_sized([button_width, button_height], egui::Button::new("Off"))
                        .clicked()
                    {
                        IS_ACTIVE.store(false, Ordering::SeqCst);
                        println!("Recoil control deactivated");
                    }
                });

                ui.label(format!(
                    "Status: {}",
                    if self.is_active { "On" } else { "Off" }
                ));

                ui.separator();
 
                /*
                    Operator Selection Box
                */
                ui.label("Select Operator:");
                // avoid borrow conflicts by cloning the current operator name
                let current_name = self.operators[self.selected_index].name.clone();
                egui::ComboBox::from_id_salt("operators")
                    .width(ui.available_width())
                    .wrap() // Ensure the dropdown doesn't wrap
                    .height(1000.0) // Increase the height of the dropdown to show more items
                    .selected_text(current_name)
                    .show_ui(ui, |ui| {
                        for (i, op) in self.operators.iter().enumerate() {
                            if ui.selectable_value(&mut self.selected_index, i, &op.name).clicked() {
                                let str_val = op.current_strength as i32;
                                RECOIL_STRENGTH.store(str_val, Ordering::SeqCst);
                            }
                        }
                    });

                ui.separator(); 

                /*
                    Recoil Strength Slider
                */
                let op = &mut self.operators[self.selected_index];
                ui.vertical(|ui| {
                    ui.label(format!("Adjust Recoil Strength: {}", op.current_strength));
                    ui.spacing_mut().slider_width = ui.available_width() - 20.0;
                    if ui.add(egui::Slider::new(&mut op.current_strength, 0..=20)).changed() {
                        let val = op.current_strength as i32;
                        RECOIL_STRENGTH.store(val, Ordering::SeqCst);
                    }
                });

                ui.separator();

                /*
                    Reset & Save Buttons
                */
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        let op = &mut self.operators[self.selected_index];
                        op.reset();
                        RECOIL_STRENGTH.store(op.current_strength as i32, Ordering::SeqCst);
                    }

                    if ui.button("Save").clicked() {
                        save_operators(OPERATORS_FILE, &self.operators).unwrap();
                        println!("Operators saved to {}", OPERATORS_FILE);
                    }
                });

            });
        });

        // Always request repaint to keep UI in sync with keybind toggless
        ctx.request_repaint();
    }
}
