# RustGL

A minimal OpenGL template project in Rust, perfect for learning OpenGL or as a starting point for graphics programming projects.

## Overview

This project provides a clean, well-documented foundation for OpenGL development in Rust. It sets up a basic window with an OpenGL 4.6 Core profile context using GLFW, with clear color rendering and basic event handling.

## Features

- **OpenGL 4.6 Core Profile** - Modern OpenGL setup without legacy functions
- **GLFW Window Management** - Cross-platform window creation and event handling
- **Well-Commented Code** - Educational comments explaining each OpenGL concept
- **Basic Event Handling** - Keyboard input and window resize support
- **Optimized Build Profiles** - Configured for both development and release builds

## Prerequisites

- Rust (edition 2024 or later)
- System dependencies for OpenGL and GLFW:
  - **Linux**: `libgl1-mesa-dev libglfw3-dev`
  - **macOS**: GLFW (via Homebrew: `brew install glfw`)
  - **Windows**: No additional dependencies needed

## Installation

1. Clone this repository:
```bash
git clone <your-repo-url>
cd rustgl
```

2. Build and run:
```bash
cargo run
```

## Project Structure

```
rustgl/
├── src/
│   └── main.rs          # Main application with OpenGL setup
├── Cargo.toml           # Project dependencies and configuration
└── README.md
```

## Dependencies

- **gl** (0.14.0) - OpenGL bindings
- **glfw** (0.60.0) - Window and context management
- **gl_generator** (0.14.0) - Generate OpenGL bindings
- **gl_loader** (0.1.2) - Load OpenGL function pointers

## What's Included

The template demonstrates:

1. **GLFW Initialization** - Setting up the windowing library
2. **OpenGL Context Creation** - Requesting OpenGL 4.6 Core profile
3. **Window Creation** - 800x800 window with title
4. **OpenGL Function Loading** - Loading function pointers via GLFW
5. **Event Polling** - Keyboard and framebuffer resize events
6. **Viewport Management** - Proper viewport setup and resize handling
7. **Render Loop** - Basic clear color rendering with double buffering

## Controls

- **ESC** - Close the window

## Usage as Template

To use this as a template for your own project:

1. Clone or fork this repository
2. Update `Cargo.toml` with your project name
3. Modify the window title in `src/main.rs:20`
4. Start adding your OpenGL rendering code in the main loop (after line 52)

## Build Profiles

The project includes optimized build profiles:

- **Development**: Fast compilation with basic optimization
- **Release**: Full optimization with LTO enabled

## Learning Path

This template is designed as a foundation. Next steps for learning OpenGL:

1. Drawing basic shapes (triangles, quads)
2. Using shaders (vertex and fragment shaders)
3. Textures and texture mapping
4. 3D transformations and camera systems
5. Lighting and materials
6. Advanced techniques (framebuffers, instancing, etc.)

## Resources

- [Learn OpenGL](https://learnopengl.com/) - Excellent OpenGL tutorials
- [OpenGL Documentation](https://www.khronos.org/opengl/) - Official OpenGL reference
- [GLFW Documentation](https://www.glfw.org/documentation.html) - GLFW API reference

## License

This project is provided as-is for educational purposes. Feel free to use it as a template for your own projects.

## Contributing

This is a minimal template project. If you find bugs or have suggestions for improvements that maintain the "minimal template" nature, feel free to open an issue or pull request.
