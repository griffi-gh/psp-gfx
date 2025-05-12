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
                texture: TEXTURE_16BIT,
                vertex: VERTEX_16BIT,
                transform: TRANSFORM_2D,
            }
        };
        let vertex_buf = self.get_memory(&[
            Vertex {
                x: rect.x as _,
                y: rect.y as _,
                ..Default::default()
            },
            Vertex {
                x: (rect.x + rect.w) as _,
                y: (rect.y + rect.h) as _,
                ..Default::default()
            },
        ]);
        self.draw_array(GuPrimitive::Sprites, &vertex_buf);
    }
}
