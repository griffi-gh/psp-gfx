use psp::sys::GuPrimitive;

use crate::{Frame, define_vertex_layout, rect::Rect};

pub trait GfxExt {
    fn gfx_rect(&self, rect: Rect);
}

impl<'gfx> GfxExt for Frame<'gfx> {
    /// Draw a filled rectangle at the specified position
    ///
    /// To set the color use [`Frame::set_color`]
    fn gfx_rect(&self, rect: Rect) {
        define_vertex_layout! {
            Vertex {
                vertex: VERTEX_16BIT,
                transform: TRANSFORM_2D,
            }
        };
        let vertex_buf = self.get_memory(&[
            Vertex::from_position2(rect.x as u16, rect.y as u16),
            Vertex::from_position2((rect.x + rect.w) as u16, (rect.y + rect.h) as u16),
        ]);
        self.draw_array(GuPrimitive::Sprites, &vertex_buf);
    }
}
