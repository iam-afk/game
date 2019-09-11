#[repr(i32)]
pub enum Scancode {
    Unknown = 0,

    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
}

impl Scancode {
    const MASK: i32 = 1 << 30;
    const fn to_keycode(self) -> i32 {
        self as i32 | Self::MASK
    }
}

#[repr(i32)]
pub enum Keycode {
    Unknown = 0,

    Right = Scancode::Right.to_keycode(),
    Left = Scancode::Left.to_keycode(),
    Down = Scancode::Down.to_keycode(),
    Up = Scancode::Up.to_keycode(),
}

#[repr(C)]
pub struct Keysym {
    pub scancode: Scancode,
    pub sym: Keycode,
    pub modifiers: u16,
    unused: u32,
}
