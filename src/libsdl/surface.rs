extern crate libc;

use super::error;

#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[repr(C)]
pub struct Palette<'a> {
    ncolors: i32,
    colors: &'a Color,
    version: u32,
    refcount: i32,
}

#[repr(C)]
pub struct PixelFormat<'a> {
    format: u32,
    palette: &'a Palette<'a>,
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
    next: &'a PixelFormat<'a>,
}

#[repr(C)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[repr(C)]
struct BlitMap;

#[repr(C)]
pub struct Surface<'a> {
    flags: u32,
    format: &'a PixelFormat<'a>,
    x: i32,
    y: i32,
    pitch: i32,
    pixels: *const libc::c_void,
    userdata: *const libc::c_void,
    locked: i32,
    lock_data: *const libc::c_void,
    clip_rect: Rect,
    map: &'a BlitMap,
    refcount: i32,
}

#[link(name = "SDL2")]
extern {
    fn SDL_FillRect(dst: *mut Surface, rect: Option<&Rect>, color: u32) -> libc::c_int;
    fn SDL_MapRGB(format: *const PixelFormat, r: u8, g: u8, b: u8) -> u32;
}

impl<'a> Surface<'a> {
    pub fn fill_rect(&mut self, rect: Option<&Rect>, color: u32) {
        match unsafe { SDL_FillRect(self, rect, color) } {
            0 => {},
            _ => panic!("{:?}", error::get_error())
        }
    }

    pub fn map_rgb(&self, r: u8, g: u8, b: u8) -> u32 {
        unsafe { SDL_MapRGB(self.format, r, g, b) }
    }
}


