#[repr(i32)]
pub enum Scancode {
    Unknown = 0,

    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
}

const SCANCODE_MASK: i32 = 1 << 30;

macro_rules! scancode_to_keycode {
    ($code:path) => { $code as i32 | SCANCODE_MASK }
}

#[repr(i32)]
pub enum Keycode {
    Unknown = 0,

    Right = scancode_to_keycode!(Scancode::Right),
    Left = scancode_to_keycode!(Scancode::Left),
    Down = scancode_to_keycode!(Scancode::Down),
    Up = scancode_to_keycode!(Scancode::Up),
}

#[repr(C)]
pub struct Keysym {
    pub scancode: Scancode,
    pub sym: Keycode,
    pub modifiers: u16,
    unused: u32,
}
