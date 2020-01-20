use glium::{VertexBuffer, IndexBuffer, Display};
use crate::vertex::Vertex;
pub trait Mesh{


    fn vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex>;
    fn index_buffer(&self, display: &Display) -> IndexBuffer<u16>;
}