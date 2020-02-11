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

    'game: loop {
        while let Some(event) = sdl.poll_event() {
            if let sdl::Event::Quit { .. } = event {
                break 'game;
            }
        }
        renderer.set_draw_color(0xff, 0xff, 0xff, 0xff)?;
        renderer.clear()?;

        let fill_rect = sdl::Rect {
            x: SCREEN_WIDTH / 4,
            y: SCREEN_HEIGHT / 4,
            w: SCREEN_WIDTH / 2,
            h: SCREEN_HEIGHT / 2,
        };
        renderer.set_draw_color(0xff, 0x00, 0x00, 0xff)?;
        renderer.fill_rect(Some(&fill_rect))?;

        let outline_rect = sdl::Rect {
            x: SCREEN_WIDTH / 6,
            y: SCREEN_HEIGHT / 6,
            w: SCREEN_WIDTH * 2 / 3,
            h: SCREEN_HEIGHT * 2 / 3,
        };
        renderer.set_draw_color(0x00, 0xff, 0x00, 0xff)?;
        renderer.draw_rect(Some(&outline_rect))?;

        renderer.set_draw_color(0x00, 0x00, 0xff, 0xff)?;
        renderer.draw_line(0, SCREEN_HEIGHT / 2, SCREEN_WIDTH, SCREEN_HEIGHT / 2)?;

        renderer.set_draw_color(0xff, 0xff, 0x00, 0xff)?;
        for i in (0..SCREEN_HEIGHT).step_by(4) {
            renderer.draw_point(SCREEN_WIDTH / 2, i)?;
        }

        renderer.present();
    }

    Ok(())
}
