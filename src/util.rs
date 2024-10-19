use std::collections::HashMap;
use std::{thread, time};

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::Direction::Click;
use enigo::{Enigo, Key, Keyboard, Settings};


pub fn key_press() {
    let mut commands = HashMap::new();
    commands.insert(Keycode::Key9, "!b g36");
    commands.insert(Keycode::Key0, "!b hk69");
    commands.insert(Keycode::Home, "!b sil on");
    commands.insert(Keycode::End, "!kit rifle");
    commands.insert(Keycode::PageUp, "!b ump on");
    commands.insert(Keycode::PageDown, "!gamble 100000");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();

    for key in &keys {
        if !commands.contains_key(key) {
            if *key == Keycode::Key2 {
                let _ = enigo.text("t");
                thread::sleep(time::Duration::from_millis(50));
                let _ = enigo.text("!kit arm");
                thread::sleep(time::Duration::from_millis(50));
                let _ = enigo.key(Key::Return, Click);
                thread::sleep(time::Duration::from_millis(200));
                let _ = enigo.text("t");
                thread::sleep(time::Duration::from_millis(200));
                let _ = enigo.text("!kit att");
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
