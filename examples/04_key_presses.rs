extern crate sdl;

use sdl::{Keycode, Keysym};

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() -> sdl::Result<()> {
    let sdl = sdl::init(sdl::INIT_VIDEO)?;

    let window = sdl.window_create(
        "SDL Tutorial",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        sdl::window::Flags::Shown,
    )?;
    let screen_surface = window.get_surface()?;

    let default_surface = sdl.load_bmp("assets/04_key_presses/press.bmp")?;
    let up_surface = sdl.load_bmp("assets/04_key_presses/up.bmp")?;
    let down_surface = sdl.load_bmp("assets/04_key_presses/down.bmp")?;
    let left_surface = sdl.load_bmp("assets/04_key_presses/left.bmp")?;
    let right_surface = sdl.load_bmp("assets/04_key_presses/right.bmp")?;

    let mut current_surface = &default_surface;
    'game: loop {
        while let Some(event) = sdl.poll_event() {
            match event {
                sdl::Event::Quit { .. } => break 'game,
                sdl::Event::KeyDown {
                    keysym: Keysym { sym, .. },
                    ..
                } => {
                    current_surface = match sym {
                        Keycode::Up => &up_surface,
                        Keycode::Down => &down_surface,
                        Keycode::Left => &left_surface,
                        Keycode::Right => &right_surface,
                        _ => &default_surface,
                    }
                }
                _ => (),
            }
        }
        current_surface.blit(None, &screen_surface, None);
        window.update_surface()?;
    }

    Ok(())
}
