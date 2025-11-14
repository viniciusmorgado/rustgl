use glfw::{
    Action, Context, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode, init,
};

extern crate glfw;

fn main() {
    // Initialize the GLFW library (no window or OpenGL context is created yet)
    let mut glfw = init(glfw::fail_on_errors).expect("Failed to initialize GLFW library");

    // Request an OpenGL context with version 4.6
    glfw.window_hint(WindowHint::ContextVersion(4, 6));

    // Select the Core profile (modern OpenGL functions only)
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // Create a window and its associated OpenGL context
    let (mut window, events) = glfw
        .create_window(800, 800, "RustGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make this window's context the active OpenGL context
    window.make_current();

    // Main loop
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

// Handle window-level events (keyboard, resize, etc.)
fn handle_window_event(window: &mut Window, event: WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
