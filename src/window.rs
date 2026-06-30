use pixels::{wgpu::web_sys::wasm_bindgen::UnwrapThrowExt, Pixels, SurfaceTexture};
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
    pub window: Option<Window>, // whyyy does this need to be public
    /// put ur pixels here
    /// also compiler rlly wants this to have a lifetime so wtv
    pub pixels: Option<Pixels<'static>>,
    /// RGBA
    pub bg_clr: [u8; 4],
}

impl ApplicationHandler for App {
    /// create a new window if there is no window already
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Cellular Automata") // this is probably how its spelled
                .with_transparent(true) // why not
                .with_inner_size(LogicalSize::new(800, 600)); // do some more method shopping if u want
            // clarification that one is window and the other is a reference to a window (the one in the App)
            let window: Window = event_loop
                .create_window(window_attributes)
                .unwrap();
            // we have to store window in self.window first
            self.window = Some(window);
            // then use self.window so the next usages of window dont come with borrow checker errors
            let window: &Window = self.window.as_ref().expect("ur window doesnt exist");
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
            dbg!("pixels dont exist");
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorEntered { .. } => {
                self.bg_clr = [206, 66, 43, 255]; // this is ferris rust orange btw
                window.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                self.bg_clr = [43, 66, 206, 255]; // this would be the OPPOSITE of ferris orange
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {}
            _ => (),
        }
    }
}
