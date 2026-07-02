mod window;
use window::App;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();
    println!("Hello, World!");
    let event_loop = EventLoop::new().expect("ur stupid event loop didnt even start correctly");
    let mut app = App {
        ..Default::default()
    };
    let _ = event_loop.run_app(&mut app);
}
