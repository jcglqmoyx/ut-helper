use std::{thread, time};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, Key, Keyboard, Settings};
use enigo::Direction::Click;
use rdev::{EventType, simulate};

pub fn now() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs() * 1000 + now.subsec_millis() as u64
}

pub fn quit_game() {
    simulate(&EventType::KeyPress(rdev::Key::BackQuote)).expect("");
    thread::sleep(time::Duration::from_millis(300));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.text("\\quit");
    thread::sleep(time::Duration::from_millis(300));
    let _ = enigo.key(Key::Return, Click);
}

pub fn key_press(kit: &mut i32) {
    let mut commands = HashMap::new();
    /*
    commands.insert(Keycode::Key4, "!b m4a on");
    commands.insert(Keycode::Key5, "!b ump45 on");
    commands.insert(Keycode::Key6, "!b mp5k");
    commands.insert(Keycode::Key7, "!b g36");
    commands.insert(Keycode::Key8, "!b sr8");
    commands.insert(Keycode::Key9, "!b hk69");
    commands.insert(Keycode::Key0, "!b spas12");
    commands.insert(Keycode::Insert, "!like");
    commands.insert(Keycode::Delete, "!dislike");
    commands.insert(Keycode::Minus, "!gamble 300000");
    commands.insert(Keycode::Equal, "!gamble 500000");
    commands.insert(Keycode::Home, "!gamble 700000");
    commands.insert(Keycode::End, "!gamble 1000000");
    commands.insert(Keycode::PageUp, "!gamble 1500000");
    commands.insert(Keycode::PageDown, "!gamble 2000000");
     */
    commands.insert(Keycode::Key2, "!mo juliet");
    commands.insert(Keycode::Key3, "!mo urt");
    commands.insert(Keycode::Key4, "!mo friedrich");
    commands.insert(Keycode::Key5, "!mo camel");
    commands.insert(Keycode::Key6, "!pay juliet 2500000");
    commands.insert(Keycode::Key7, "!pay urt 2500000");
    commands.insert(Keycode::Key8, "!pay friedrich 2500000");
    commands.insert(Keycode::Key9, "!pay camel 2500000");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();

    for key in &keys {
        if !commands.contains_key(key) {
            if *key == Keycode::Key2 {
                let _ = enigo.text("t");
                thread::sleep(time::Duration::from_millis(50));
                if (*kit & 1) == 0 {
                    let _ = enigo.text("!kit arm");
                } else {
                    let _ = enigo.text("!kit att");
                }
                *kit += 1;
                thread::sleep(time::Duration::from_millis(50));
                let _ = enigo.key(Key::Return, Click);
                thread::sleep(time::Duration::from_millis(50));
            } else if *key == Keycode::Key3 {
                let _ = enigo.text("t");
                thread::sleep(time::Duration::from_millis(50));
                let _ = enigo.text("!heal");
                thread::sleep(time::Duration::from_millis(50));
                let _ = enigo.key(Key::Return, Click);
            }
        } else {
            let _ = enigo.text("t");
            thread::sleep(time::Duration::from_millis(150));
            let _ = enigo.text(commands.get(key).unwrap());
            thread::sleep(time::Duration::from_millis(150));
            let _ = enigo.key(Key::Return, Click);
            break;
        }
    }
}
pub fn gamble(stake: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.text("t");
    thread::sleep(time::Duration::from_millis(200));
    let text = format!("!gamble {}", stake);
    let _ = enigo.text(&text);
    let _ = enigo.key(Key::Return, Click);
}
