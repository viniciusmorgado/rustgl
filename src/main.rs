use std::ffi::c_void;

use gl::{
    AttachShader, BindBuffer, BufferData, CompileShader, CreateProgram, CreateShader, DeleteShader,
    FRAGMENT_SHADER, GenBuffers, GenVertexArrays, LinkProgram, ShaderSource, VERTEX_SHADER,
    VertexArrayAttribBinding, VertexArrayAttribFormat,
    types::{GLfloat, GLuint},
};
use glfw::{
    Action, Context, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode, init,
};

extern crate gl;
extern crate glfw;

mod shaders;

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

    // Load and compile the vertex shader
    let vertex_shader: GLuint = unsafe { CreateShader(VERTEX_SHADER) };
    let source_ptr = shaders::VERTEX_SHADER_SOURCE.as_ptr() as *const i8;
    unsafe {
        ShaderSource(vertex_shader, 1, &source_ptr, std::ptr::null());
        CompileShader(vertex_shader);
    };

    // Load and compile the fragment shader
    let fragment_shader: GLuint = unsafe { CreateShader(FRAGMENT_SHADER) };
    let source_ptr = shaders::FRAGMENT_SHADER_SOURCE.as_ptr() as *const i8;
    unsafe {
        ShaderSource(fragment_shader, 1, &source_ptr, std::ptr::null());
        CompileShader(fragment_shader);
    };

    // Create and link the shader program
    let program: GLuint = unsafe { CreateProgram() };
    unsafe {
        AttachShader(program, vertex_shader);
        AttachShader(program, fragment_shader);
        LinkProgram(program);
    };

    // Delete the shaders after linking
    unsafe {
        DeleteShader(vertex_shader);
        DeleteShader(fragment_shader);
    }

    // Communication between CPU and GPU is slow, so we want to
    // send data between them in batchs, we call it buffers, but is
    // not the same thing as front and back buffers.
    unsafe {
        // vertex_buffer initialization
        let mut vertex_buffer: GLuint = 0;
        // vertex_array initialization
        let mut vertex_array: GLuint = 0;

        // Converts a mutable safe rust reference into a raw pointer.
        let vertex_buffer_ptr: *mut u32 = &mut vertex_buffer as *mut u32;
        let vertex_array_ptr: *mut u32 = &mut vertex_array as *mut u32;

        // Creates the buffer in C using the data from the pointer.
        GenVertexArrays(1, vertex_array_ptr); // the order here matters.
        // Creates the buffer in C using the data from the pointer
        GenBuffers(1, vertex_buffer_ptr);
        // Binds the named buffer object (vertex_buffer) to the specified target (gl::ARRAY_BUFFER).
        // This makes it the current buffer for subsequent operations on that target.
        BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

        let num_elements = vertices.len();
        // Get the size of a single element (GLfloat, which is f32)
        let element_size = size_of::<GLfloat>();
        // Calculate the total size in bytes for the data on the heap
        // This is the equivalent of sizeof(vertices) in C for a static array or a buffer.
        let total_size_bytes = (num_elements * element_size) as isize;

        // Convert vector data into a raw c_void pointer.
        let vertices_raw_ptr: *const c_void = vertices.as_ptr() as *const c_void;

        // Store the vertices in the vector_buffer
        BufferData(
            gl::ARRAY_BUFFER,
            total_size_bytes,
            vertices_raw_ptr,
            gl::STATIC_DRAW,
        );

        VertexArrayAttribFormat(0, 3, gl::FLOAT, gl::FALSE);
    }

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
