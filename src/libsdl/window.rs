use super::error;

#[repr(C)]
pub struct Window;

#[link(name = "SDL2")]
extern {
    fn SDL_CreateWindow(title: *const u8, x: i32, y: i32, w: i32, h: i32, flags: u32)
        -> *const Window;
    fn SDL_DestroyWindow(window: *const Window);
}

const UNDEFINED_MASK: isize = 0x1FFF0000;

pub enum Pos {
    Undefined = UNDEFINED_MASK | 0
}

pub enum Flags {
    Fullscreen = 0x00000001,         /**< fullscreen window */
    FullscreenDesktop = Flags::Fullscreen as isize | 0x00001000,
    OpenGL = 0x00000002,             /**< window usable with OpenGL context */
    Shown = 0x00000004,              /**< window is visible */
    Hidden = 0x00000008,             /**< window is not visible */
    Borderless = 0x00000010,         /**< no window decoration */
    Resizable = 0x00000020,          /**< window can be resized */
    Minimized = 0x00000040,          /**< window is minimized */
    Maximized = 0x00000080,          /**< window is maximized */
    InputGrabbed = 0x00000100,       /**< window has grabbed input focus */
    AllowHighDPI = 0x00002000,
}

impl Window {

    fn create(title: &str, x: i32, y: i32, w: i32, h: i32, flags: u32) -> &Window {
        let ptr = unsafe { SDL_CreateWindow(title.as_ptr(), x, y, w, h, flags) };
        if ptr.is_null() {
            panic!("{:?}", error::get_error());
        }
        unsafe { &*ptr }
    }

    pub fn destroy(&self) {
        unsafe { SDL_DestroyWindow(self) }
    }

}

pub fn create(title: &str, w: i32, h: i32, flags: u32) -> &Window {
    Window::create(title, Pos::Undefined as i32, Pos::Undefined as i32, w, h, flags)
}

