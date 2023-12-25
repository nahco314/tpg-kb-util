use std::io;
use std::io::Write;

#[derive(Debug)]
enum KBEventType {
    SetKeyboard { device_id: u8 },
    KeyPressed { key_num: u8 },
    KeyReleased { key_num: u8 },
}

#[derive(Debug)]
pub struct KBEvent {
    keyboard_id: u8,
    event_type: KBEventType,
}

impl KBEvent {
    pub fn create_set_keyboard(keyboard_id: u8, device_id: usize) -> KBEvent {
        KBEvent {
            keyboard_id,
            event_type: KBEventType::SetKeyboard {
                device_id: device_id.try_into().unwrap(),
            },
        }
    }

    pub fn create_key_pressed(keyboard_id: u8, key_num: u8) -> KBEvent {
        KBEvent {
            keyboard_id,
            event_type: KBEventType::KeyPressed { key_num },
        }
    }

    pub fn create_key_released(keyboard_id: u8, key_num: u8) -> KBEvent {
        KBEvent {
            keyboard_id,
            event_type: KBEventType::KeyReleased { key_num },
        }
    }

    fn get_id(&self, type_id: u8) -> u8 {
        if self.keyboard_id >= 16 {
            panic!("Keyboard ID must be less than 16");
        }
        if type_id >= 16 {
            panic!("Type ID must be less than 16");
        }

        (self.keyboard_id << 4) | type_id
    }

    pub fn create_message(&self) -> Vec<u8> {
        match &self.event_type {
            KBEventType::SetKeyboard { device_id } => vec![self.get_id(0), *device_id],
            KBEventType::KeyPressed { key_num } => vec![self.get_id(1), *key_num],
            KBEventType::KeyReleased { key_num } => vec![self.get_id(2), *key_num],
        }
    }

    pub fn write(&self) {
        if cfg!(debug_assertions) {
            eprintln!("{:?}", &self);
        }
        io::stdout()
            .write_all(&self.create_message().as_slice())
            .unwrap();
        io::stdout().flush().unwrap();
    }
}
