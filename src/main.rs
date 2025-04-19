mod gui;
mod recoil;

use std::sync::{Arc, atomic::AtomicBool};
use eframe::NativeOptions;

fn main() {
    // Button state via Arc
    let right_button = Arc::new(AtomicBool::new(false));
    let left_button = Arc::new(AtomicBool::new(false));

    // Start recoil control and input listener threads
    recoil::start_recoil_control(Arc::clone(&right_button), Arc::clone(&left_button));
    recoil::start_input_listener(Arc::clone(&right_button), Arc::clone(&left_button));

    // Launch the GUI
    let options = NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Recoil Control GUI",
        options,
        Box::new(|_cc| Ok(Box::new(gui::MyApp::default()))),
    ).expect("Failed to start eframe");
}
