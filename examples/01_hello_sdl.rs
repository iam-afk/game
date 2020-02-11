extern crate sdl;

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

    let surface = window.get_surface()?;
    let color = surface.map_rgb(0xFFu8, 0xFFu8, 0xFFu8);
    surface.fill_rect(None, color)?;

    window.update_surface()?;

    sdl.delay(2000);

    Ok(())
}
