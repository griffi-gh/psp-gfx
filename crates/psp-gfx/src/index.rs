use psp::sys::VertexType;

/// Marker trait implemented on types that can be used as indices for indexed rendering
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
