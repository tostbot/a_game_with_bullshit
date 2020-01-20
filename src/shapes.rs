use glium::{Frame, VertexBuffer, IndexBuffer};
use std::fs;
use crate::mesh::Mesh;
use crate::vertex::Vertex;
use glium::index::PrimitiveType;
use glium::Display;
use std::io::Cursor;
use glium::framebuffer::ColorAttachment::Texture;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;

pub struct Triangle {
    x: f32,
    y: f32,
}

impl Triangle {
    pub fn new(x: f32, y: f32) -> Self {
        Triangle { x, y }
    }
}


impl Mesh for Triangle {
    fn vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
        glium::VertexBuffer::new(display,
                                 &[
                                     Vertex { uv: [0.0, 0.0], position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                                     Vertex { uv: [0.0, 1.0], position: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
                                     Vertex { uv: [1.0, 0.0], position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
                                 ],
        ).unwrap()
    }

    fn index_buffer(&self, display: &Display) -> IndexBuffer<u16> {
        glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                &[0u16, 1, 2]).unwrap()
    }
}

pub struct Bullet {
    x: f32,
    y: f32,
    size: f32,
    vbo: Option<VertexBuffer<Vertex>>,

}

impl Bullet {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Bullet { x, y, size, vbo: None}
    }
}


impl Bullet {
    fn get_texture(&self, display: &Display) -> Texture2d {
        let image = image::load(Cursor::new(&include_bytes!("/home/andrey/Pictures/sprites/bullet.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        glium::texture::Texture2d::new(display, image).unwrap()


    }
}


impl Mesh for Bullet {
    fn vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
        let texture = self.get_texture(display);

        let image_width = texture.width() as f32 / 300.0;
        let image_height = texture.height() as f32 / 300.0;


        glium::VertexBuffer::new(display,
                                 &[
                                     Vertex { uv: [0.0, 0.0], position: [self.x - image_width / 2.0, self.y - image_height / 2.0], color: [1.0, 0.0, 0.0] },
                                     Vertex { uv: [0.0, 1.0], position: [self.x - image_width / 2.0, self.y + image_height / 2.0], color: [1.0, 0.0, 0.0] },
                                     Vertex { uv: [1.0, 1.0], position: [self.x + image_width / 2.0, self.y + image_height / 2.0], color: [1.0, 0.0, 0.0] },
                                     Vertex { uv: [1.0, 0.0], position: [self.x + image_width / 2.0, self.y - image_height / 2.0], color: [1.0, 0.0, 0.0] },
                                 ],
        ).unwrap()
    }

    fn index_buffer(&self, display: &Display) -> IndexBuffer<u16> {
        glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                &[0u16, 1, 2, 0, 2, 3]).unwrap()
    }
}




