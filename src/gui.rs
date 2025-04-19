// Move GUI-related code to this module
use crate::recoil::{IS_ACTIVE, RECOIL_STRENGTH};
use eframe::egui;
use std::sync::atomic::Ordering;

#[derive(Default)]
pub struct MyApp {
    pub is_active: bool,
    pub selected_operator: String,
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

                ui.separator(); // Add a separator for better UI organization

                ui.label("Select Operator:");
                let operators = ["Ash", "Mira", "Doc"];
                let selected_operator = &mut self.selected_operator;
                egui::ComboBox::from_id_salt("operators")
                    .width(ui.available_width()) // Make the ComboBox fill the row
                    .selected_text(selected_operator.clone())
                    .show_ui(ui, |ui| {
                        for operator in &operators {
                            if ui
                                .selectable_value(
                                    selected_operator,
                                    operator.to_string(),
                                    *operator,
                                )
                                .clicked()
                            {
                                println!("Selected operator: {}", selected_operator);
                            }
                        }
                    });

                ui.separator(); // Add a separator for better UI organization

                // Modify the slider to remove the "Strength" text and make it span the full width of the window
                ui.label(
                    "Adjust Recoil Strength:".to_owned()
                        + &RECOIL_STRENGTH.load(Ordering::SeqCst).to_string(),
                );
                let mut recoil_strength = RECOIL_STRENGTH.load(Ordering::SeqCst);
                ui.spacing_mut().slider_width = ui.available_width();
                if ui
                    .add(egui::Slider::new(&mut recoil_strength, 0..=20).show_value(false))
                    .changed()
                {
                    RECOIL_STRENGTH.store(recoil_strength, Ordering::SeqCst);
                    println!("Recoil strength updated to: {}", recoil_strength);
                }
            });
        });

        // Always request repaint to keep UI in sync with keybind toggless
        ctx.request_repaint();
    }
}
