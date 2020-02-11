use std::ffi;

pub const RENDER_SCALE_QUALITY: &str = "SDL_RENDER_SCALE_QUALITY";

pub(crate) fn set(name: &str, value: &str) -> bool {
    let name = ffi::CString::new(name).unwrap();
    let value = ffi::CString::new(value).unwrap();
    let result = unsafe { SDL_SetHint(name.as_ptr(), value.as_ptr()) };
    result == 1
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_SetHint(name: *const libc::c_char, value: *const libc::c_char) -> libc::c_int;
}
