use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use pixels::{
    wgpu::{Backends, Color},
    Pixels, PixelsBuilder, SurfaceTexture,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{ElementState::Pressed, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, Window, WindowId},
};

use super::logic;

pub struct PixelInfo {
    pub x: usize,
    pub y: usize,
    pub color: [u8; 4],
}

pub struct Graphics {
    /// just make this the default
    /// window is window
    /// WINDOW SSISS NIDOASDNA OSIDNALSD
    pub window: Arc<Window>,
    /// put ur pixels here
    /// also compiler rlly wants this to have a lifetime so wtv
    pub pixels: Pixels<'static>,

    /// the scale for drawing pixels, it might just be better to leave it here
    pub scale: usize,

    /// RGB without the alpha to see if it doesnt crash
    pub bg_clr: [f64; 3],

    // abstract mutable grid
    // the grid is (y, x) format if you try to use it
    // indexing through the row elements is vertical,
    // indexing through the inner elements is horizontal
    pub grid: Vec<Vec<bool>>,

    pub next_tick: std::time::Instant,

    /// stores the last known position of the cursor
    /// it only updates on move
    /// go look in the code
    pub cursor_pos: PhysicalPosition<f64>,
}

/// now all of T S boiler plate has to be written
/// bc winit decided it must be so
/// google gemini got a bit confused
/// u can always tell when something is ai when they
/// use outdated dependencies lol
/// also the window lives always
#[derive(Default)]
pub struct App {
    pub graphics: Option<Graphics>,
}

impl ApplicationHandler for App {
    /// create a new window if there is no window already
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.graphics.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Cellular Automata") // this is probably how its spelled
                // no transparency bc it might crash
                .with_fullscreen(Some(Fullscreen::Borderless(None)));
            // window is an arc which makes things so much better
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();
            println!("created pixels with size {}x{}", size.width, size.height);
            // so we can solve borrow checker issues by just cloning window earlier
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
            // no pollster!!!
            let pixels = PixelsBuilder::new(size.width, size.height, surface_texture)
                .wgpu_backend(Backends::GL) // maybe use vulkan too if u can
                .build()
                .expect("some error while making pxels");
            // TODO: move this somewhere more accessible
            self.graphics = Some(Graphics {
                window,
                pixels,
                bg_clr: [0.0, 0.0, 0.0],
                scale: 32,
                grid: vec![],
                next_tick: Instant::now(),
                cursor_pos: PhysicalPosition { x: 0.0, y: 0.0 },
            });
            // TODO: use this to draw pixels!
            let graphics = self.graphics.as_mut().expect("could not create graphics");

            // initialize the grid
            let (size_x, size_y) = (
                graphics.pixels.texture().size().width as usize,
                graphics.pixels.texture().size().height as usize,
            );
            graphics.grid = vec![vec![false; size_x / graphics.scale]; size_y / graphics.scale];
            // draw graphics now
            logic::draw_fn(graphics);

            // try rendering immediately
            self.graphics
                .as_mut()
                .unwrap()
                .pixels
                .render()
                .expect("initial render failed");
            self.graphics.as_ref().unwrap().window.request_redraw();
        }
    }
    /// detect events and run actions on them if you want
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId, // why do i even need window id what is this for
        event: WindowEvent,
    ) {
        if let WindowEvent::Resized(size) = &event {
            if size.width > 0 && size.height > 0 {
                if let Some(graphics) = self.graphics.as_mut() {
                    // dont rebuild Pixels here it creates a second surface on the same
                    // window while the old one still owns it which is an Invalid surface
                    // error on vulkan
                    // resizing the existing one reconfigures in place
                    graphics
                        .pixels
                        .resize_surface(size.width, size.height)
                        .expect("couldn't resize surface");
                    graphics
                        .pixels
                        .resize_buffer(size.width, size.height)
                        .expect("couldn't resize buffer");
                    graphics.window.request_redraw();
                }
            }
            return;
        }
        println!("{:?} happened to window {:?}", &event, &window_id); // can we do something with window_id
        let Some(graphics) = self.graphics.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                graphics.cursor_pos = position;
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if button == winit::event::MouseButton::Left && state == Pressed {
                    let (x, y) = (
                        graphics.cursor_pos.x as usize / graphics.scale,
                        graphics.cursor_pos.y as usize / graphics.scale,
                    );
                    // even if you click the very edge
                    // will not panic
                    // please use this instead of grid[y][x]
                    if let Some(cell) = graphics.grid.get_mut(y).and_then(|row| row.get_mut(x)) {
                        *cell = true;
                    }
                }
            }
            WindowEvent::KeyboardInput { .. } => {
                logic::draw_fn(graphics); // TODO: redraw the window in a better place
                graphics.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let window_size = graphics.window.inner_size();
                let surface_size = graphics.pixels.texture().size();
                if window_size.width != surface_size.width
                    || window_size.height != surface_size.height
                {
                    println!("size mismatch, skipping render");
                    return;
                }
                let [r, g, b] = graphics.bg_clr;

                graphics.pixels.clear_color(Color { r, g, b, a: 1.0 });

                if let Err(err) = graphics.pixels.render() {
                    println!("couldnt render pixels bc of error: {err}");
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }

    /// this just so happens to be perfect for making an update loop
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(graphics) = self.graphics.as_mut() else {
            eprintln!("could not create graphics in about_to_wait method");
            return;
        };
        if Instant::now() >= graphics.next_tick {
            logic::draw_fn(graphics);
            graphics.window.request_redraw();
            graphics.next_tick = Instant::now() + Duration::from_secs(1); // change the cooldown as u wish
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(graphics.next_tick));
    }
}

impl Graphics {
    // this method is worse than drawing pixels on the grid
    // unless you want to draw individual pixels for whatever reason
    // /// draws a single pixel into the frame buffer [r, g, b, a] which is why you see 4 a lot
    // /// every pixel is 4 bytes
    // /// has a safety check for if the pixel is out of bounds
    // /// x and y start from the top left corner
    // /// pixels drawn here stay forever
    // /// make sure you remember to clear them
    // pub fn draw_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
    //     let size = self.pixels.texture().size();
    //     // you know what this is
    //     if x >= size.width || y >= size.height {
    //         eprintln!("pixel x={}, y={} is not in the window", x, y);
    //         return;
    //     }
    //     // this calculates the correct index because the frame buffer is a list
    //     // one pixel happens every 4 bytes, and those 4 bytes are r, g, b, a
    //     // it really sucks how theres no built in draw function/method
    //     // we can kind of simulate x and y, just note that it starts from the top left corner
    //     // the pixel buffer is kind of memory efficient i guess but weird
    //     let i = ((y * size.width + x) << 2) as usize; // bitshifting hehe
    //     // replaces the 4 elements that represent the color of the pixel with the input color
    //     self.pixels.frame_mut()[i..i + 4].copy_from_slice(&color);
    // }

    /// draws a pixel at the coordinates with color both provided in `PixelInfo`
    /// and the size provided in the `Graphics` definition
    pub fn draw_pixel_on_grid(&mut self, pixel: PixelInfo) {
        let size = self.pixels.texture().size();
        let i = ((pixel.y * size.width as usize + pixel.x) << 2) as usize;
        let row_len = (size.width << 2) as usize;
        (0..self.scale).for_each(|j| {
            let row_start = j * row_len + i;
            self.pixels.frame_mut()[row_start..row_start + (self.scale << 2)]
                .copy_from_slice(&pixel.color.repeat(self.scale));
        });
    }

    /// just draws a white grid
    pub fn draw_grid(&mut self) {
        // drawing horizontal lines
        let width = self.pixels.texture().size().width as usize;
        let row_bytes = width * 4; // because each byte is 4 things; r, g, b, a
        self.pixels
            .frame_mut() // all tha pixels
            .chunks_mut(row_bytes * self.scale) // chunks of rows (bytes), `scale` amount of rows
            .for_each(|chunk| {
                chunk[..row_bytes] // get the first row_bytes bytes (the first row)
                    .copy_from_slice(
                        &[255, 255, 255, 255].repeat(width), // we have a whole row and fill in all pixels in the row!
                    )
            });
        // drawing vertical lines
        self.pixels
            .frame_mut() // all of the pixels
            .chunks_exact_mut(row_bytes) // use window width not the predefined scale
            .for_each(|row| {
                row.chunks_mut(4 * self.scale) // so the last line doesnt cut off by round
                            .for_each(|chunk| chunk[..4].copy_from_slice(&[255, 255, 255, 255]));
            });
    }
}

// /// normalizes an u8 to f64 (color from 0-255 to 0.0-1.0)
// pub fn norm(num: u8) -> f64 {
//     num as f64 / 255.0
// }
