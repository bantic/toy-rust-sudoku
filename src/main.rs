extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod render;
use render::Drawer;

fn main() {
    let width = 400;
    let height = 400;

    let sdl_context = sdl2::init().unwrap_or_else(|err| {
        println!("Failed to initialize an SDL context: {}", err);
        std::process::exit(1);
    });
    let video_subsystem = sdl_context.video().unwrap_or_else(|err| {
        println!("Failed to initialize the video subsystem: {}", err);
        std::process::exit(1);
    });
    let mut window_builder = video_subsystem.window("sokoban-rs", height, width);
    let window = window_builder.opengl()
        .build()
        .unwrap_or_else(|err| {
            println!("Failed to create the window: {}", err);
            std::process::exit(1);
        });
    let renderer = window.renderer()
        .build()
        .unwrap_or_else(|err| {
            println!("Failed to get an SDL renderer for the main window: {}", err);
            std::process::exit(1);
        });
    let mut drawer = Drawer::new(renderer);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;
    while running {
        drawer.draw();

        match event_pump.wait_event() {
            Event::Quit {..} => {
                running = false;
            }
            Event::MouseButtonDown {x, y, ..} => {
                println!("Mousesdown {},{}",x,y);
            }
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                println!("hit 1!");
            }
            _ => {}
        }
    }
}
