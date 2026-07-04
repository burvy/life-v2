use std::sync::Arc;

use pixels::{
    Pixels, PixelsBuilder, SurfaceTexture,
    wgpu::{Backends, Color},
};
use pollster;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
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
                .with_inner_size(LogicalSize::new(800, 600)); // do some more method shopping if u want
            // window is an arc which makes things so much better
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();
            println!("created pixels with size {}x{}", size.width, size.height);
            // so we can solve borrow checker issues by just cloning window earlier
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
            // the alternative to not using pollster is horrific so we should use it
            let pixels = pollster::block_on(
                async {
                    // if u dont know what await is we just poll this until its done
                    PixelsBuilder::new(size.width, size.height, surface_texture)
                        // dont use Backends::all() (the default)
                        // dx12 has no adapters on
                        // my machine but still puts a DXGI surface on the window, which
                        // gets the vulkan device lost during Surface::configure and wgpu
                        // swallows device-lost errors
                        // so the surface silently stays
                        // unconfigured and the first render() panics
                        .wgpu_backend(Backends::VULKAN | Backends::GL)
                        .build_async()
                        .await
                }
            ).expect("couldn't create pixels");
            // TODO: move this somewhere more accessible
            self.graphics = Some(Graphics { window, pixels, bg_clr: [0.0, 0.0, 0.0], scale: 10 });
            // TODO: use this to draw pixels!
            if let Some(graphics) = self.graphics.as_mut() {
                logic::draw_fn(graphics);
            }
            // try rendering immediately
            self.graphics.as_mut().unwrap().pixels.render().expect("initial render failed");
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
                    graphics.pixels.resize_surface(size.width, size.height)
                        .expect("couldn't resize surface");
                    graphics.pixels.resize_buffer(size.width, size.height)
                        .expect("couldn't resize buffer");
                    graphics.window.request_redraw();
                }
            }
            return;
        }
        println!("{:?} happened to window {:?}", &event, &window_id); // can we do something with window_id
        let Some(graphics) = self.graphics.as_mut() else { return; };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorEntered { .. } => {
                graphics.bg_clr = [norm(206), norm(66), norm(43)];
                graphics.window.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                graphics.bg_clr = [norm(43), norm(66), norm(206)]; // this would be the OPPOSITE of ferris orange
                graphics.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {

                    let window_size = graphics.window.inner_size();
                    let surface_size = graphics.pixels.texture().size();
                    if window_size.width != surface_size.width || window_size.height != surface_size.height {
                        println!("size mismatch, skipping render");
                        return;
                    }
                    let [r, g, b,] = graphics.bg_clr;

                    graphics.pixels.clear_color(Color { r, g, b, a: 1.0 });

                    if let Err(err) = graphics.pixels.render() {
                        println!("couldnt render pixels bc of error: {err}");
                        event_loop.exit();
                    }
                }
            _ => (),
        }
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
        let (x_end, y_end) = (pixel.x * self.scale, pixel.y * self.scale);
        if x_end >= size.width as usize || y_end >= size.height as usize {
            eprintln!("pixel x={}, y={} is not in the window", x_end, y_end);
            return;
        }
        // i could be 0? what happens if you bitshift on 0
        let i = ((pixel.y * size.width as usize + pixel.x) * 4) as usize;
        let row_len = (size.width << 2) as usize;
        (0..self.scale).for_each(|j| {
            let row_start = j * row_len + i;
            self.pixels.frame_mut()[row_start..row_start + (self.scale << 2)]
                .copy_from_slice(&pixel.color.repeat(self.scale));
        });
    }

    /// just draws a white grid
    pub fn draw_grid(&mut self) {
        let color = [255, 255, 255, 255]; // white
        let (size_x, size_y) = (
            self.pixels.texture().size().width as usize,
            self.pixels.texture().size().height as usize
        );
        let scale = self.scale;
        let rep_y = size_y / scale;
        let rep_x = size_x / scale;

        (0..rep_y).for_each(|i| self.draw_h_line(i * scale)); // mul by scale not 16
    }

    // draws a white line at the given y
    fn draw_h_line(&mut self, y: usize) {
        let size = self.pixels.texture().size();
        if y >= size.height as usize {
            eprintln!("pixel y={} is not in the window", y);
            return;
        }
        let row_start = y * size.width as usize * 4;
        let row_end = row_start + (size.width as usize) * 4;
        self.pixels.frame_mut()[row_start..row_end]
            .copy_from_slice(&[255, 255, 255, 255].repeat(size.width as usize));
    }
}

/// normalizes an u8 to f64 (color from 0-255 to 0.0-1.0)
pub fn norm(num: u8) -> f64 {
    num as f64 / 255.0
}
