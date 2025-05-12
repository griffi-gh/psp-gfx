#![no_std]
#![no_main]

extern crate alloc;

psp::module!("sample_module", 1, 1);

use psp::{SCREEN_HEIGHT, SCREEN_WIDTH, sys::GuPrimitive};
use psp_gfx::{PspGfx, color::Color32, define_vertex_layout, rect::Rect};

const FLAG_COLORS: &[u32] = &[0xE40303, 0xFF8C00, 0xFFED00, 0x008026, 0x004CFF, 0x732982];
const FLAG_STRIP_HEIGHT: u32 = SCREEN_HEIGHT / FLAG_COLORS.len() as u32;

define_vertex_layout! {
    Vertex {
        texture: TEXTURE_16BIT,
        vertex: VERTEX_16BIT,
        transform: TRANSFORM_2D,
        color: COLOR_8888,
    }
}

fn psp_main() -> ! {
    psp::enable_home_button();

    let (cpu, bus) = unsafe {
        psp::sys::scePowerSetClockFrequency(333, 333, 166);
        (
            psp::sys::scePowerGetCpuClockFrequency(),
            psp::sys::scePowerGetBusClockFrequency(),
        )
    };
    psp::dprintln!("current clock speed {cpu}/{bus}MHz");
    psp::dprintln!("Hello PSP from rust! owo");

    let mut vertices: [Vertex; FLAG_COLORS.len() * 2] = Default::default();
    for (idx, color) in FLAG_COLORS.iter().copied().enumerate() {
        let rect = Rect {
            x: 0,
            y: FLAG_STRIP_HEIGHT as i32 * idx as i32,
            w: SCREEN_WIDTH as i32,
            h: FLAG_STRIP_HEIGHT as i32,
        };
        let base_idx = idx << 1;
        vertices[base_idx] = Vertex {
            x: rect.x as _,
            y: rect.y as _,
            color: Color32::from_rgb(color),
            ..Default::default()
        };
        vertices[base_idx + 1] = Vertex {
            x: (rect.x + rect.w) as _,
            y: (rect.y + rect.h) as _,
            color: Color32::from_rgb(color),
            ..Default::default()
        };
    }

    let mut gfx = PspGfx::init();
    loop {
        let frame = gfx.start_frame();
        frame.clear_color_depth(Color32::BLACK, 0);

        let buf = frame.get_memory(&vertices);
        frame.draw_array(GuPrimitive::Sprites, &buf);
    }
}
