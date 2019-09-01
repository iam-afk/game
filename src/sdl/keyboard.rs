#[repr(i32)]
pub enum Scancode {
    Unknown = 0,

    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
}

#[repr(i32)]
pub enum Keycode {
    Unknown = 0,

    Right = Scancode::Right as i32 | 1 << 30,
    Left = Scancode::Left as i32 | 1 << 30,
    Down = Scancode::Down as i32 | 1 << 30,
    Up = Scancode::Up as i32 | 1 << 30,
}

#[repr(C)]
pub struct Keysym {
    pub scancode: Scancode,
    pub sym: Keycode,
    pub modifiers: u16,
    unused: u32,
}
