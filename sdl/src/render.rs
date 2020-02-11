use crate::error;
use crate::surface;
use crate::window;
use crate::Rect;
use std::ops;

bitflags! {
    pub struct Flags: u32 {
        const SOFTWARE = 0x1;
        const ACCELERATED = 0x2;
        const PRESENTVSYNC = 0x4;
        const TARGETTEXTURE = 0x8;
    }
}

#[repr(C)]
struct RendererRec {
    _private: [u8; 0],
}

pub struct Renderer<'a> {
    ptr: *const RendererRec,
    pd: std::marker::PhantomData<&'a RendererRec>,
}

impl ops::Drop for Renderer<'_> {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyRenderer(self.ptr);
        }
    }
}

impl Renderer<'_> {
    pub(crate) fn new(
        window: *const window::WindowRec,
        index: i32,
        flags: Flags,
    ) -> crate::Result<Self> {
        let ptr = unsafe { SDL_CreateRenderer(window, index, flags.bits()) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(Renderer {
                ptr,
                pd: std::marker::PhantomData,
            })
        }
    }

    pub fn set_draw_color(&self, r: u8, g: u8, b: u8, a: u8) -> crate::Result<()> {
        let result = unsafe { SDL_SetRenderDrawColor(self.ptr, r, g, b, a) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn create_texture_from_surface(
        &self,
        surface: &surface::Surface,
    ) -> crate::Result<Texture> {
        let ptr = unsafe { SDL_CreateTextureFromSurface(self.ptr, surface.ptr) };
        if ptr.is_null() {
            Err(error::SDLError::get())
        } else {
            Ok(Texture {
                ptr,
                pd: std::marker::PhantomData,
            })
        }
    }

    pub fn clear(&self) -> crate::Result<()> {
        let result = unsafe { SDL_RenderClear(self.ptr) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn copy(
        &self,
        texture: &Texture,
        src_rect: Option<&Rect>,
        dst_rect: Option<&Rect>,
    ) -> crate::Result<()> {
        let result = unsafe { SDL_RenderCopy(self.ptr, texture.ptr, src_rect, dst_rect) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn present(&self) {
        unsafe {
            SDL_RenderPresent(self.ptr);
        }
    }

    pub fn fill_rect(&self, rect: Option<&Rect>) -> crate::Result<()> {
        let result = unsafe { SDL_RenderFillRect(self.ptr, rect) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn draw_rect(&self, rect: Option<&Rect>) -> crate::Result<()> {
        let result = unsafe { SDL_RenderDrawRect(self.ptr, rect) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> crate::Result<()> {
        let result = unsafe { SDL_RenderDrawLine(self.ptr, x1, y1, x2, y2) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn draw_point(&self, x: i32, y: i32) -> crate::Result<()> {
        let result = unsafe { SDL_RenderDrawPoint(self.ptr, x, y) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }

    pub fn set_viewport(&self, rect: Option<&Rect>) -> crate::Result<()> {
        let result = unsafe { SDL_RenderSetViewport(self.ptr, rect) };
        if result != 0 {
            Err(error::SDLError::get())
        } else {
            Ok(())
        }
    }
}

#[repr(C)]
struct TextureRec {
    _private: [u8; 0],
}

pub struct Texture<'a> {
    ptr: *const TextureRec,
    pd: std::marker::PhantomData<&'a TextureRec>,
}

impl ops::Drop for Texture<'_> {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.ptr);
        }
    }
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_CreateRenderer(
        window: *const window::WindowRec,
        index: libc::c_int,
        flags: u32,
    ) -> *const RendererRec;
    fn SDL_DestroyRenderer(renderer: *const RendererRec);
    fn SDL_SetRenderDrawColor(renderer: *const RendererRec, r: u8, g: u8, b: u8, a: u8) -> i32;
    fn SDL_CreateTextureFromSurface(
        renderer: *const RendererRec,
        surface: *const surface::SurfaceRec,
    ) -> *const TextureRec;
    fn SDL_DestroyTexture(texture: *const TextureRec);
    fn SDL_RenderClear(renderer: *const RendererRec) -> libc::c_int;
    fn SDL_RenderCopy(
        renderer: *const RendererRec,
        texture: *const TextureRec,
        src_rect: Option<&Rect>,
        dst_rect: Option<&Rect>,
    ) -> libc::c_int;
    fn SDL_RenderPresent(renderer: *const RendererRec);
    fn SDL_RenderFillRect(renderer: *const RendererRec, rect: Option<&Rect>) -> libc::c_int;
    fn SDL_RenderDrawRect(renderer: *const RendererRec, rect: Option<&Rect>) -> libc::c_int;
    fn SDL_RenderDrawLine(
        renderer: *const RendererRec,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
    ) -> libc::c_int;
    fn SDL_RenderDrawPoint(
        renderer: *const RendererRec,
        x: libc::c_int,
        y: libc::c_int,
    ) -> libc::c_int;
    fn SDL_RenderSetViewport(renderer: *const RendererRec, rect: Option<&Rect>) -> libc::c_int;
}
