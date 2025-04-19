use eframe::egui;
use enigo::{Enigo, MouseControllable};
use rdev::{EventType, Key, listen};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicI32, Ordering},
};
use std::thread;
use std::time::Duration;

// Configuration constants
const MOVE_INTERVAL: Duration = Duration::from_millis(7);
const IDLE_SLEEP: Duration = Duration::from_millis(10);

// button state managed via Arc<AtomicBool> in main

static IS_ACTIVE: AtomicBool = AtomicBool::new(false);
// Replace the constant with a mutable variable to allow dynamic updates
static RECOIL_STRENGTH: AtomicI32 = AtomicI32::new(2);

fn main() {
    // Button state via Arc
    let right_button = Arc::new(AtomicBool::new(false));
    let left_button = Arc::new(AtomicBool::new(false));

    // Spawn recoil control thread
    let rb_clone = Arc::clone(&right_button);
    let lb_clone = Arc::clone(&left_button);
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if IS_ACTIVE.load(Ordering::SeqCst)
                && rb_clone.load(Ordering::SeqCst)
                && lb_clone.load(Ordering::SeqCst)
            {
                let strength = RECOIL_STRENGTH.load(Ordering::SeqCst);
                let _ = enigo.mouse_move_relative(0, strength);
                thread::sleep(MOVE_INTERVAL);
            } else {
                thread::sleep(IDLE_SLEEP);
            }
        }
    });

    // Spawn input listener thread
    let rb_clone2 = Arc::clone(&right_button);
    let lb_clone2 = Arc::clone(&left_button);
    thread::spawn(move || {
        if let Err(err) = listen(move |event| match event.event_type {
            EventType::ButtonPress(rdev::Button::Right) => rb_clone2.store(true, Ordering::SeqCst),
            EventType::ButtonPress(rdev::Button::Left) => lb_clone2.store(true, Ordering::SeqCst),
            EventType::ButtonRelease(rdev::Button::Right) => {
                rb_clone2.store(false, Ordering::SeqCst)
            }
            EventType::ButtonRelease(rdev::Button::Left) => {
                lb_clone2.store(false, Ordering::SeqCst)
            }
            EventType::KeyRelease(Key::Home) => {
                let curr = IS_ACTIVE.load(Ordering::SeqCst);
                IS_ACTIVE.store(!curr, Ordering::SeqCst);
                println!(
                    "Recoil control {} via Home key",
                    if !curr { "activated" } else { "deactivated" }
                );
            }
            _ => {}
        }) {
            eprintln!("Listener error: {:?}", err);
        }
    });

    // Launch the GUI
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Recoil Control GUI",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
    .expect("Failed to start eframe");
}

#[derive(Default)]
struct MyApp {
    is_active: bool,
    selected_operator: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Synchronize GUI state with the global IS_ACTIVE flag
        self.is_active = IS_ACTIVE.load(Ordering::SeqCst);

        // Adjust the style for larger buttons and text
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
                ui.label("Adjust Recoil Strength: ".to_owned() + &RECOIL_STRENGTH.load(Ordering::SeqCst).to_string());
                let mut recoil_strength = RECOIL_STRENGTH.load(Ordering::SeqCst);
                ui.spacing_mut().slider_width = ui.available_width();
                if ui
                    .add_sized(
                        [ui.available_width(), 20.0],
                        egui::Slider::new(&mut recoil_strength, 0..=20).show_value(true),
                    )
                    .changed()
                {
                    RECOIL_STRENGTH.store(recoil_strength, Ordering::SeqCst);
                    println!("Recoil strength updated to: {}", recoil_strength);
                }
            });
        });

        // Always request repaint to keep UI in sync with keybind toggles
        ctx.request_repaint();
    }
}
