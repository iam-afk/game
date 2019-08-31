extern crate sdl;

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

    let x_out = sdl.load_bmp("assets/03_event_driven_programming/x.bmp")?;

    'game: loop {
        while let Some(event) = sdl.poll_event() {
            match event {
                sdl::Event::Quit { .. } => break 'game,
                _ => (),
            }
        }
        x_out.blit(None, &screen_surface, None);
        window.update_surface()?;
    }

    Ok(())
}
