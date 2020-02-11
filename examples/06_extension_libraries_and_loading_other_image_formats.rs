extern crate sdl;

use sdl::Rect;

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

    let img = sdl.image(sdl::image::Init::Png)?;

    let screen_surface = window.get_surface()?;

    let stretched_surface = img
        .load("assets/06_extension_libraries_and_loading_other_image_formats/loaded.png")?
        .convert(screen_surface.format(), 0)?;

    'game: loop {
        while let Some(event) = sdl.poll_event() {
            if let sdl::Event::Quit { .. } = event {
                break 'game;
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
