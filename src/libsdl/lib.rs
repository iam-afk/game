mod error;
pub mod surface;
pub mod window;

#[link(name = "SDL2")]
extern {
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_Delay(ms: u32);
    fn SDL_Quit();
}

pub const INIT_TIMER          : u32 = 0x00000001;
pub const INIT_AUDIO          : u32 = 0x00000010;
pub const INIT_VIDEO          : u32 = 0x00000020;  /**< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
pub const INIT_JOYSTICK       : u32 = 0x00000200;  /**< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
pub const INIT_HAPTIC         : u32 = 0x00001000;
pub const INIT_GAMECONTROLLER : u32 = 0x00002000;  /**< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
pub const INIT_EVENTS         : u32 = 0x00004000;
pub const INIT_NOPARACHUTE    : u32 = 0x00100000;  /**< Don't catch fatal signals */
pub const INIT_EVERYTHING     : u32 =
                INIT_TIMER | INIT_AUDIO | INIT_VIDEO | INIT_EVENTS |
                INIT_JOYSTICK | INIT_HAPTIC | INIT_GAMECONTROLLER;

pub fn init(flags: u32) {
    match unsafe { SDL_Init(flags) } {
        0 => {},
        _ => panic!("{:?}", error::get_error())
    }
}

pub fn delay(ms: u32) {
    unsafe { SDL_Delay(ms) }
}

pub fn quit() {
    unsafe { SDL_Quit() }
}
