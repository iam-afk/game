#![feature(arbitrary_enum_discriminant)]
mod error;
pub mod event;
pub mod image;
mod keyboard;
mod rwops;
pub mod surface;
pub mod window;

pub use error::SDLError as Error;
pub use event::Event;
pub use keyboard::{Keycode, Keysym};
use std::ops;
use std::path;
pub use surface::Rect;

#[link(name = "SDL2")]
extern "C" {
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_Delay(ms: u32);
    fn SDL_Quit();
}

pub const INIT_TIMER: u32 = 0x0000_0001;
pub const INIT_AUDIO: u32 = 0x0000_0010;
pub const INIT_VIDEO: u32 = 0x0000_0020;
/**< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
pub const INIT_JOYSTICK: u32 = 0x0000_0200;
/**< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
pub const INIT_HAPTIC: u32 = 0x0000_1000;
pub const INIT_GAMECONTROLLER: u32 = 0x0000_2000;
/**< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
pub const INIT_EVENTS: u32 = 0x0000_4000;
pub const INIT_NOPARACHUTE: u32 = 0x0010_0000;
/**< Don't catch fatal signals */
pub const INIT_EVERYTHING: u32 = INIT_TIMER
    | INIT_AUDIO
    | INIT_VIDEO
    | INIT_EVENTS
    | INIT_JOYSTICK
    | INIT_HAPTIC
    | INIT_GAMECONTROLLER;

pub type Result<T> = std::result::Result<T, error::SDLError>;

pub fn init(flags: u32) -> Result<SDL> {
    match unsafe { SDL_Init(flags) } {
        0 => Ok(SDL {}),
        _ => Err(error::SDLError::get()),
    }
}

pub struct SDL {}

impl ops::Drop for SDL {
    fn drop(&mut self) {
        unsafe { SDL_Quit() }
    }
}

impl SDL {
    pub fn window_create(
        &self,
        title: &str,
        w: i32,
        h: i32,
        flags: window::Flags,
    ) -> Result<window::Window> {
        window::Window::create(
            title,
            window::Pos::Undefined as i32,
            window::Pos::Undefined as i32,
            w,
            h,
            flags,
        )
    }

    pub fn img(&self, flag: image::Init) -> crate::Result<image::Img> {
        let flags = flag as libc::c_int;
        if unsafe { image::IMG_Init(flags) } != flags {
            Err(crate::error::SDLError::get())
        } else {
            Ok(image::Img::new(&self))
        }
    }

    pub fn load_bmp<P: AsRef<path::Path>>(&self, file: P) -> crate::Result<surface::Surface> {
        surface::Surface::load_bmp(file)
    }

    pub fn delay(&self, ms: u32) {
        unsafe { SDL_Delay(ms) }
    }

    pub fn poll_event(&self) -> Option<Event> {
        Event::poll()
    }
}
