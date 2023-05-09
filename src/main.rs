extern crate glium;
extern crate glutin;

use glium::{Surface, VertexBuffer, IndexBuffer, Program};
use glium::glutin::{Event, WindowEvent, VirtualKeyCode, KeyboardInput};
use std::io::Cursor;
use std::f32::consts::PI;

// Define the vertex structure
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

// Define the indices for the wireframe model
const INDICES: &[u16] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
    120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
    140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179,
    180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199,
];

// Define the vertices for the wireframe model
const VERTICES: &[Vertex] = &[
Vertex { position: [-1.000000, 0.000000, 0.000000] },
Vertex { position: [0.000000, -1.000000, 0.000000] },
Vertex { position: [0.000000, 0.000000, -1.000000] },
Vertex { position: [0.723607, 0.525725, -0.447220] },
Vertex { position: [-0.276388, 0.850649, -0.447220] },
Vertex { position: [-0.894426, 0.000000, -0.447216] },
Vertex { position: [-0.276388, -0.850649, -0.447220] },
Vertex { position: [0.723607, -0.525725, -0.447220] },
Vertex { position: [0.276388, 0.850649, 0.447220] },
Vertex { position: [-0.723607, 0.525725, 0.447220] },
Vertex { position: [-0.723607, -0.525725, 0.447220] },
Vertex { position: [0.276388, -0.850649, 0.447220] },
Vertex { position: [0.894426, 0.000000, 0.447216] },
Vertex { position: [0.000000, 0.000000, 1.000000] },
Vertex { position: [0.425323, 0.309011, -0.850654] },
Vertex { position: [-0.162456, 0.499995, -0.850654] },
Vertex { position: [0.262869, 0.809012, -0.525738] },
Vertex { position: [0.425323, -0.309011, -0.850654] },
Vertex { position: [0.850648, 0.000000, -0.525736] },
Vertex { position: [-0.525730, 0.000000, -0.850652] },
Vertex { position: [-0.688190, 0.499997, -0.525736] },
Vertex { position: [-0.162456, -0.499995, -0.850654] },
Vertex { position: [-0.688190, -0.499997, -0.525736] },
Vertex { position: [0.262869, -0.809012, -0.525738] },
Vertex { position: [0.723607, 0.525725, 0.447220] },
Vertex { position: [-0.276388, 0.850649, 0.447220] },
Vertex { position: [-0.894426, 0.000000, 0.447216] },
Vertex { position: [-0.276388, -0.850649, 0.447220] },
Vertex { position: [0.723607, -0.525725, 0.447220] },
Vertex { position: [0.276388, 0.850649, -0.447220] },
Vertex { position: [-0.723607, 0.525725, -0.447220] },
Vertex { position: [-0.723607, -0.525725, -0.447220] },
Vertex { position: [0.276388, -0.850649, -0.447220] },
Vertex { position: [0.525730, 0.000000, -0.850652] },
Vertex { position: [-0.162456, 0.499995, 0.850654] },
Vertex { position: [0.425323, 0.309011, 0.850654] },
Vertex { position: [0.262869, 0.809012, 0.525738] },
Vertex { position: [-0.525730, 0.000000, 0.850652] },
Vertex { position: [-0.162456, -0.499995, 0.850654] },
Vertex { position: [-0.688190, 0.499997, 0.525736] },
Vertex { position: [0.425323, -0.309011, 0.850654] },
Vertex { position: [-0.688190, -0.499997, 0.525736] },
Vertex { position: [0.262869, -0.809012, 0.525738] },
];

static indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 1, 1, 5, 6, 1, 6, 2, 2, 6, 7, 2, 7, 3, 3, 7, 8, 3, 8, 4, 4, 8, 9, 4, 9, 5, 5, 9, 6, 10, 11, 6, 10, 6, 9, 11, 7, 6, 11, 12, 7, 11, 12, 8, 7, 12, 9, 8, 12, 10, 13, 14, 15, 13, 15, 16, 13, 16, 17, 13, 17, 18, 13, 18, 19, 13, 19, 14, 14, 19, 20, 14, 20, 15, 15, 20, 21, 15, 21, 16, 16, 21, 22, 16, 22, 17, 17, 22, 23, 17, 23, 18, 18, 23, 24, 18, 24, 19, 19, 24, 25, 19, 25, 20, 20, 25, 26, 20, 26, 21, 21, 26, 27, 21, 27, 22, 22, 27, 28, 22, 28, 23, 23, 28, 29, 23, 29, 24, 24, 29, 30, 24, 30, 25, 25, 30, 31, 25, 31, 26, 26, 31, 32, 26, 32, 27, 27, 32, 33, 27, 33, 28, 28, 33, 34, 28, 34, 29, 29, 34, 35, 29, 35, 30, 30, 35, 36, 30, 36, 31, 31, 36, 37, 31, 37, 32, 32, 37, 38, 32, 38, 33, 33, 38, 39, 33, 39, 34, 34, 39, 40, 34, 40, 35, 35, 40, 41, 35, 41, 36, 36, 41, 42, 36, 42, 37, 37, 42, 43, 37, 43, 38, 38, 43, 44, 38, 44, 39, 39, 44, 45, 39, 45, 40, 40, 45, 46, 40, 46, 41, 41, 46, 47, 41, 47, 42, 42, 47, 48, 42, 48, 43, 43, 48, 49, 43, 49, 44, 44, 49, 50, 44, 50, 45, 45, 50, 51, 45, 51, 46, 46, 51, 52, 46, 52, 47, 47, 52, 53, 47, 53, 48, 48, 53, 54, 48, 54, 49, 49, 54, 55, 49, 55, 50, 50, 55, 56, 50, 56, 51, 51, 56, 57, 51, 57, 52, 52, 57, 58, 52, 58, 53, 53, 58, 59, 53, 59, 54, 54, 59, 60, 54, 60, 55, 55, 60, 61, 55, 61, 56, 56, 61, 62, 56, 62, 57, 57, 62, 63, 57, 63, 58, 58, 63, 64, 58, 64, 59, 59, 64, 65, 59, 65, 60, 60, 65, 66, 60, 66, 61, 61, 66, 67, 61, 67, 62, 62, 67, 68, 62, 68, 63, 63, 68, 69, 63, 69, 64, 64, 69, 70, 64, 70, 65, 65, 70, 71, 65, 71, 66, 66, 71, 72, 66, 72, 67, 67, 72, 73, 67, 73, 68, 68, 73, 74, 68, 74, 69, 69, 74, 75, 69, 75, 70, 70, 75, 76, 70, 76, 71, 71, 76, 77, 71, 77, 72, 72, 77, 78, 72, 78, 73, 73, 78, 79, 73, 79, 74, 74, 79, 80, 74, 80, 75, 75, 80, 81, 75, 81, 76, 76, 81, 82, 76, 82, 77, 77, 82, 83, 77, 83, 78, 78, 83, 84, 78, 84, 79, 79, 84, 85, 79, 85, 80, 80, 85, 86, 80, 86, 81, 81, 86, 87, 81, 87, 82, 82, 87, 88, 82, 88, 83, 83, 88, 89, 83, 89, 84, 84, 89, 90, 84, 90, 85, 85, 90, 91, 85, 91, 86, 86, 91, 92, 86, 92, 87, 87, 92, 93, 87, 93, 88, 88, 93, 94, 88, 94, 89, 89, 94, 95, 89, 95, 90, 90, 95, 96, 90, 96, 91, 91, 96, 97, 91, 97, 92, 92, 97, 98, 92, 98, 93, 93, 98, 99, 93, 99, 94, 94, 99, 100, 94, 100, 95, 95, 100, 101, 95, 101, 96, 96, 101, 102, 96, 102, 97, 97, 102, 103, 97, 103, 98, 98, 103, 104, 98, 104, 99, 99, 104, 105, 99, 105, 100, 100, 105, 106, 100, 106, 101, 101, 106, 107, 101, 107, 102, 102, 107, 108, 102, 108, 103, 103, 108, 109, 103, 109, 104, 104, 109, 110, 104, 110, 105, 105, 110, 111, 105, 111, 106, 106, 111, 112, 106, 112, 107, 107, 112, 113, 107, 113, 108, 108, 113, 114, 108, 114, 109, 109, 114, 115, 109, 115, 110, 110, 115, 116, 110, 116, 111, 111, 116, 117, 111, 117, 118, 111, 118, 112, 112, 118, 119, 112, 119, 113, 113, 119, 120, 113, 120, 114, 114, 120, 121, 114, 121, 115, 115, 121, 122, 115, 122, 116, 116, 122, 123, 116, 123, 117, 117, 123, 124, 117, 124, 118, 118, 124, 125, 118, 125, 119, 119, 125, 126, 119, 126, 120, 120, 126, 127, 120, 127, 121, 121, 127, 128, 121, 128, 122, 122, 128, 129, 122, 129, 123, 123, 129, 130, 123, 130, 124, 124, 130, 131, 124, 131, 125, 125, 131, 132, 125, 132, 126, 126, 132, 133, 126, 133, 127, 127, 133, 134, 127, 134, 128, 128, 134, 135, 128, 135, 129, 129, 135, 136, 129, 136, 130, 130, 136, 137, 130, 137, 131, 131, 137, 138, 131, 138, 132, 132, 138, 139, 132, 139, 133, 133, 139, 140, 133, 140, 134, 134, 140, 141, 134, 141, 135, 135, 141, 142, 135, 142, 136, 136, 142, 143, 136, 143, 137, 137, 143, 144, 137, 144, 138, 138, 144, 145, 138, 145, 139, 139, 145, 146, 139, 146, 140, 140, 146, 147, 140, 147, 141, 141, 147, 148, 141, 148, 142, 142,
148, 149, 142, 149, 143, 143, 149, 150, 143, 150, 144, 144, 150, 151, 144, 151, 145, 145, 151, 152, 145, 152, 146, 146, 152, 153, 146, 153, 147, 147, 153, 154, 147, 154, 148, 148, 154, 155, 148, 155, 149, 149, 155, 156, 149, 156, 150, 150, 156, 157, 150, 157, 151, 151, 157, 158, 151, 158, 152, 152, 158, 159, 152, 159, 153, 159, 160, 153, 160, 154, 154, 160, 161, 154, 161, 155, 155, 161, 162, 155, 162, 156, 156, 162, 163, 156, 163, 157, 157, 163, 164, 157, 164, 158, 158, 164, 165, 158, 165, 159, 159, 165, 166, 159, 166, 160, 160, 166, 167, 160, 167, 161, 161, 167, 168, 161, 168, 162, 162, 168, 169, 162, 169, 163, 163, 169, 170, 163, 170, 164, 164, 170, 171, 164, 171, 165, 165, 171, 172, 165, 172, 166, 166, 172, 173, 166, 173, 167, 167, 173, 174, 167, 174, 168, 168, 174, 175, 168, 175, 169, 169, 175, 176, 169, 176, 170, 170, 176, 177, 170, 177, 171, 171, 177, 178, 171, 178, 172, 172, 178, 179, 172, 179, 173, 173, 179, 180, 173, 180, 174, 174, 180, 181, 174, 181, 175, 175, 181, 182, 175, 182, 176, 176, 182, 183, 176, 183, 177, 177, 183, 184, 177, 184, 178, 178, 184, 185, 178, 185, 179, 179, 185, 186, 179, 186, 180, 180, 186, 187, 180, 187, 181, 181, 187, 188, 181, 188, 182, 182, 188, 189, 182, 189, 183, 183, 189, 190, 183, 190, 184, 184, 190, 191, 184, 191, 185, 185, 191, 192, 185, 192, 186, 186, 192, 193, 186, 193, 187, 187, 193, 194, 187, 194, 188, 188, 194, 195, 188, 195, 189, 189, 195, 196, 189, 196, 190, 190, 196, 197, 190, 197, 191, 191, 197, 198, 191, 198, 192, 192, 198, 199, 192, 199, 193, 193, 199, 200, 193, 200, 194, 194, 200, 201, 194, 201
];

static indices: Vec<u16> = vec![
    0, 1, 2, 2, 3, 0, 3, 2, 6, 6, 7, 3, 7, 6, 5, 5, 4, 7, 4, 0, 3, 3, 7, 4, 0, 1, 5, 5, 4, 0, 1, 2, 6, 6, 5, 1, 2, 3, 7, 7, 6, 2,
    // Equator
    8, 9, 10, 10, 11, 8, 11, 10, 14, 14, 15, 11, 15, 14, 13, 13, 12, 15, 12, 8, 11, 11, 15, 8, 8, 9, 13, 13, 12, 8, 9, 10, 14, 14, 13, 9, 10, 11, 15, 15, 14, 10,
    // Southern Hemisphere
    16, 17, 18, 18, 19, 16, 19, 18, 22, 22, 23, 19, 23, 22, 21, 21, 20, 23, 20, 16, 19, 19, 23, 16, 16, 17, 21, 21, 20, 16, 17, 18, 22, 22, 21, 17, 18, 19, 23, 23, 22, 18
];

mut vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
let index_buffer =  glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices)
        .unwrap();

let mut camera = Camera::new();

let mut closed = false;

while !closed {
    for event in display.poll_events() {
        match event {
            glium::glutin::event::Event::Closed => closed = true,
            glium::glutin::event::Event::KeyboardInput(_, _, Some(key_code), _) => {
                match key_code {
                    glium::glutin::event::VirtualKeyCode::W => camera.forward(),
                    glium::glutin::event::VirtualKeyCode::S => camera.backward(),
                    glium::glutin::event::VirtualKeyCode::A => camera.left(),
                    glium::glutin::event::VirtualKeyCode::D => camera.right(),
                    _ => (),
                }
            }
            glium::glutin::event::Event::MouseMoved(x, y) => camera.mouse_move(x, y),
            glium::glutin::event::Event::MouseWheel(delta, _) => camera.zoom(delta),
            _ => (),
        }
    }

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

    let uniforms = uniform! {
        matrix: camera.matrix(),
    };

    target
        .draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniforms,
           
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

struct Camera {
position: Vector3<f32>,
horizontal_angle: f32,
vertical_angle: f32,
fov: f32,
near: f32,
far: f32,
aspect_ratio: f32,
}

impl Camera {
    fn new() -> Camera {
        Camera {
            position: Vector3::new(0.0, 0.0, 2.0),
            horizontal_angle: 0.0,
            vertical_angle: 0.0,
            fov: 90.0,
            near: 0.1,
            far: 1000.0,
            aspect_ratio: 4.0 / 3.0,
        }
    }
}

fn matrix(&self) -> Matrix4<f32> {
    let eye = self.position;
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    let projection = cgmath::perspective(
        cgmath::Deg(self.fov),
        self.aspect_ratio,
        self.near,
        self.far,
    );

    let view = cgmath::Matrix4::look_at_rh(eye, center, up);
    projection * view
}

fn forward(&mut self) {
    let direction = Vector3::new(
        self.horizontal_angle.to_radians().sin(),
        self.vertical_angle.to_radians().sin(),
        self.horizontal_angle.to_radians().cos(),
    );
    self.position += direction * 0.1;
}

fn backward(&mut self) {
    let direction = Vector3::new(
        self.horizontal_angle.to_radians().sin(),
        self.vertical_angle.to_radians().sin(),
        self.horizontal_angle.to_radians().cos(),
    );
    self.position -= direction * 0.1;
}

fn left(&mut self) {
    let direction = Vector3::new(
        (self.horizontal_angle - 90.0).to_radians().sin(),
        self.vertical_angle.to_radians().sin(),
        (self.horizontal_angle - 90.0).to_radians().cos(),
    );
    self.position += direction * 0.1;
}

fn right(&mut self) {
    let direction = Vector3::new(
        (self.horizontal_angle + 90.0).to_radians().sin(),
        self.vertical_angle.to_radians().sin(),
        (self.horizontal_angle + 90.0).to_radians().cos(),
    );
    self.position += direction * 0.1;
}

fn mouse_move(&mut self, x: f64, y: f64) {
    let mouse_speed = 0.001;
    self.horizontal_angle += mouse_speed * (x - 640.0) as f32;
    self.vertical_angle += mouse_speed * (y - 360.0) as f32;

    if self.vertical_angle > 89.0 {
        self.vertical_angle = 89.0;
    }
    if self.vertical_angle < -89.0 { 
        self.vertical_angle=-89.0; 
    } 
} 

fn zoom(&mut self, delta: glium::glutin::event::MouseScrollDelta) { 
    match delta { glium::glutin::event::MouseScrollDelta::LineDelta(_, y)=> {
            self.fov -= y * 2.0;
            if self.fov < 5.0 {
                self.fov=5.0;
            } 
            if self.fov> 175.0 {
                self.fov = 175.0;
            }
        }
    _ => {}
    }
}


fn main() {
let events_loop = glium::glutin::event_loop::EventLoop::new();
let window_builder = glium::glutin::window::WindowBuilder::new()
.with_title("Earth")
.with_inner_size(glium::glutin::dpi::LogicalSize::new(1280.0, 720.0));
let context_builder = glium::glutin::ContextBuilder::new()
.with_depth_buffer(24)
.with_vsync(true);
let display =
glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
let vertices = [
    Vertex::new(-1.0, -1.0, -1.0, 0.0, 0.0),
    Vertex::new(1.0, -1.0, -1.0, 1.0, 0.0),
    Vertex::new(1.0, 1.0, -1.0, 1.0, 1.0),
    Vertex::new(-1.0, 1.0, -1.0, 0.0, 1.0),
    Vertex::new(-1.0, -1.0, 1.0, 0.0, 0.0),
    Vertex::new(1.0, -1.0, 1.0, 1.0, 0.0),
    Vertex::new(1.0, 1.0, 1.0, 1.0, 1.0),
    Vertex::new(-1.0, 1.0, 1.0, 0.0, 1.0),
];

let indices: [u16; 36] = [
    0, 1, 2, 2, 3, 0, 1, 5, 6, 6, 2, 1, 5, 4, 7, 7, 6, 5, 4, 0, 3, 3, 7, 4, 3, 2, 6, 6, 7,
    3, 4, 5, 1, 1, 0, 4,
];

let vertex_buffer =
    glium::VertexBuffer::new(&display, &vertices).expect("Failed to create vertex buffer");
let index_buffer =
    glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices)
        .expect("Failed to create index buffer");

let vertex_shader_src = r#"
    #version 330 core

    layout(location = 0) in vec3 position;
    layout(location = 1) in vec2 tex_coords;

    out vec2 v_tex_coords;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 1.0);
        v_tex_coords = tex_coords;
    }
"#;

let fragment_shader_src = r#"
    #version 330 core

    in vec2 v_tex_coords;

    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex,
v_tex_coords);
}
"#;

let program =
    glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("Failed to compile program");

let mut camera = Camera::new();
let mut is_first_mouse = true;
let mut last_x = 0.0;
let mut last_y = 0.0;

let texture = texture_from_image(&display, "earth.jpg");

events_loop.run(move |event, _, control_flow| {
    match event {
        glium::glutin::event::Event::WindowEvent { event, .. } => match event {
            glium::glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glium::glutin::event_loop::ControlFlow::Exit
            }
            glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                camera.process_keyboard(&input);
            }
            glium::glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                camera.process_scroll(delta);
            }
            glium::glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as f32;
                let y = position.y as f32;
                if is_first_mouse {
                    last_x = x;
                    last_y = y;
                    is_first_mouse = false;
                }
                let x_offset = x - last_x;
                let y_offset = last_y - y;
                last_x = x;
                last_y = y;
                camera.process_mouse_movement(x_offset, y_offset);
            }
            glium::glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                if button == glium::glutin::event::MouseButton::Left {
                    camera.process_mouse_button(state);
                }
            }
            _ => {}
        },
        glium::glutin::event::Event::MainEventsCleared => {
            let mut target = display.draw();
            target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

            let model = Mat4::from_scale(0.5);
            let view = camera.get_view_matrix();
            let projection = camera.get_projection_matrix();
            let matrix = projection * view * model;

            target
                .draw(
                    &vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniform! {matrix: Into::<[[f32; 4]; 4]>::into(matrix)}, &glium::DrawParameters { depth: glium::Depth { test: glium::DepthTest::IfLessOrEqual, write: true, ..Default::default() }, ..Default::default() }, ) .unwrap(); target.finish().unwrap(); } _=> {}
    }
});
}

struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,
    yaw: f32,
    pitch: f32,
    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}
 
impl Camera {
    fn new() -> Camera {
        let position = Vec3::new(0.0, 0.0, 3.0);
        let front = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = front.cross(&world_up).normalize();
        Camera {
            position,
            front,
            up,
            right,
            world_up,
            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        }
    }
 
    fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }
 
    fn get_projection_matrix(&self) -> Mat4 {
        let aspect_ratio = 4.0 / 3.0;
        Mat4::perspective_rh_no(
            self.zoom.to_radians(),
            aspect_ratio,
            0.1,
            100.0,
        )
    }
 
    fn process_keyboard(&mut self, input: &glium::glutin::event::KeyboardInput) {
        let speed = self.movement_speed;
        match input.virtual_keycode {
            Some(glium::glutin::event::VirtualKeyCode::W) => {
                self.position += self.front * speed;
            }
            Some(glium::glutin::event::VirtualKeyCode::S) => {
                self.position -= self.front * speed;
            }
            Some(glium::glutin::event::VirtualKeyCode::A) => {
                self.position -= self.right * speed;
            }
            Some(glium::glutin::event::VirtualKeyCode::D) => {
                self.position += self.right * speed;
            }
            _ => {}
        }
    }
 
    fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;
        self.yaw += x_offset;
        self.pitch += y_offset;
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0{ 
            self.pitch=-89.0; 
        } 
        let front=Vec3 { x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(), y: self.pitch.to_radians().sin(), z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(), } .normalize(); self.front=front; self.right=self.front.cross(&self.world_up).normalize(); self.up=self.right.cross(&self.front) } 
        
    fn process_mouse_scroll(&mut self, y_offset: f32) { 
        if self.zoom>= 1.0 && self.zoom <= 45.0 { 
            self.zoom -=y_offset; 
        } 
        if self.zoom<= 1.0 { 
            self.zoom=1.0; 
        } 
        if self.zoom>= 45.0 {
        self.zoom = 45.0;
        }
    }
}