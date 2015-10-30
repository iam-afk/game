extern crate libc;

use std::ffi;

#[link(name = "SDL2")]
extern {
    fn SDL_GetError() -> *mut libc::c_char;
}

pub fn get_error() -> ffi::CString {
    unsafe { ffi::CString::from_raw(SDL_GetError()) }
}

