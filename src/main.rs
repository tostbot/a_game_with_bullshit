#[macro_use]
extern crate glium;

use glium::texture::Texture2d;
use glium::index::PrimitiveType;
use glium::Surface;
use glium::glutin;
use crate::vertex::Vertex;
use crate::shapes::Bullet;
use crate::mesh::Mesh;
use std::io::Cursor;
use std::time::Instant;

mod vertex;
mod shapes;
mod mesh;
extern crate image;


fn main() {
    let mut events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
       .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();




    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;

                in vec2 uv;
                in vec2 position;

                out vec2 v_uv;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    v_uv = uv;
                }
            ",

            fragment: "
                #version 140
                in vec2 v_uv;
                out vec4 f_color;
                uniform sampler2D tex;
                void main() {
                    f_color = texture(tex, v_uv);
                    //f_color = vec4(v_uv, 1, 1);

                }
            "
        },

    ).expect("Failed to create shader");

    let bullet = Bullet::new(&display,0.0, 0.0, 5.0);

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };



    let mut t: f32 = 0.0;
    let mut last_time = std::time::Instant::now();

    events_loop.run(move |event, _, control_flow| {

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);



        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                ,
                // Redraw the triangle when the window is resized.
                glium:: glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };

        let time_delta = std::time::Instant::now() - last_time;

        let matrix = [
            [t.sin(), t.cos(), 0.0, 0.0],
            [-t.cos(), t.sin(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        t += 0.01 * time_delta.as_millis() as f32;

        let uniforms = uniform! {
            matrix: matrix,
            tex: &bullet.texture

        };

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);
        target.draw(&*bullet.vertex_buffer(), &*bullet.index_buffer(), &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        last_time = std::time::Instant::now();
    });

}