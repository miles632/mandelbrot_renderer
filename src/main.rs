extern crate glium;

use glium::Surface;
use num_complex;
use std::time::{Duration, Instant};

const screen_height: u32 = 1080;
const screen_width: u32 = 1080;

static num_frames: u32 = 0;
static last_time: f64 = 0.0;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}
glium::implement_vertex!(Vertex, position);

fn main() {
    let vertices = [
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
    ];

    let indices = [0, 1, 2, 0, 3, 1];

    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(
            screen_width,
            screen_height,
        ))
        .with_title("fractal renderer");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    /*===Shaders and buffers===*/
    let vbo = glium::VertexBuffer::new(&display, &vertices);
    let ibo =
        glium::IndexBuffer::<u32>::new(&display, glium::index::PrimitiveType::TrianglesList, &indices);

    let vertex_shader_src = include_str!("shaders.vert");
    let vertex_frag_src = include_str!("frag.frag");
    let shader_program = glium::Program::from_source(
        &display, 
        &vertex_shader_src, 
        vertex_frag_src, 
        None
    ).unwrap();

    events_loop.run(move |event, _, control_flow| {
        use glium::glutin;

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vbo, &ibo, &shader_program, &glium::uniforms::EmptyUniforms, &Default::default());

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
    });
}
