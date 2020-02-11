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
    let screen_surface = window.get_surface()?;

    let hello_world = sdl.load_bmp("assets/02_getting_an_image_on_the_screen/hello_world.bmp")?;

    hello_world.blit(None, &screen_surface, None);
    window.update_surface()?;

    sdl.delay(2000);

    Ok(())
}
