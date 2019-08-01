use crate::error;
use crate::surface;
use std::ffi;
use std::ops;

const UNDEFINED_MASK: i32 = 0x1FFF0000;

#[repr(i32)]
pub enum Pos {
    Undefined = UNDEFINED_MASK | 0,
}

#[repr(u32)]
pub enum Flags {
    Fullscreen = 0x00000001,
    FullscreenDesktop = Flags::Fullscreen as u32 | 0x00001000,
    OpenGL = 0x00000002,
    Shown = 0x00000004,
    Hidden = 0x00000008,
    Borderless = 0x00000010,
    Resizable = 0x00000020,
    Minimized = 0x00000040,
    Maximized = 0x00000080,
    InputGrabbed = 0x00000100,
    AllowHighDPI = 0x00002000,
}

pub struct Window<'a> {
    ptr: *const WindowRec,
    _pd: std::marker::PhantomData<&'a WindowRec>,
}

impl ops::Drop for Window<'_> {
    fn drop(&mut self) {
        unsafe { SDL_DestroyWindow(self.ptr) }
    }
}

impl<'a> Window<'a> {
    pub fn create(
        title: &str,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: Flags,
    ) -> crate::Result<Window<'a>> {
        let title = ffi::CString::new(title)?;
        let ptr = unsafe { SDL_CreateWindow(title.as_ptr(), x, y, w, h, flags) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(Window {
                ptr,
                _pd: std::marker::PhantomData,
            })
        }
    }

    pub fn get_surface(&self) -> crate::Result<surface::WindowSurface> {
        let ptr = unsafe { SDL_GetWindowSurface(self.ptr) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(surface::WindowSurface::new(ptr))
        }
    }

    pub fn update_surface(&self) -> crate::Result<()> {
        match unsafe { SDL_UpdateWindowSurface(self.ptr) } {
            0 => Ok(()),
            _ => Err(error::SDLError::get()),
        }
    }
}

#[repr(C)]
struct WindowRec {
    _private: [u8; 0],
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_CreateWindow(
        title: *const libc::c_char,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: Flags,
    ) -> *const WindowRec;
    fn SDL_DestroyWindow(window: *const WindowRec);
    fn SDL_GetWindowSurface(window: *const WindowRec) -> *const surface::SurfaceRec;
    fn SDL_UpdateWindowSurface(window: *const WindowRec) -> i32;
}
