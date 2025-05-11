use psp::sys::VertexType;

/// Marker trait used on [`TypedBuffers`] that can be used as Index Buffers for drawing
pub unsafe trait IndexItem {
    /// internal implementeation detail.
    fn vtype() -> VertexType;
}

unsafe impl IndexItem for u8 {
    fn vtype() -> VertexType {
        VertexType::INDEX_8BIT
    }
}

unsafe impl IndexItem for u16 {
    fn vtype() -> VertexType {
        VertexType::INDEX_16BIT
    }
}
