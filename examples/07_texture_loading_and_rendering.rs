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

    let renderer = window.create_renderer()?;
    renderer.set_draw_color(0xff, 0xff, 0xff, 0xff)?;

    let sdl_img = sdl.image(sdl::image::Init::PNG)?;

    let loaded_surface = sdl_img.load("assets/07_texture_loading_and_rendering/texture.png")?;
    let texture = renderer.create_texture_from_surface(&loaded_surface)?;

    'game: loop {
        while let Some(event) = sdl.poll_event() {
            if let sdl::Event::Quit { .. } = event {
                break 'game;
            }
        }
        renderer.clear()?;
        renderer.copy(&texture, None, None)?;
        renderer.present();
    }

    Ok(())
}
