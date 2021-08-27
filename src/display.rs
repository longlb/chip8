use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

pub struct Display {
    pub canvas: WindowCanvas,
    scale: u32,
}

impl Display {
    pub fn new(context: &sdl2::Sdl, scale: u32) -> Self {
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 64 * scale, 32 * scale)
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().build().unwrap(),
            scale,
        }
    }

    pub fn wipe_screen(&mut self) {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn fill_pixel(&mut self, x: i32, y: i32) {
        self.canvas.set_draw_color(WHITE);
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(
                x * self.scale as i32,
                y * self.scale as i32,
                self.scale,
                self.scale,
            ))
            .unwrap();
        self.canvas.present();
    }

    pub fn wipe_pixel(&mut self, x: i32, y: i32) {
        self.canvas.set_draw_color(BLACK);
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(
                x * self.scale as i32,
                y * self.scale as i32,
                self.scale,
                self.scale,
            ))
            .unwrap();
        self.canvas.present();
    }
}
