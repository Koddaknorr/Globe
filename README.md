# Globe animation

## Let's quickly go over the code. The `Camera` struct has the following fields:

- `position`: the position of the camera in 3D space.

- `front`: the direction the camera is facing.

- `up`: the up direction of the camera.

- `right`: the right direction of the camera.

- `world_up`: the up direction of the world.

- `yaw`: the yaw angle of the camera (rotation around the y-axis).

- `pitch`: the pitch angle of the camera (rotation around the x-axis).

- `movement_speed`: the speed at which the camera moves.

- `mouse_sensitivity`: the sensitivity of the mouse for controlling the camera.

- `zoom`: the zoom level of the camera.
  
  

- The `new` function initializes the camera with some default values. 

- The `get_view_matrix` function returns the view matrix of the camera, which transforms world coordinates to camera coordinates. 

- The `get_projection_matrix` function returns the projection matrix of the camera, which transforms camera coordinates to clip coordinates. 

- The `process_keyboard` function updates the camera position based on keyboard input. 

- The `process_mouse_movement` function updates the camera orientation based on mouse movement. 

- Finally, the `process_mouse_scroll` function updates the zoom level of the camera based on mouse scroll.

## Main

Now that we have defined all the necessary structures and functions, we can start writing the main function that will create a window, initialize the OpenGL context, and run the main rendering loop. 

Here is the code for the main function:

```rust
fn main() {
    // Initialize the window and OpenGL context.
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_title("Rust 3D Wireframe Model");
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Initialize the shaders and program.
    let program = glium::Program::from_source(
        &display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        None,
    )
    .unwrap();

    // Initialize the meshes.
    let earth_mesh = generate_earth_mesh();

    // Initialize the camera.
    let mut camera = Camera::new();

    // Run the main rendering loop.
    let mut closed = false;
    while !closed {
        // Handle events.
        let mut target = display.draw();
        let mut needs_redraw = false;
        event_loop.poll_events(|event| {
            match event {
                glium::glutin::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::event::WindowEvent::CloseRequested => {
                            closed = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                            camera.process
                        // the left shift key is pressed.
                        glium::glutin::event::VirtualKeyCode::LShift => {
                            camera.movement_speed = 50.0;
                        }
                        _ => {}
                    }
                }
            }
            glium::glutin::event::Event::DeviceEvent { event, .. } => {
                match event {
                    glium::glutin::event::DeviceEvent::MouseMotion { delta } => {
                        camera.process_mouse_movement(delta.0 as f32, delta.1 as f32);
                        needs_redraw = true;
                    }
                    glium::glutin::event::DeviceEvent::MouseWheel { delta } => {
                        match delta {
                            glium::glutin::event::MouseScrollDelta::LineDelta(_, y) => {
                                camera.process_mouse_scroll(y as f32);
                                needs_redraw = true;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });

    // Update the camera position.
    camera.process_keyboard(&event_loop);

    // Draw the scene.
    if needs_redraw {
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix();
        let earth_model_matrix = Matrix4::from_scale(10.0);
        let earth_mvp_matrix = projection_matrix * view_matrix * earth_model_matrix;
        let params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        target
            .draw(
                &earth_mesh.vertices,
                &earth_mesh.indices,
                &program,
                &uniform! {
                    mvp_matrix: Into::<[[f32; 4]; 4]>::into(earth_mvp_matrix),
                    color: [1.0, 1.0, 1.0, 1.0],
                },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    }
}
}
```

#### Let's go over the code for the main function.

- First, we initialize the window and OpenGL context using the `glium` library. 

- We set the window size to 800x600 pixels and the title to "Rust 3D Wireframe Model". We also request a depth buffer with 24 bits of precision to enable 3D rendering. 

- Then, we load the vertex and fragment shaders using the `from_source` function of the `glium::Program` struct. We also initialize the mesh data for the earth using the `generate_earth_mesh` function. 

- Next, we create a `Camera` object and enter the main rendering loop.

- Inside the rendering loop, we first handle events using the `poll_events` function of the `glium::glutin::event_loop::EventLoop` struct. 
  
  - We check for the `CloseRequested` event to know when to exit the loop. 
  - We also handle keyboard and mouse input events and update the camera accordingly. 
  - We set the `needs_redraw` flag to true whenever the camera is updated or the mouse is moved to indicate that the scene needs to be redrawn.

- Next, we update the camera position using the `process_keyboard` function of the `Camera` struct. This function checks for keyboard input events and updates the camera position accordingly.

- Finally, we draw the scene if the `needs_redraw` flag is set to true. 
  
  - We first clear the color and depth buffers using the clear_color_and_depth function of the Frame struct. 
  - We then calculate the view and projection matrices for the camera using the get_view_matrix and get_projection_matrix functions of the Camera struct. 
  - We also create a model matrix for the earth mesh using the from_scale function of the cgmath::Matrix4 struct. 
  - We then calculate the model-view-projection matrix for the earth mesh and create the DrawParameters object to enable depth testing.

- Finally, we draw the earth mesh using the draw function of the Frame struct. 
  
  - We pass in the vertices, indices, program, uniforms, and draw parameters as arguments to this function. 
  - We call the finish function of the Frame struct to finish rendering the frame and present it to the user.



That's it for the code! 

You can now compile and run this Rust program to display a 3D wireframe model of the earth rotating on screen.