extern crate libc;

use std::ffi;

#[repr(C)]
pub struct Window;

#[link(name = "SDL2")]
extern {
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_CreateWindow(title: *const u8, x: i32, y: i32, w: i32, h: i32, flags: u32) -> *const Window;
    fn SDL_GetError() -> *mut libc::c_char;
    fn SDL_Delay(ms: u32);
    fn SDL_DestroyWindow(window: *const Window);
    fn SDL_Quit();
}

const WINDOWPOS_UNDEFINED_MASK: i32 = 0x1FFF0000;
pub const WINDOWPOS_UNDEFINED: i32 = 0 | WINDOWPOS_UNDEFINED_MASK;

pub enum WindowFlags {
    Fullscreen = 0x00000001,         /**< fullscreen window */
    FullscreenDesktop = WindowFlags::Fullscreen as isize | 0x00001000,
    OpenGL = 0x00000002,             /**< window usable with OpenGL context */
    Shown = 0x00000004,              /**< window is visible */
    Hidden = 0x00000008,             /**< window is not visible */
    Borderless = 0x00000010,         /**< no window decoration */
    Resizable = 0x00000020,          /**< window can be resized */
    Minimized = 0x00000040,          /**< window is minimized */
    Maximized = 0x00000080,          /**< window is maximized */
    InputGrabbed = 0x00000100,       /**< window has grabbed input focus */
    AllowHighDPI = 0x00002000,
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

impl Window {
    pub fn destroy(&self) {
        unsafe { SDL_DestroyWindow(self) }
    }
}

fn get_error() -> ffi::CString {
    unsafe { ffi::CString::from_raw(SDL_GetError()) }
}

pub fn init(flags: u32) {
    match unsafe { SDL_Init(flags) } {
        0 => {},
        _ => panic!("{:?}", get_error())
    }
}

pub fn create_window(title: &str, x: i32, y: i32, w: i32, h: i32, flags: u32) -> &Window {
    let ptr = unsafe { SDL_CreateWindow(title.as_ptr(), x, y, w, h, flags) };
    if ptr.is_null() {
        panic!("{:?}", get_error());
    }
    unsafe { &*ptr }
}

pub fn delay(ms: u32) {
    unsafe { SDL_Delay(ms) }
}

pub fn quit() {
    unsafe { SDL_Quit() }
}
