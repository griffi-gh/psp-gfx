#![no_std]
#![no_main]
#![allow(static_mut_refs)]

extern crate alloc;

psp::module!("sample_module", 1, 1);

mod graphics;
use graphics::{GraphicsManager, color::Color, rect::Rect};

use psp::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn psp_main() {
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

    let mut gfx = GraphicsManager::init();
    loop {
        gfx.begin_frame();
        gfx.clear_color_depth(Color::BLACK, 0);

        const FLAG_COLORS: &[u32] = &[0xE40303, 0xFF8C00, 0xFFED00, 0x008026, 0x004CFF, 0x732982];
        const FLAG_STRIP_HEIGHT: u32 = SCREEN_HEIGHT / FLAG_COLORS.len() as u32;
        for (idx, &color) in FLAG_COLORS.iter().enumerate() {
            gfx.fill_rect(
                Rect {
                    x: 0,
                    y: FLAG_STRIP_HEIGHT as i32 * idx as i32,
                    w: SCREEN_WIDTH as i32,
                    h: FLAG_STRIP_HEIGHT as i32,
                },
                Color::from_rgb(color),
            );
        }

        gfx.end_frame();
    }
}
