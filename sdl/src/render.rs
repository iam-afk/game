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
}
