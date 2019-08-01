use crate::error;
use std::ffi;
use std::ops;
use std::path;

pub struct RWOps {
    pub(crate) ptr: *const RWOpsRec,
}

impl ops::Drop for RWOps {
    fn drop(&mut self) {
        unsafe { SDL_FreeRW(self.ptr) }
    }
}

impl RWOps {
    pub fn from_file<P: AsRef<path::Path>>(file: P) -> crate::Result<RWOps> {
        let file = ffi::CString::new(file.as_ref().to_string_lossy().as_bytes())?;
        let ptr = unsafe { SDL_RWFromFile(file.as_ptr(), ['r' as i8, 'b' as i8, 0].as_ptr()) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(RWOps { ptr })
        }
    }
}

#[repr(C)]
pub(crate) struct RWOpsRec {
    _private: [u8; 0],
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_FreeRW(area: *const RWOpsRec);
    fn SDL_RWFromFile(file: *const libc::c_char, mode: *const libc::c_char) -> *const RWOpsRec;
}
