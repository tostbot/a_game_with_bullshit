#[derive(Copy, Clone)]
pub struct Vertex {
    pub uv: [f32; 2],
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);