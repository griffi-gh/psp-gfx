#![no_std]
#![no_main]

extern crate alloc;

psp::module!("sample_module", 1, 1);

use psp::{SCREEN_HEIGHT, SCREEN_WIDTH, sys::GuPrimitive};
use psp_gfx::{PspGfx, buffer::TypedBuffer, color::Color, rect::Rect};

const FLAG_COLORS: &[u32] = &[0xE40303, 0xFF8C00, 0xFFED00, 0x008026, 0x004CFF, 0x732982];
const FLAG_STRIP_HEIGHT: u32 = SCREEN_HEIGHT / FLAG_COLORS.len() as u32;

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

    let mut x;

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
            let vtx_buf = unsafe { TypedBuffer::new(&rect.to_sprites_vertices()) };

            frame.set_color(Color::from_rgb(color));
            frame.draw_array(&vtx_buf, None, GuPrimitive::Sprites);

            x = vtx_buf;
        }

        frame.finish();
    }
}
