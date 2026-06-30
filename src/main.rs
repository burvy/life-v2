mod window;
use window::App;
use winit::event_loop::EventLoop;

fn main() {
    println!("Hello, World!");
    let event_loop = EventLoop::new().expect("ur stupid event loop didnt even start correctly");
    let mut app = App {
        bg_clr: [206, 66, 43, 255],
        ..Default::default() // this is to handle the window which works with just default
    };
    let _ = event_loop.run_app(&mut app);
}
