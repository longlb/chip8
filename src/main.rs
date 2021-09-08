mod audio;
mod cpu;
mod display;
mod opcode;

use audio::Audio;
use cpu::State;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub fn main() -> Result<(), String> {
    // load sdl
    let sdl_context = sdl2::init().unwrap();
    // create a new state for the processor with a display
    let mut state = State::new(&sdl_context, 10);
    // load rom into the processor
    state.load_rom("roms/test_opcode.ch8");
    // init audio loader
    let device = Audio::new(&sdl_context);

    // init eventpump to track events like keypresses
    let mut event_pump = sdl_context.event_pump().unwrap();
    // main game loop, press Esc button or close window in top right to exit
    'main: loop {
        // debugging

        // check for keypresses
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown { scancode, .. } => match scancodes(scancode) {
                    Some(x) => state.key_moved(x, true),
                    None => {}
                },
                Event::KeyUp { scancode, .. } => match scancodes(scancode) {
                    Some(x) => state.key_moved(x, false),
                    None => {}
                },
                _ => {}
            }
        }

        // retrieve the instruction at PC and increment
        let code = state.fetch();
        // debug_print(&state, &code);
        // process the opcode received at PC
        state.process(code)?;

        match state.sound > 0 {
            true => device.resume(),
            false => device.pause(),
        }
        state.decrement();
        // 1000000 microseconds / 600 = 1667 micros, so ~600 instrs per second
        std::thread::sleep(std::time::Duration::from_micros(1667));
    }
    Ok(())
}

fn scancodes(sc: Option<Scancode>) -> Option<u8> {
    match sc {
        Some(Scancode::Num1) => Some(0x1),
        Some(Scancode::Num2) => Some(0x2),
        Some(Scancode::Num3) => Some(0x3),
        Some(Scancode::Num4) => Some(0xC),
        Some(Scancode::Q) => Some(0x4),
        Some(Scancode::W) => Some(0x5),
        Some(Scancode::E) => Some(0x6),
        Some(Scancode::R) => Some(0xD),
        Some(Scancode::A) => Some(0x7),
        Some(Scancode::S) => Some(0x8),
        Some(Scancode::D) => Some(0x9),
        Some(Scancode::F) => Some(0xE),
        Some(Scancode::Z) => Some(0xA),
        Some(Scancode::X) => Some(0x0),
        Some(Scancode::C) => Some(0xB),
        Some(Scancode::V) => Some(0xF),
        _ => None,
    }
}
