extern crate sdl;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() -> sdl::Result<()> {
    let sdl = sdl::init(sdl::Init::VIDEO)?;

    sdl.set_hint(sdl::hint::RENDER_SCALE_QUALITY, "1");

    let window = sdl.create_window(
        "SDL Tutorial",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        sdl::window::Flags::SHOWN,
    )?;

    let renderer = window.create_renderer()?;

    renderer.set_draw_color(0xff, 0xff, 0xff, 0xff)?;

    let sdl_img = sdl.image(sdl::image::Init::PNG)?;

    let loaded_surface = sdl_img.load("assets/09_the_viewport/viewport.png")?;
    let texture = renderer.create_texture_from_surface(&loaded_surface)?;

    'game: loop {
        while let Some(event) = sdl.poll_event() {
            if let sdl::Event::Quit { .. } = event {
                break 'game;
            }
        }
        renderer.set_draw_color(0xff, 0xff, 0xff, 0xff)?;
        renderer.clear()?;

        let top_left_viewport = sdl::Rect {
            x: 0,
            y: 0,
            w: SCREEN_WIDTH / 2,
            h: SCREEN_HEIGHT / 2,
        };
        renderer.set_viewport(Some(&top_left_viewport))?;
        renderer.copy(&texture, None, None)?;

        let top_right_viewport = sdl::Rect {
            x: SCREEN_WIDTH / 2,
            y: 0,
            w: SCREEN_WIDTH / 2,
            h: SCREEN_HEIGHT / 2,
        };
        renderer.set_viewport(Some(&top_right_viewport))?;
        renderer.copy(&texture, None, None)?;

        let bottom_viewport = sdl::Rect {
            x: 0,
            y: SCREEN_HEIGHT / 2,
            w: SCREEN_WIDTH,
            h: SCREEN_HEIGHT / 2,
        };
        renderer.set_viewport(Some(&bottom_viewport))?;
        renderer.copy(&texture, None, None)?;

        renderer.present();
    }

    Ok(())
}
