extern crate libc;

use crate::error;
use crate::rwops;
use crate::Rect;
use std::ops;
use std::path;

pub struct Surface {
    pub(crate) ptr: *const SurfaceRec,
}

impl ops::Drop for Surface {
    fn drop(&mut self) {
        unsafe { SDL_FreeSurface(self.ptr) }
    }
}

impl Surface {
    pub fn load_bmp<P: AsRef<path::Path>>(file: P) -> crate::Result<Surface> {
        let src = rwops::RWOps::from_file(file)?;
        let ptr = unsafe { SDL_LoadBMP_RW(src.ptr, 0) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(Surface { ptr })
        }
    }
    pub(crate) fn new(ptr: *const SurfaceRec) -> Surface {
        Surface { ptr }
    }

    pub fn format(&self) -> &PixelFormat {
        unsafe { &*((*self.ptr).format) }
    }

    pub fn fill_rect(&self, rect: Option<&Rect>, color: u32) -> crate::Result<()> {
        match unsafe { SDL_FillRect(self.ptr, rect, color) } {
            0 => Ok(()),
            _ => Err(error::SDLError::get()),
        }
    }

    pub fn map_rgb(&self, r: u8, g: u8, b: u8) -> u32 {
        unsafe { SDL_MapRGB((*self.ptr).format, r, g, b) }
    }

    pub fn convert(self, fmt: &PixelFormat, flags: u32) -> crate::Result<Surface> {
        let ptr = unsafe { SDL_ConvertSurface(self.ptr, fmt, flags) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(Surface { ptr })
        }
    }

    pub fn blit(&self, src_rect: Option<&Rect>, dst: &Surface, dst_rect: Option<&Rect>) -> bool {
        let result = unsafe { SDL_UpperBlit(self.ptr, src_rect, dst.ptr, dst_rect) };
        result == 0
    }

    pub fn blit_scaled(
        &self,
        src_rect: Option<&Rect>,
        dst: &Surface,
        dst_rect: Option<&Rect>,
    ) -> bool {
        let result = unsafe { SDL_UpperBlitScaled(self.ptr, src_rect, dst.ptr, dst_rect) };
        result == 0
    }
}

pub struct WindowSurface<'a> {
    _ptr: *const SurfaceRec,
    _pd: std::marker::PhantomData<&'a SurfaceRec>,
}

impl<'a> ops::Deref for WindowSurface<'a> {
    type Target = Surface;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const WindowSurface<'a> as *const Surface) }
    }
}

impl<'a> WindowSurface<'a> {
    pub(crate) fn new(ptr: *const SurfaceRec) -> WindowSurface<'a> {
        WindowSurface {
            _ptr: ptr,
            _pd: std::marker::PhantomData,
        }
    }
}

#[repr(C)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[repr(C)]
struct Palette {
    ncolors: i32,
    colors: *const Color,
    version: u32,
    refcount: i32,
}

#[repr(C)]
pub struct PixelFormat {
    format: u32,
    palette: *const Palette,
    bits_per_pixel: u8,
    bytes_per_pixel: u8,
    padding1: u8,
    padding2: u8,
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
    r_loss: u8,
    g_loss: u8,
    b_loss: u8,
    a_loss: u8,
    r_shift: u8,
    g_shift: u8,
    b_shift: u8,
    a_shift: u8,
    refcount: i32,
    next: *const PixelFormat,
}

#[repr(C)]
struct BlitMap {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct SurfaceRec {
    flags: u32,
    format: *const PixelFormat,
    x: i32,
    y: i32,
    pitch: i32,
    pixels: *const libc::c_void,
    userdata: *const libc::c_void,
    locked: i32,
    lock_data: *const libc::c_void,
    clip_rect: Rect,
    map: *const BlitMap,
    refcount: i32,
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_ConvertSurface(
        src: *const SurfaceRec,
        fmt: *const PixelFormat,
        flags: u32,
    ) -> *const SurfaceRec;
    fn SDL_FillRect(dst: *const SurfaceRec, rect: Option<&Rect>, color: u32) -> libc::c_int;
    fn SDL_FreeSurface(surface: *const SurfaceRec);
    fn SDL_MapRGB(format: *const PixelFormat, r: u8, g: u8, b: u8) -> u32;
    fn SDL_LoadBMP_RW(src: *const rwops::RWOpsRec, free_src: i32) -> *const SurfaceRec;
    fn SDL_UpperBlit(
        src: *const SurfaceRec,
        src_rect: Option<&Rect>,
        dst: *const SurfaceRec,
        dst_rect: Option<&Rect>,
    ) -> i32;
    fn SDL_UpperBlitScaled(
        src: *const SurfaceRec,
        src_rect: Option<&Rect>,
        dst: *const SurfaceRec,
        dst_rect: Option<&Rect>,
    ) -> i32;
}
