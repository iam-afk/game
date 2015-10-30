extern crate sdl;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    sdl::init(sdl::INIT_VIDEO);
    let window = sdl::create_window("SDL Tutorial",
                                    sdl::WINDOWPOS_UNDEFINED, sdl::WINDOWPOS_UNDEFINED,
                                    SCREEN_WIDTH, SCREEN_HEIGHT,
                                    sdl::WindowFlags::Shown as u32);

    sdl::delay(2000);

    window.destroy();

    sdl::quit();
}
