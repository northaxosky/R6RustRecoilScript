use enigo::{Enigo, MouseControllable};
use rdev::{EventType, Key, listen};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicI32, Ordering},
};
use std::thread;
use std::time::Duration;

pub static IS_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static RECOIL_STRENGTH: AtomicI32 = AtomicI32::new(0);

const MOVE_INTERVAL: Duration = Duration::from_millis(7);
const IDLE_SLEEP: Duration = Duration::from_millis(10);

pub fn start_recoil_control(right_button: Arc<AtomicBool>, left_button: Arc<AtomicBool>) {
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
}

pub fn start_input_listener(right_button: Arc<AtomicBool>, left_button: Arc<AtomicBool>) {
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
}
