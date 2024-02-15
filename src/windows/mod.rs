mod keys;

extern crate multiinput;

use keys::key_to_num;
use multiinput::*;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;

use crate::kb_event::KBEvent;

const USE_CONTINUED_INPUT: bool = false;
const KEYBOARD_CNT: u8 = 2;
const SET_KEYS: [KeyId; 2] = [KeyId::K, KeyId::B];

fn detect_set(dev_keys: &HashMap<KeyId, bool>) -> Option<u8> {
    let pressed_keys: HashSet<&KeyId> = dev_keys
        .iter()
        .filter(|(_, &v)| v)
        .map(|(k, _)| k)
        .collect();
    let set_keys: HashSet<&KeyId> = SET_KEYS.iter().collect();

    if !set_keys.is_subset(&pressed_keys) {
        return None;
    }

    let remaining_keys: Vec<&KeyId> = pressed_keys.difference(&set_keys).map(|&k| k).collect();
    if remaining_keys.len() != 1 {
        return None;
    }

    let remaining_key: &KeyId = remaining_keys[0];

    let num = match remaining_key {
        KeyId::Zero => 0,
        KeyId::One => 1,
        KeyId::Two => 2,
        KeyId::Three => 3,
        KeyId::Four => 4,
        KeyId::Five => 5,
        KeyId::Six => 6,
        KeyId::Seven => 7,
        KeyId::Eight => 8,
        KeyId::Nine => 9,
        _ => return None,
    };

    return if num >= KEYBOARD_CNT { None } else { Some(num) };
}

pub fn  listen_events(out_tx: mpsc::Sender<KBEvent>) {
    let mut is_on_press: HashMap<usize, HashMap<KeyId, bool>> = HashMap::new();
    let mut device_to_keyboard: HashMap<usize, u8> = HashMap::new();
    let mut keyboard_to_device: HashMap<u8, usize> = HashMap::new();

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);

    loop {
        if let Some(event) = manager.get_event() {
            match event {
                RawEvent::KeyboardEvent(device_id, key_id, State::Pressed) => {
                    let dev_keys = is_on_press.entry(device_id).or_insert(HashMap::new());
                    let is_continued = *dev_keys.get(&key_id).unwrap_or(&false);

                    dev_keys.insert(key_id.clone(), true);

                    if let Some(keyboard_id) = device_to_keyboard.get(&device_id) {
                        if (!is_continued) || USE_CONTINUED_INPUT {
                            if let Some(key_num) = key_to_num(&key_id) {
                                let kb_event = KBEvent::create_key_pressed(*keyboard_id, key_num);
                                out_tx.send(kb_event).unwrap();
                            }
                        }
                    }

                    if let Some(keyboard_id) = detect_set(dev_keys) {
                        if let Some(old_device_id) = keyboard_to_device.get(&keyboard_id) {
                            device_to_keyboard.remove(old_device_id);
                        }

                        device_to_keyboard.insert(device_id, keyboard_id);
                        keyboard_to_device.insert(keyboard_id, device_id);
                        let kb_event = KBEvent::create_set_keyboard(keyboard_id, device_id);
                        out_tx.send(kb_event).unwrap();
                    }
                }
                RawEvent::KeyboardEvent(device_id, key_id, State::Released) => {
                    let dev_keys = is_on_press.entry(device_id).or_insert(HashMap::new());
                    dev_keys.insert(key_id.clone(), false);

                    if let Some(keyboard_id) = device_to_keyboard.get(&device_id) {
                        if let Some(key_num) = key_to_num(&key_id) {
                            let kb_event = KBEvent::create_key_released(*keyboard_id, key_num);
                            out_tx.send(kb_event).unwrap();
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
