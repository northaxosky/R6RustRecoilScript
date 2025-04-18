use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use enigo::{Enigo, MouseControllable};
use rdev::{listen, Event, EventType, Key};
use eframe::egui;

static IS_ACTIVE: AtomicBool = AtomicBool::new(false);

fn main() {
    // Start the recoil control logic in a separate thread before launching the GUI
    println!("Starting recoil control thread");
    thread::spawn(recoil_control);
    println!("Recoil control thread started");

    // Launch the GUI
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Recoil Control GUI",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).expect("Failed to start eframe");
}

#[derive(Default)]
struct MyApp {
    is_active: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Synchronize GUI state with the global IS_ACTIVE flag
        self.is_active = IS_ACTIVE.load(Ordering::SeqCst);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Recoil Control");

            if ui.button("On").clicked() {
                IS_ACTIVE.store(true, Ordering::SeqCst);
                println!("Recoil control activated");
            }

            if ui.button("Off").clicked() {
                IS_ACTIVE.store(false, Ordering::SeqCst);
                println!("Recoil control deactivated");
            }

            ui.label(format!("Status: {}", if self.is_active { "On" } else { "Off" }));
        });
    }
}

fn recoil_control() {
    println!("Recoil control function started");

    let recoil_strength = 5; // Adjust this value for recoil strength
    let delay_rate = Duration::from_millis(7); // Delay in milliseconds

    // Use an atomic boolean to track button states
    let right_button_pressed = Arc::new(AtomicBool::new(false));
    let left_button_pressed = Arc::new(AtomicBool::new(false));

    let right_button_clone = Arc::clone(&right_button_pressed);
    let left_button_clone = Arc::clone(&left_button_pressed);

    // Spawn a thread to handle mouse movement
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if IS_ACTIVE.load(Ordering::SeqCst) {
                if right_button_pressed.load(Ordering::SeqCst) && left_button_pressed.load(Ordering::SeqCst) {
                    enigo.mouse_move_relative(0, recoil_strength);
                    thread::sleep(delay_rate);
                } else {
                    thread::sleep(Duration::from_millis(10)); // Prevent busy-waiting
                }
            } else {
                thread::sleep(Duration::from_millis(10)); // Prevent busy-waiting
            }
        }
    });

    // Listen for mouse and key events
    if let Err(error) = listen(move |event: Event| {
        match event.event_type {
            EventType::ButtonPress(button) => {
                if button == rdev::Button::Right {
                    println!("Right mouse button pressed");
                    right_button_clone.store(true, Ordering::SeqCst);
                } else if button == rdev::Button::Left {
                    println!("Left mouse button pressed");
                    left_button_clone.store(true, Ordering::SeqCst);
                }
            }
            EventType::ButtonRelease(button) => {
                if button == rdev::Button::Right {
                    println!("Right mouse button released");
                    right_button_clone.store(false, Ordering::SeqCst);
                } else if button == rdev::Button::Left {
                    println!("Left mouse button released");
                    left_button_clone.store(false, Ordering::SeqCst);
                }
            }
            EventType::KeyPress(key) => {
                if key == Key::CapsLock {
                    let current_state = IS_ACTIVE.load(Ordering::SeqCst);
                    IS_ACTIVE.store(!current_state, Ordering::SeqCst);
                    println!("Recoil control toggled: {}", if !current_state { "On" } else { "Off" });
                }
            }
            _ => {}
        }
    }) {
        eprintln!("Error: {:?}", error);
    }
}
