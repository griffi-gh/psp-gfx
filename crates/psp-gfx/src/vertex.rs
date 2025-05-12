use psp::sys::VertexType;

pub trait Vertex {
    fn vtype() -> VertexType;
}

// TODO support multiple weights/vertices (GU_WEIGHTS(n), GU_VERTICES(n))
#[macro_export]
macro_rules! define_vertex_layout {
    (
        $name:ident {
            texture: $texture:ident
            , vertex: $vertex:ident
            , transform: $transform:ident
            $(, color: $color:ident)?
            $(, normal: $normal:ident)?
            $(, weight: $weight:ident)?
            $(, index: $index:ident)?
            $(,)?
        } $(;)?
    ) => {
        #[repr(C, align(4))]
        #[derive(::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
        struct $name {
            pub u: $crate::define_vertex_layout!(@texture $texture),
            pub v: $crate::define_vertex_layout!(@texture $texture),
            $(
                pub color: $crate::define_vertex_layout!(@color $color),
            )?
            $(
                pub normal_x: $crate::define_vertex_layout!(@normal $normal),
                pub normal_y: $crate::define_vertex_layout!(@normal $normal),
                pub normal_z: $crate::define_vertex_layout!(@normal $normal),
            )?
            $(
                pub weight: $crate::define_vertex_layout!(@weight $weight),
            )?
            pub x: define_vertex_layout!(@vertex $vertex),
            pub y: define_vertex_layout!(@vertex $vertex),
            pub z: define_vertex_layout!(@vertex $vertex),
            pub _padding: [u8; {
                const SIZE: usize = {
                    (2 * ::core::mem::size_of::<$crate::define_vertex_layout!(@texture $texture)>())
                    $(
                        + ::core::mem::size_of::<$crate::define_vertex_layout!(@color $color)>()
                    )?
                    $(
                        + (3 * ::core::mem::size_of::<$crate::define_vertex_layout!(@normal $normal)>())
                    )?
                    $(
                        + ::core::mem::size_of::<$crate::define_vertex_layout!(@weight $weight)>()
                    )?
                    + (3 * ::core::mem::size_of::<$crate::define_vertex_layout!(@vertex $vertex)>())
                };
                (4 - (SIZE % 4)) % 4
            }]
        }
        impl $crate::vertex::Vertex for $name {
            fn vtype() -> ::psp::sys::VertexType {
                $(
                    ::psp::sys::VertexType::$weight |
                    ::psp::sys::VertexType::WEIGHTS1 |
                )?
                ::psp::sys::VertexType::$texture
                $(
                    | ::psp::sys::VertexType::$color
                )?
                $(
                    | ::psp::sys::VertexType::$normal
                )?
                $(
                    | ::psp::sys::VertexType::$index
                )?
                | ::psp::sys::VertexType::$vertex
                | ::psp::sys::VertexType::VERTICES1
                | ::psp::sys::VertexType::$transform
            }
        }
    };

    (@texture TEXTURE_8BIT) => {
        u8
    };
    (@texture TEXTURE_16BIT) => {
        u16
    };
    (@texture TEXTURE_32BITF) => {
        f32
    };
    (@color COLOR_5650) => {
        u16
    };
    (@color COLOR_5551) => {
        u16
    };
    (@color COLOR_4444) => {
        u16
    };
    (@color COLOR_8888) => {
        $crate::color::Color32
    };
    (@normal NORMAL_8BIT) => {
        u8
    };
    (@normal NORMAL_16BIT) => {
        u16
    };
    (@normal NORMAL_32BITF) => {
        f32
    };
    (@vertex VERTEX_8BIT) => {
        u8
    };
    (@vertex VERTEX_16BIT) => {
        u16
    };
    (@vertex VERTEX_32BITF) => {
        f32
    };
    (@weight WEIGHT_8BIT) => {
        u8
    };
    (@weight WEIGHT_16BIT) => {
        u16
    };
    (@weight WEIGHT_32BITF) => {
        f32
    };
    (@index INDEX_8BIT) => {
        u8
    };
    (@index INDEX_16BIT) => {
        u16
    };
}

define_vertex_layout! {
    Test {
        texture: TEXTURE_16BIT,
        vertex: VERTEX_16BIT,
        transform: TRANSFORM_2D
    }
}
