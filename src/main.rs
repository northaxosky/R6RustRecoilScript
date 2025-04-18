use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use enigo::{Enigo, MouseControllable};
use rdev::{listen, EventType, Key};
use eframe::egui;

// Configuration constants
const RECOIL_STRENGTH: i32 = 5;
const MOVE_INTERVAL: Duration = Duration::from_millis(7);
const IDLE_SLEEP: Duration = Duration::from_millis(10);

// button state managed via Arc<AtomicBool> in main

static IS_ACTIVE: AtomicBool = AtomicBool::new(false);

fn main() {
    // Button state via Arc
    let right_button = Arc::new(AtomicBool::new(false));
    let left_button = Arc::new(AtomicBool::new(false));

    // Spawn movement thread
    let rb_clone = Arc::clone(&right_button);
    let lb_clone = Arc::clone(&left_button);
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if IS_ACTIVE.load(Ordering::SeqCst)
                && rb_clone.load(Ordering::SeqCst)
                && lb_clone.load(Ordering::SeqCst)
            {
                enigo.mouse_move_relative(0, RECOIL_STRENGTH);
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
        if let Err(err) = listen(move |event| {
            match event.event_type {
                EventType::ButtonPress(rdev::Button::Right) => rb_clone2.store(true, Ordering::SeqCst),
                EventType::ButtonPress(rdev::Button::Left) => lb_clone2.store(true, Ordering::SeqCst),
                EventType::ButtonRelease(rdev::Button::Right) => rb_clone2.store(false, Ordering::SeqCst),
                EventType::ButtonRelease(rdev::Button::Left) => lb_clone2.store(false, Ordering::SeqCst),
                EventType::KeyRelease(Key::Home) => {
                    let curr = IS_ACTIVE.load(Ordering::SeqCst);
                    IS_ACTIVE.store(!curr, Ordering::SeqCst);
                    println!("Recoil control {} via Home key", if !curr { "activated" } else { "deactivated" });
                }
                _ => {},
            }
        }) {
            eprintln!("Listener error: {:?}", err);
        }
    });

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

        // Always request repaint to keep UI in sync with keybind toggles
        ctx.request_repaint();
    }
}
