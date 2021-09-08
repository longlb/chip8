use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

pub struct Display {
    canvas: WindowCanvas,
    scale: u32,
}

// a visual display for our chip8 interpreter using sdl2
impl Display {
    pub fn new(context: &sdl2::Sdl, scale: u32) -> Self {
        // init visual window and canvas
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window("chip8", 64 * scale, 32 * scale)
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().build().unwrap(),
            scale,
        }
    }

    // make the display show any recent modifications
    pub fn present(&mut self) {
        self.canvas.present()
    }

    // for the below presenetation will be taken care of by the CPU to prevent excessive calls
    // clear the display of any modifications
    pub fn wipe(&mut self) {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
    }

    // fill in or erase a pizel
    pub fn draw_pixel(&mut self, x: i32, y: i32, write: bool) -> Result<(), String> {
        self.canvas.set_draw_color(match write {
            true => WHITE,
            false => BLACK,
        });
        self.canvas.fill_rect(Rect::new(
            x * self.scale as i32,
            y * self.scale as i32,
            self.scale,
            self.scale,
        ))
    }
}
