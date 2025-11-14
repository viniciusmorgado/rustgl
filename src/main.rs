use gl::{
    CreateShader, ShaderSource, VERTEX_SHADER,
    types::{GLfloat, GLuint},
};
use glfw::{
    Action, Context, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode, init,
};

extern crate gl;
extern crate glfw;

mod shader;

fn main() {
    // Initialize the GLFW library (no window or OpenGL context is created yet)
    let mut glfw = init(glfw::fail_on_errors).expect("Failed to initialize GLFW library");

    // Request an OpenGL context with version 4.6
    glfw.window_hint(WindowHint::ContextVersion(4, 6));

    // Select the Core profile (modern OpenGL functions only)
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let vertices: Vec<GLfloat> = vec![
        -0.5,
        -0.5 * 3.0_f32.sqrt() / 3.0,
        0.0,
        -0.5,
        -0.5 * 3.0_f32.sqrt() / 3.0,
        0.0,
        -0.0,
        -0.5 * 3.0_f32.sqrt() * 2.0 / 3.0,
        0.0,
    ];

    // Create a window and its associated OpenGL context
    let (mut window, events) = glfw
        .create_window(1920, 1080, "RustGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make this window's context the active OpenGL context
    window.make_current();

    // Load OpenGL function pointers using GLFW's procedure address function
    // This must be called after make_current() and before any OpenGL calls
    // We use transmute to convert the Option<fn()> returned by GLFW to a *const c_void
    gl::load_with(|symbol| {
        let proc_addr = window.get_proc_address(symbol);
        unsafe { std::mem::transmute(proc_addr) }
    });

    // Enable window event polling for keyboard input and framebuffer resize events
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Set the OpenGL viewport to match the window dimensions
    // Viewport defines the region of the window where OpenGL will render
    unsafe {
        gl::Viewport(0, 0, 800, 800);
    }

    let vertex_shader: GLuint = unsafe { CreateShader(VERTEX_SHADER) };
    let source_ptr = shader::VERTEX_SHADER_SOURCE.as_ptr() as *const i8;

    unsafe { ShaderSource(vertex_shader, 1, &source_ptr, std::ptr::null()) };

    // Main loop
    while !window.should_close() {
        // Clear the color buffer with a background color (dark blue)
        // glClearColor sets the color used when clearing (R, G, B, A)
        // glClear actually performs the clearing operation
        unsafe {
            gl::ClearColor(0.1, 0.15, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Swap front and back buffers (double buffering)
        // This displays the rendered frame to the screen
        window.swap_buffers();

        // Poll for and process window events (keyboard, mouse, resize, etc.)
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

// Handle window-level events (keyboard, resize, etc.)
fn handle_window_event(window: &mut Window, event: WindowEvent) {
    match event {
        // Close window when Escape key is pressed
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        // Update viewport when window is resized
        // This ensures the rendering area matches the new window dimensions
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}
