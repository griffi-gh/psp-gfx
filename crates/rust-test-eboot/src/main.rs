#![no_std]
#![no_main]

extern crate alloc;

psp::module!("sample_module", 1, 1);

use psp::{SCREEN_HEIGHT, SCREEN_WIDTH, sys::GuPrimitive};
use psp_gfx::{PspGfx, buffer::TypedBuffer, color::Color, define_vertex_layout, rect::Rect};

const FLAG_COLORS: &[u32] = &[0xE40303, 0xFF8C00, 0xFFED00, 0x008026, 0x004CFF, 0x732982];
const FLAG_STRIP_HEIGHT: u32 = SCREEN_HEIGHT / FLAG_COLORS.len() as u32;

define_vertex_layout! {
    Vertex,
    texture = 16,
    vertex = 16,
    transform = D2
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

    let gfx = PspGfx::init();
    loop {
        let frame = gfx.start_frame();
        frame.clear_color_depth(Color::BLACK, 0);

        for (idx, color) in FLAG_COLORS.iter().copied().enumerate() {
            let rect = Rect {
                x: 0,
                y: FLAG_STRIP_HEIGHT as i32 * idx as i32,
                w: SCREEN_WIDTH as i32,
                h: FLAG_STRIP_HEIGHT as i32,
            };
            frame.set_color(Color::from_rgb(color));
            frame.draw_array(
                &frame.new_typed_buffer(&[
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
                ]),
                None,
                GuPrimitive::Sprites,
            );
        }

        frame.finish();
    }
}
