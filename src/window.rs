use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{CursorGrabMode, Window, WindowBuilder},
};

use crate::{HEIGHT, WIDTH};

pub struct GameWindow {
    pub window: Window,
    pub size: (u32, u32),
    pub pixels: Pixels,
}

impl GameWindow {
    pub fn new(title: &str, event_loop: &EventLoop<()>, scale_factor: f64) -> Result<Self, Error> {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(event_loop)
            .unwrap();

        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        window.set_cursor_visible(false);

        let texture_size = (
            (WIDTH as f64 * scale_factor) as u32,
            (HEIGHT as f64 * scale_factor) as u32,
        );
        let surface_texture = SurfaceTexture::new(texture_size.0, texture_size.1, &window);
        let pixels = Pixels::new(texture_size.0, texture_size.1, surface_texture)?;

        Ok(Self {
            window,
            size: (texture_size.0, texture_size.1),
            pixels,
        })
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        // If you want to resize the texture when the window is resized,
        // you should replace the new_size here with your desired scaling logic.
        self.pixels.resize_surface(new_size.0, new_size.1).unwrap();
        self.size = new_size;
    }
}
