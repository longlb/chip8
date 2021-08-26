// // init SDL context
// let sdl_context = sdl2::init()?;
// // init a copy of the the above SDL context as a video sub_system
// let video_subsystem = sdl_context.video()?;

// // init a windowbuilder
// let _window = video_subsystem
//     .window("Chip8", 640, 320)
//     .position_centered()
//     .build()
//     .map_err(|e| e.to_string())?;

// let mut event_pump = sdl_context.event_pump()?;
// 'main: loop {
//     for event in event_pump.poll_iter() {
//         // handle user input here
//         match event {
//             sdl2::event::Event::Quit { .. } => break 'main,
//             sdl2::event::Event::KeyDown { scancode, .. } => {
//                 println!("{:?}", scancodes(scancode))
//             }
//             _ => {}
//         }
//     }
//     std::thread::sleep(TIMING);
// }
