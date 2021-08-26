mod cpu;
mod opcode;
// use sdl2::keyboard::Keycode;

// use cpu::State;

// pub fn main() -> Result<(), String> {
//     // create a new state for the processor
//     let mut state = State::new();
//     // load rom into the processor
//     state.load_rom("roms/IBM.ch8");

//     // load sdl context
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     let window = video_subsystem
//         .window("chip8", 640, 320)
//         .position_centered()
//         .build()
//         .unwrap();

//     let mut event_pump = sdl_context.event_pump().unwrap();

//     'main: loop {
//         let code = state.fetch();
//         state.process(code)?;
//         for row in &state.display {
//             for col in row {
//                 match col > &0 {
//                     true => print!("1"),
//                     false => print!("0"),
//                 }
//             }
//             println!("");
//         }

//         // let stdin = std::io::stdin();
//         // let mut string = String::new();
//         // stdin.read_line(&mut string).ok().expect("fail");

//         std::thread::sleep(std::time::Duration::from_millis(50));
//     }

//     Ok(())
// }

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut x = 0;
    let mut y = 0;
    'running: loop {
        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(sdl2::rect::Rect::new(x, y, 10, 10))?;
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        x += 1;
        y += 1;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    Ok(())
}
