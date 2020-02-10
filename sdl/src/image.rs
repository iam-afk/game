use crate::surface;
use std::ffi;
use std::ops;
use std::path;

#[repr(C)]
pub enum Init {
    Jpg = 0x1,
    Png = 0x2,
    Tif = 0x4,
    Webp = 0x8,
}

pub struct Img<'a> {
    _pd: std::marker::PhantomData<&'a crate::SDL>,
}

impl<'a> ops::Drop for Img<'a> {
    fn drop(&mut self) {
        unsafe {
            IMG_Quit();
        }
    }
}

impl<'a> Img<'a> {
    pub fn new(_sdl: &crate::SDL) -> Img {
        Img {
            _pd: std::marker::PhantomData,
        }
    }
    pub fn load<P: AsRef<path::Path>>(&self, file: P) -> crate::Result<crate::surface::Surface> {
        let file = ffi::CString::new(file.as_ref().to_string_lossy().as_bytes())?;
        let ptr = unsafe { IMG_Load(file.as_ptr()) };
        if ptr.is_null() {
            Err(crate::error::SDLError::get())
        } else {
            Ok(surface::Surface::new(ptr))
        }
    }
}

#[link(name = "SDL2_image")]
extern "C" {
    pub(crate) fn IMG_Init(flags: libc::c_int) -> libc::c_int;
    fn IMG_Load(file: *const libc::c_char) -> *const surface::SurfaceRec;
    fn IMG_Quit();
}
