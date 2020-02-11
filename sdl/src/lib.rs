#![feature(arbitrary_enum_discriminant)]

#[macro_use]
extern crate bitflags;

mod error;
pub mod event;
pub mod image;
mod keyboard;
pub mod rect;
mod render;
mod rwops;
pub mod surface;
pub mod window;

pub use error::SDLError as Error;
pub use event::Event;
pub use keyboard::{Keycode, Keysym};
pub use rect::Rect;
use std::ops;
use std::path;

#[link(name = "SDL2")]
extern "C" {
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_Delay(ms: u32);
    fn SDL_Quit();
}

bitflags! {
    #[repr(C)]
    pub struct Init: u32 {
        const TIMER = 0x0000_0001;
        const AUDIO = 0x0000_0010;
        const VIDEO = 0x0000_0020;
        const JOYSTICK = 0x0000_0200;
        const HAPTIC = 0x0000_1000;
        const GAMECONTROLLER = 0x0000_2000;
        const EVENTS = 0x0000_4000;
        const NOPARACHUTE = 0x0010_0000;
        const EVERYTHING = Self::TIMER.bits
            | Self::AUDIO.bits
            | Self::VIDEO.bits
            | Self::EVENTS.bits
            | Self::JOYSTICK.bits
            | Self::HAPTIC.bits
            | Self::GAMECONTROLLER.bits;
    }
}

pub type Result<T> = std::result::Result<T, error::SDLError>;

pub fn init(flags: Init) -> Result<SDL> {
    match unsafe { SDL_Init(flags.bits()) } {
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
    pub fn create_window(
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

    pub fn image(&self, flags: image::Init) -> crate::Result<image::Img> {
        image::Img::init(self, flags)
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
