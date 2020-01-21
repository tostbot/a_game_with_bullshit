#[derive(Copy, Clone)]
pub struct Vertex {
    pub uv: [f32; 2],
    pub position: [f32; 2],

}

implement_vertex!(Vertex, uv, position);