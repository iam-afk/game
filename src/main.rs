extern crate sdl;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    sdl::init(sdl::INIT_VIDEO);
    let window = sdl::window::create("SDL Tutorial",
                                     SCREEN_WIDTH, SCREEN_HEIGHT,
                                     sdl::window::Flags::Shown as u32);

    let surface = window.get_surface();
    let color = surface.map_rgb(0xFFu8, 0xFFu8, 0xFFu8);
    surface.fill_rect(None, color);

    window.update_surface();

    sdl::delay(2000);

    window.destroy();

    sdl::quit();
}
