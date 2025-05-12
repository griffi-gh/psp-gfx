use psp::sys::VertexType;

pub trait Vertex {
    fn vtype() -> VertexType;
}

// TODO support multiple weights/vertices (GU_WEIGHTS(n), GU_VERTICES(n))
#[macro_export]
macro_rules! define_vertex_layout {
    (
        $name:ident {
            vertex: $vertex:ident,
            transform: $transform:ident
            $(, texture: $texture:ident)?
            $(, color: $color:ident)?
            $(, normal: $normal:ident)?
            $(, weight: $weight:ident)?
            $(, index: $index:ident)?
            $(,)?
        } $(;)?
    ) => {
        #[repr(C, align(4))]
        #[derive(::core::marker::Copy, ::core::clone::Clone)]
        struct $name {
            $(
                pub weight: $crate::define_vertex_layout!(@weight $weight),
            )?
            $(
                pub u: $crate::define_vertex_layout!(@texture $texture),
                pub v: $crate::define_vertex_layout!(@texture $texture),
            )?
            $(
                pub color: $crate::define_vertex_layout!(@color $color),
            )?
            $(
                pub normal_x: $crate::define_vertex_layout!(@normal $normal),
                pub normal_y: $crate::define_vertex_layout!(@normal $normal),
                pub normal_z: $crate::define_vertex_layout!(@normal $normal),
            )?
            pub x: $crate::define_vertex_layout!(@vertex $vertex),
            pub y: $crate::define_vertex_layout!(@vertex $vertex),
            pub z: $crate::define_vertex_layout!(@vertex $vertex),
            pub _padding: [u8; Self::PADDING]
        }

        #[allow(unused)]
        impl $name {
            const PADDING: usize = {
                const SIZE: usize = {
                    0
                    $(
                        + (2 * ::core::mem::size_of::<$crate::define_vertex_layout!(@texture $texture)>())
                    )?
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
            };
            const DEFAULT: Self = Self {
                $(
                    weight: { stringify!($weight); 0 },
                )?
                $(
                    u: { stringify!($texture); 0 },
                    v: 0,
                )?
                $(
                    color: $crate::define_vertex_layout!(@color_default $color),
                )?
                $(
                    normal_x: { stringify!($normal); 0 },
                    normal_y: 0,
                    pub normal_z: 0,
                )?
                x: 0,
                y: 0,
                z: 0,
                _padding: [0; Self::PADDING],
            };

            // TODO use macros to generate these
            pub const fn from_position2(
                x: $crate::define_vertex_layout!(@vertex $vertex),
                y: $crate::define_vertex_layout!(@vertex $vertex),
            ) -> Self {
                Self {
                    x,
                    y,
                    ..Self::DEFAULT
                }
            }

            pub const fn from_position(
                x: $crate::define_vertex_layout!(@vertex $vertex),
                y: $crate::define_vertex_layout!(@vertex $vertex),
                z: $crate::define_vertex_layout!(@vertex $vertex),
            ) -> Self {
                Self {
                    x,
                    y,
                    z,
                    ..Self::DEFAULT
                }
            }

            $(
                pub const fn from_position2_uv(
                    x: $crate::define_vertex_layout!(@vertex $vertex),
                    y: $crate::define_vertex_layout!(@vertex $vertex),
                    u: $crate::define_vertex_layout!(@texture $texture),
                    v: $crate::define_vertex_layout!(@texture $texture),
                ) -> Self {
                    Self {
                        x,
                        y,
                        u,
                        v,
                        ..Self::DEFAULT
                    }
                }

                pub const fn from_position_uv(
                    x: $crate::define_vertex_layout!(@vertex $vertex),
                    y: $crate::define_vertex_layout!(@vertex $vertex),
                    z: $crate::define_vertex_layout!(@vertex $vertex),
                    u: $crate::define_vertex_layout!(@texture $texture),
                    v: $crate::define_vertex_layout!(@texture $texture),
                ) -> Self {
                    Self {
                        x,
                        y,
                        z,
                        u,
                        v,
                        ..Self::DEFAULT
                    }
                }
            )?

            $(
                pub const fn from_position2_color(
                    x: $crate::define_vertex_layout!(@vertex $vertex),
                    y: $crate::define_vertex_layout!(@vertex $vertex),
                    color: $crate::define_vertex_layout!(@color $color),
                ) -> Self {
                    Self {
                        x,
                        y,
                        color,
                        ..Self::DEFAULT
                    }
                }

                pub const fn from_position_color(
                    x: $crate::define_vertex_layout!(@vertex $vertex),
                    y: $crate::define_vertex_layout!(@vertex $vertex),
                    z: $crate::define_vertex_layout!(@vertex $vertex),
                    color: $crate::define_vertex_layout!(@color $color),
                ) -> Self {
                    Self {
                        x,
                        y,
                        z,
                        color,
                        ..Self::DEFAULT
                    }
                }

                // $(
                //     pub const fn from_position2_uv_color(
                //         x: $crate::define_vertex_layout!(@vertex $vertex),
                //         y: $crate::define_vertex_layout!(@vertex $vertex),
                //         u: $crate::define_vertex_layout!(@texture $texture),
                //         v: $crate::define_vertex_layout!(@texture $texture),
                //         color: $crate::define_vertex_layout!(@color $color),
                //     ) -> Self {
                //         Self {
                //             x,
                //             y,
                //             u,
                //             v,
                //             color,
                //             ..Self::DEFAULT
                //         }
                //     }

                //     pub const fn from_position_uv_color(
                //         x: $crate::define_vertex_layout!(@vertex $vertex),
                //         y: $crate::define_vertex_layout!(@vertex $vertex),
                //         z: $crate::define_vertex_layout!(@vertex $vertex),
                //         u: $crate::define_vertex_layout!(@texture $texture),
                //         v: $crate::define_vertex_layout!(@texture $texture),
                //         color: $crate::define_vertex_layout!(@color $color),
                //     ) -> Self {
                //         Self {
                //             x,
                //             y,
                //             z,
                //             u,
                //             v,
                //             color,
                //             ..Self::DEFAULT
                //         }
                //     }
                // )?
            )?
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::DEFAULT
            }
        }

        impl $crate::vertex::Vertex for $name {
            fn vtype() -> ::psp::sys::VertexType {
                ::psp::sys::VertexType::empty()
                $(
                    | ::psp::sys::VertexType::$weight
                    | ::psp::sys::VertexType::WEIGHTS1
                )?
                $(
                    | ::psp::sys::VertexType::$texture
                )?
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

    (@color_default COLOR_5650) => {
        0
    };
    (@color_default COLOR_5551) => {
        0
    };
    (@color_default COLOR_4444) => {
        0
    };
    (@color_default COLOR_8888) => {
        $crate::color::Color32::TRANSPARENT
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
