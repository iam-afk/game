extern crate sdl;

use sdl::Rect;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() -> sdl::Result<()> {
    let sdl = sdl::init(sdl::Init::VIDEO)?;

    let window = sdl.create_window(
        "SDL Tutorial",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        sdl::window::Flags::SHOWN,
    )?;
    let screen_surface = window.get_surface()?;

    let stretched_surface = sdl
        .load_bmp("assets/05_optimized_surface_loading_and_soft_stretching/stretch.bmp")?
        .convert(screen_surface.format(), 0)?;

    'game: loop {
        while let Some(event) = sdl.poll_event() {
            match event {
                sdl::Event::Quit { .. } => break 'game,
                _ => (),
            }
        }
        let stretched_rect = Rect {
            x: 0,
            y: 0,
            w: SCREEN_WIDTH,
            h: SCREEN_HEIGHT,
        };
        stretched_surface.blit_scaled(None, &screen_surface, Some(&stretched_rect));
        window.update_surface()?;
    }

    Ok(())
}
