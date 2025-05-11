use psp::sys::VertexType;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub u: u16,
    pub v: u16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Vertex {
    pub(crate) fn vtype() -> VertexType {
        //TODO
        VertexType::TEXTURE_16BIT | VertexType::VERTEX_16BIT | VertexType::TRANSFORM_2D
    }
}
