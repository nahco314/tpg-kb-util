mod keys;

extern crate evdev;

use evdev::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::kb_event::KBEvent;
use crate::linux::keys::key_to_num;

const USE_CONTINUED_INPUT: bool = false;
const KEYBOARD_CNT: u8 = 2;
const SET_KEYS: [Key; 2] = [Key::KEY_K, Key::KEY_B];

fn detect_set(dev_keys: &HashMap<Key, bool>) -> Option<u8> {
    let pressed_keys: HashSet<&Key> = dev_keys
        .iter()
        .filter(|(_, &v)| v)
        .map(|(k, _)| k)
        .collect();
    let set_keys: HashSet<&Key> = SET_KEYS.iter().collect();

    if !set_keys.is_subset(&pressed_keys) {
        return None;
    }

    let remaining_keys: Vec<&Key> = pressed_keys.difference(&set_keys).map(|&k| k).collect();
    if remaining_keys.len() != 1 {
        return None;
    }

    let remaining_key: &Key = remaining_keys[0];

    let num = match remaining_key {
        &Key::KEY_0 => 0,
        &Key::KEY_1 => 1,
        &Key::KEY_2 => 2,
        &Key::KEY_3 => 3,
        &Key::KEY_4 => 4,
        &Key::KEY_5 => 5,
        &Key::KEY_6 => 6,
        &Key::KEY_7 => 7,
        &Key::KEY_8 => 8,
        &Key::KEY_9 => 9,
        _ => return None,
    };

    return if num >= KEYBOARD_CNT { None } else { Some(num) };
}

fn handle_device(tx: mpsc::Sender<(usize, InputEvent)>, num: usize, mut d: Device) {
    loop {
        for event in d.fetch_events().unwrap() {
            tx.send((num, event)).unwrap();
        }
    }
}

fn listen(tx: mpsc::Sender<(usize, InputEvent)>) {
    let mut threads: HashMap<usize, JoinHandle<_>> = HashMap::new();

    loop {
        threads.retain(|_, h| !h.is_finished());

        for (p, d) in enumerate() {
            let path_str = p.to_str().unwrap();
            let re = Regex::new(r"/dev/input/event(\d+)").unwrap();
            let num_match = re.captures(path_str).unwrap().get(1).unwrap();
            let num: usize = num_match.as_str().parse().unwrap();

            if !threads.contains_key(&num) {
                let new_tx = mpsc::Sender::clone(&tx);
                threads.insert(num, thread::spawn(move || handle_device(new_tx, num, d)));
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

pub fn listen_events(out_tx: mpsc::Sender<KBEvent>) {
    let mut is_on_press: HashMap<usize, HashMap<Key, bool>> = HashMap::new();
    let mut device_to_keyboard: HashMap<usize, u8> = HashMap::new();
    let mut keyboard_to_device: HashMap<u8, usize> = HashMap::new();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        listen(tx);
    });

    for (device_id, event) in rx {
        match (event.kind(), event.value()) {
            (InputEventKind::Key(key), 1) => {
                let dev_keys = is_on_press.entry(device_id).or_insert(HashMap::new());

                dev_keys.insert(key.clone(), true);

                if let Some(keyboard_id) = device_to_keyboard.get(&device_id) {
                    if let Some(key_num) = key_to_num(&key) {
                        let kb_event = KBEvent::create_key_pressed(*keyboard_id, key_num);
                        out_tx.send(kb_event).unwrap();
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
            (InputEventKind::Key(key), 0) => {
                let dev_keys = is_on_press.entry(device_id).or_insert(HashMap::new());
                dev_keys.insert(key.clone(), false);

                if let Some(keyboard_id) = device_to_keyboard.get(&device_id) {
                    if let Some(key_num) = key_to_num(&key) {
                        let kb_event = KBEvent::create_key_released(*keyboard_id, key_num);
                        out_tx.send(kb_event).unwrap();
                    }
                }
            }
            (InputEventKind::Key(key), 2) => {
                let dev_keys = is_on_press.entry(device_id).or_insert(HashMap::new());

                dev_keys.insert(key.clone(), true);

                if let Some(keyboard_id) = device_to_keyboard.get(&device_id) {
                    if USE_CONTINUED_INPUT {
                        if let Some(key_num) = key_to_num(&key) {
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
            _ => (),
        }
    }
}
