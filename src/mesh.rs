use glium::{VertexBuffer, IndexBuffer, Display};
use crate::vertex::Vertex;
use std::rc::Rc;

use glium::texture::Texture2d;
pub trait Mesh{


    fn vertex_buffer(&self) -> Rc<VertexBuffer<Vertex>>;
    fn index_buffer(&self) -> Rc<IndexBuffer<u16>>;
    fn texture(&self) -> &Texture2d;

}


