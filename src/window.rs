use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture, wgpu::Color};
use pollster;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct Graphics {
    /// just make this the default
    /// window is window
    /// WINDOW SSISS NIDOASDNA OSIDNALSD
    pub window: Arc<Window>,
    /// put ur pixels here
    /// also compiler rlly wants this to have a lifetime so wtv
    pub pixels: Pixels<'static>,

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
                    Pixels::new_async(size.width, size.height, surface_texture).await
                }
            ).expect("couldn't create pixels");
            self.graphics = Some(Graphics { window, pixels, bg_clr: [0.0, 0.0, 0.0] });
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
                    let surface = SurfaceTexture::new(size.width, size.height, graphics.window.clone());
                    graphics.pixels = pollster::block_on(
                        Pixels::new_async(size.width, size.height, surface)
                    ).expect("couldn't recreate pixels on resize");
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

/// normalizes an u8 to f64 (color from 0-255 to 0.0-1.0)
pub fn norm(num: u8) -> f64 {
    num as f64 / 255.0
}
