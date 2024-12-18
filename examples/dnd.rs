use winit::{
    event::Event,
    event_loop::EventLoop,
    window::WindowAttributes,
};

fn main() {
    // Create a new event loop
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // Define window attributes
    let window_attributes = WindowAttributes::default().with_title("Drop files on me!!");

    // Create the window using the event loop and attributes
    let window = event_loop
        .create_window(window_attributes)
        .expect("Failed to create window");

    // Run the event loop
    event_loop.run(move |event, _| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::DragEnter { .. }
                | winit::event::WindowEvent::DragOver { .. }
                | winit::event::WindowEvent::DragDrop { .. }
                | winit::event::WindowEvent::DragLeave => {
                    println!("{:?}", event);
                }
                _ => (),
            },
            _ => (),
        }
    });
}
