use crate::error;
use crate::render;
use crate::surface;
use std::ffi;
use std::ops;

const UNDEFINED_MASK: i32 = 0x1FFF_0000;

#[repr(i32)]
pub enum Pos {
    Undefined = UNDEFINED_MASK,
}

bitflags! {
    pub struct Flags: u32 {
        const FULLSCREEN = 0x0000_0001;
        const OPENGL = 0x0000_0002;
        const SHOWN = 0x0000_0004;
        const HIDDEN = 0x0000_0008;
        const BORDERLESS = 0x0000_0010;
        const RESIZABLE = 0x0000_0020;
        const MINIMIZED = 0x0000_0040;
        const MAXIMIZED = 0x0000_0080;
        const INPUT_GRABBED = 0x0000_0100;
        const INPUT_FOCUS = 0x0000_0200;
        const MOUSE_FOCUS = 0x0000_0400;
        const FULLSCREEN_DESKTOP = Self::FULLSCREEN.bits | 0x0000_1000;
        const FOREIGN = 0x0000_0800;
        const ALLOW_HIGH_DPI = 0x0000_2000;
        const MOUSE_CAPTURE = 0x00004000;
        const ALWAYS_ON_TOP = 0x00008000;
        const SKIP_TASKBAR  = 0x00010000;
        const UTILITY       = 0x00020000;
        const TOOLTIP       = 0x00040000;
        const POPUP_MENU    = 0x00080000;
        const VULKAN        = 0x10000000;
    }
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
        let ptr = unsafe { SDL_CreateWindow(title.as_ptr(), x, y, w, h, flags.bits()) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(Window {
                ptr,
                _pd: std::marker::PhantomData,
            })
        }
    }

    pub fn create_renderer(&self) -> crate::Result<render::Renderer> {
        render::Renderer::new(self.ptr, -1, render::Flags::ACCELERATED)
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
pub(crate) struct WindowRec {
    _private: [u8; 0],
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_CreateWindow(
        title: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        flags: u32,
    ) -> *const WindowRec;
    fn SDL_DestroyWindow(window: *const WindowRec);
    fn SDL_GetWindowSurface(window: *const WindowRec) -> *const surface::SurfaceRec;
    fn SDL_UpdateWindowSurface(window: *const WindowRec) -> libc::c_int;
}
