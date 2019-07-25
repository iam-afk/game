extern crate libc;

use std::convert;
use std::error;
use std::ffi;
use std::fmt;

#[derive(Debug)]
pub struct SDLError(String);

impl SDLError {
    pub fn get() -> SDLError {
        SDLError(
            unsafe { ffi::CStr::from_ptr(SDL_GetError()) }
                .to_string_lossy()
                .into(),
        )
    }
}

impl fmt::Display for SDLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for SDLError {}

impl convert::From<ffi::NulError> for SDLError {
    fn from(_: ffi::NulError) -> Self {
        SDLError("data provided contains a nul byte".into())
    }
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_GetError() -> *mut libc::c_char;
}
