extern crate glium;

use glium::{
    glutin::{event::VirtualKeyCode, event_loop::ControlFlow},
    uniform, Surface,
};
use num_complex;
use std::time::{Duration, Instant};

const SCREEN_HEIGHT: f32 = 1050.0;
const SCREEN_WIDTH: f32 = 1900.0;

const FPS:u32 = 60;

//static num_frames: u32 = 0;
//static last_time: f64 = 0.0;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}
glium::implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        ))
        .with_title("fractal renderer");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertices = [
        // VBO for the quad
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
    ];
    let vbo = glium::VertexBuffer::new(&display, &vertices).unwrap();

    /*===Shaders===*/

    let vertex_shader_src = include_str!("shaders.vert");
    let vertex_frag_src = include_str!("frag.frag");

    let shader_program =
        match glium::Program::from_source(&display, vertex_shader_src, vertex_frag_src, None) {
            Ok(program) => program,
            Err(err) => panic!("Shader compilation error: {}", err),
        };

    loop {
        // draw quad
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        event_loop.run(move |ev, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            let window_size = display.gl_window().window().inner_size();

            let viewport = glium::Rect {
                left: 0,
                bottom: 0,
                width: window_size.width,
                height: window_size.height,
            };

            let mut target = display.draw();

            target.clear_color(0.0, 0.0, 0.0, 1.0);

            target.draw(
                    &vbo,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                    &shader_program,
                    &uniform! {
                        width: SCREEN_WIDTH,
                        height: SCREEN_HEIGHT,
                    },
                    &Default::default(),
                )
                .unwrap();

            target.finish().unwrap();
            match ev {
                glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => (),
            }
        });
    }
}
