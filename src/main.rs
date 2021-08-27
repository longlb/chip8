mod cpu;
mod display;
mod opcode;

use cpu::State;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    // load sdl
    let sdl_context = sdl2::init().unwrap();
    // create a new state for the processor
    let mut state = State::new(&sdl_context, 10);
    // load rom into the processor
    state.load_rom("roms/IBM.ch8");

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        let code = state.fetch();
        state.process(code)?;

        // let stdin = std::io::stdin();
        // let mut string = String::new();
        // stdin.read_line(&mut string).ok().expect("fail");

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        // std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}
