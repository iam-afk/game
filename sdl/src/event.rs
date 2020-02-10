use crate::keyboard;
use std::mem;

#[repr(u32)]
pub enum Event {
    Quit {
        timestamp: u32,
    } = 0x100,

    KeyDown {
        timestamp: u32,
        window_id: u32,
        state: u8,
        repeat: u8,
        padding2: u8,
        padding3: u8,
        keysym: keyboard::Keysym,
    } = 0x300,

    Padding {
        padding1: [u8; 32],
        padding2: [u8; 20],
    } = 0x0,
}

impl Event {
    pub(crate) fn poll() -> Option<Event> {
        let mut event = mem::MaybeUninit::uninit();
        unsafe {
            if SDL_PollEvent(event.as_mut_ptr()) == 0 {
                None
            } else {
                Some(event.assume_init())
            }
        }
    }
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_PollEvent(event: *mut Event) -> libc::c_int;
}
