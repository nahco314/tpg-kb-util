use multiinput::KeyId;

pub fn key_to_num(key_id: &KeyId) -> Option<u8> {
    let num = match key_id {
        KeyId::Escape => 0,
        KeyId::Backspace => 1,
        KeyId::Left => 2,
        KeyId::Right => 3,
        KeyId::Up => 4,
        KeyId::Down => 5,
        KeyId::Space => 6,
        KeyId::A => 7,
        KeyId::B => 8,
        KeyId::C => 9,
        KeyId::D => 10,
        KeyId::E => 11,
        KeyId::F => 12,
        KeyId::G => 13,
        KeyId::H => 14,
        KeyId::I => 15,
        KeyId::J => 16,
        KeyId::K => 17,
        KeyId::L => 18,
        KeyId::M => 19,
        KeyId::N => 20,
        KeyId::O => 21,
        KeyId::P => 22,
        KeyId::Q => 23,
        KeyId::R => 24,
        KeyId::S => 25,
        KeyId::T => 26,
        KeyId::U => 27,
        KeyId::V => 28,
        KeyId::W => 29,
        KeyId::X => 30,
        KeyId::Y => 31,
        KeyId::Z => 32,
        KeyId::Zero => 33,
        KeyId::One => 34,
        KeyId::Two => 35,
        KeyId::Three => 36,
        KeyId::Four => 37,
        KeyId::Five => 38,
        KeyId::Six => 39,
        KeyId::Seven => 40,
        KeyId::Eight => 41,
        KeyId::Nine => 42,
        KeyId::Shift => 43,
        KeyId::LeftCtrl => 44,
        KeyId::RightCtrl => 45,
        KeyId::LeftAlt => 46,
        KeyId::RightAlt => 47,
        KeyId::ForwardSlash => 48,
        KeyId::Minus => 49,
        KeyId::FullStop => 50,
        KeyId::Comma => 51,
        _ => return None,
    };

    return Some(num);
}
