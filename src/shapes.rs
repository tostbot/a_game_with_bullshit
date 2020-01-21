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
use std::rc::Rc;



pub struct Bullet {
    x: f32,
    y: f32,
    size: f32,
    vertex_buffer: Rc<VertexBuffer<Vertex>>,
    index_buffer: Rc<IndexBuffer<u16>>,
    pub texture: Texture2d


}

impl Bullet {
    pub fn new(display: &Display, x: f32, y: f32, size: f32) -> Self {

        let image = image::load(Cursor::new(&include_bytes!("/home/andrey/Pictures/sprites/bullet.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::Texture2d::new(display, image).unwrap();

        let image_width = texture.width() as f32 / 200.0;
        let image_height = texture.height() as f32 / 200.0;


        let vertex_buffer = glium::VertexBuffer::new(display,
                                 &[
                                     Vertex { uv: [0.0, 0.0], position: [x - image_width / 2.0, y - image_height / 2.0] },
                                     Vertex { uv: [0.0, 1.0], position: [x - image_width / 2.0, y + image_height / 2.0] },
                                     Vertex { uv: [1.0, 1.0], position: [x + image_width / 2.0, y + image_height / 2.0] },
                                     Vertex { uv: [1.0, 0.0], position: [x + image_width / 2.0, y - image_height / 2.0] },
                                 ],
        ).unwrap();

        let index_buffer = glium::IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 0, 2, 3]
        ).unwrap();

        Bullet { x, y, size, vertex_buffer: Rc::new(vertex_buffer), index_buffer: Rc::new(index_buffer), texture}
    }
}


impl Mesh for Bullet {
    fn vertex_buffer(&self) -> Rc<VertexBuffer<Vertex>> {
        self.vertex_buffer.clone()
    }

    fn index_buffer(&self) -> Rc<IndexBuffer<u16>> {
        self.index_buffer.clone()
    }
    fn texture(&self) -> &Texture2d {
        &self.texture
    }
}
