use psp::sys::VertexType;

pub trait Vertex {
    fn vtype() -> VertexType;
}

#[macro_export]
macro_rules! define_vertex_layout {
    (
        auto,
        texture = $tex_bits:literal,
        vertex = $vtx_bits:literal,
        transform = $transform:ident
        $(, color = $color_fmt:tt)?
        $(, normal = $normal_bits:literal)?
        $(, weight = $weight_bits:literal)?
        $(, index = $index_bits:literal)?
    ) => {
        $crate::paste::paste! {
            define_vertex_layout!(
                [<VertexTex $tex_bits Vtx $vtx_bits $transform:upper
                    $( C $color_fmt)? $( N $normal_bits)? $( W $weight_bits)? $( I $index_bits)? >],
                texture = $tex_bits,
                vertex = $vtx_bits,
                transform = $transform
                $(, color = $color_fmt)?
                $(, normal = $normal_bits)?
                $(, weight = $weight_bits)?
                $(, index = $index_bits)?
            );
        }
    };

    (
        $name:ident,
        texture = $tex_bits:literal,
        vertex = $vtx_bits:literal,
        transform = $transform:ident
        $(, color = $color_fmt:tt)?
        $(, normal = $normal_bits:literal)?
        $(, weight = $weight_bits:literal)?
        $(, index = $index_bits:literal)?
    ) => {
        $crate::paste::paste! {
            #[repr(C)]
            #[derive(::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
            struct $name {

                // Texture
                pub u: define_vertex_layout!(@type $tex_bits, float, unsigned),
                pub v: define_vertex_layout!(@type $tex_bits, float, unsigned),

                // Color
                $(
                    pub color: define_vertex_layout!(@color_type $color_fmt),
                )?

                // Normal
                $(
                    pub nx: define_vertex_layout!(@type $normal_bits, float, signed),
                    pub ny: define_vertex_layout!(@type $normal_bits, float, signed),
                    pub nz: define_vertex_layout!(@type $normal_bits, float, signed),
                )?

                // Weights
                $(
                    pub weight: define_vertex_layout!(@type $weight_bits, float, unsigned),
                )?

                // Index
                $(
                    pub index: define_vertex_layout!(@type $index_bits, int, unsigned),
                )?

                // Vertex
                pub x: define_vertex_layout!(@type $vtx_bits, float, signed),
                pub y: define_vertex_layout!(@type $vtx_bits, float, signed),
                pub z: define_vertex_layout!(@type $vtx_bits, float, signed),
            }

            impl $crate::vertex::Vertex for $name {
                fn vtype() -> ::psp::sys::VertexType {
                    const __TRANSFORM_D2: ::psp::sys::VertexType = ::psp::sys::VertexType::TRANSFORM_2D;
                    const __TRANSFORM_D3: ::psp::sys::VertexType = ::psp::sys::VertexType::TRANSFORM_3D;
                    ::psp::sys::VertexType::[<TEXTURE_ $tex_bits BIT>]
                        | ::psp::sys::VertexType::[<VERTEX_ $vtx_bits BIT>]
                        | [<__TRANSFORM _ $transform:upper>]
                        $( | ::psp::sys::VertexType::[<COLOR_ $color_fmt>] )?
                        $( | ::psp::sys::VertexType::[<NORMAL_ $normal_bits BIT>] )?
                        $( | ::psp::sys::VertexType::[<WEIGHT_ $weight_bits BIT>] )?
                        $( | ::psp::sys::VertexType::[<INDEX_ $index_bits BIT>] )?
                }
            }
        }
    };

    (@type 8, float, signed) => { i8 };
    (@type 16, float, signed) => { i16 };
    (@type 32, float, signed) => { f32 };
    (@type 8, float, unsigned) => { u8 };
    (@type 16, float, unsigned) => { u16 };
    (@type 32, float, unsigned) => { f32 };
    (@type 8, int, signed) => { i8 };
    (@type 16, int, signed) => { i16 };
    (@type 32, int, signed) => { i32 };
    (@type 8, int, unsigned) => { u8 };
    (@type 16, int, unsigned) => { u16 };
    (@type 32, int, unsigned) => { u32 };
    (@color_type 5650) => { u16 };
    (@color_type 5551) => { u16 };
    (@color_type 4444) => { u16 };
    (@color_type 8888) => { $crate::color::Color32 };
}
