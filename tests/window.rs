use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::WindowAttributes;

#[derive(Default)]
pub struct App {
    window: Option<winit::window::Window>,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    WindowAttributes::default()
                    .with_decorations(false)
                    .with_resizable(true),
                )
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

#[test]
fn open_window() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    event_loop.run_app(&mut App::default()).unwrap();
}
