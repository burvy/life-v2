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


/// now all of T S boiler plate has to be written
/// bc winit decided it must be so
/// google gemini got a bit confused
/// u can always tell when something is ai when they
/// use outdated dependencies lol
/// also the window lives always
#[derive(Default)]
pub struct App {
    /// just make this the default
    /// window is window
    /// WINDOW SSISS NIDOASDNA OSIDNALSD
    pub window: Option<Arc<Window>>, // whyyy does this need to be public
    /// put ur pixels here
    /// also compiler rlly wants this to have a lifetime so wtv
    pub pixels: Option<Pixels<'static>>,
    /// RGB without the alpha to see if it doesnt crash
    pub bg_clr: [f64; 3],
}

impl ApplicationHandler for App {
    /// create a new window if there is no window already
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Cellular Automata") // this is probably how its spelled
                // no transparency bc it might crash
                .with_inner_size(LogicalSize::new(800, 600)); // do some more method shopping if u want
            // window is an arc which makes things so much better
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window.clone());
            // so we can solve borrow checker issues by just cloning window earlier
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, window);
            // the alternative to not using pollster is horrific so we should use it
            let pixels = pollster::block_on(
                async {
                    // if u dont know what await is we just poll this until its done
                    Pixels::new_async(size.width, size.height, surface_texture).await
                }
            ).expect("couldn't create pixels");
            self.pixels = Some(pixels); // TODO: just make this work
        }
    }
    /// detect events and run actions on them if you want
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId, // why do i even need window id what is this for
        event: WindowEvent,
    ) {
        dbg!("{} happened to window {}", &event, &window_id); // can we do something with window_id
        let window = self.window.as_ref().expect("ur window doesnt exist");
        let Some(pixels) = self.pixels.as_mut() else {
            dbg!("self pixels could not be did as mut");
            return;
        };
        match event {
            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    pixels.resize_surface(size.width, size.height).expect("couldn't resize surface");
                }
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorEntered { .. } => {
                self.bg_clr = [norm(206), norm(66), norm(43)]; // this is ferris rust orange btw
                window.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                self.bg_clr = [norm(43), norm(66), norm(206)]; // this would be the OPPOSITE of ferris orange
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                    let [r, g, b,] = self.bg_clr;

                    pixels.clear_color(Color { r, g, b, a: 1.0 });

                    if let Err(err) = pixels.render() {
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
