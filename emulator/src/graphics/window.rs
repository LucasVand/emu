use std::sync::atomic::AtomicBool;
use std::thread;
use std::{sync::Arc, time::Duration};

use crate::controller::buttons::ControllerButtons;
use crate::controller::controller::Controller;
use crate::emulator::Emulator;
use crate::graphics::double_buffer::DoubleBuffer;
use crate::graphics::keyboard_inputs::KeyboardInputs;
use crate::graphics::memory_window::MemoryWindow;
use crate::graphics::pixel_paint_mode::PixelPaintMode;
use arc_swap::ArcSwap;
use eframe::egui;

pub struct EmulatorWindow {
    pub vram: Arc<DoubleBuffer<Box<[u8]>>>,
    pub controller: Arc<ArcSwap<Controller>>,
    pub mem: Arc<DoubleBuffer<Box<[u8]>>>,
    pub vram_window_open: Arc<AtomicBool>,
    pub mem_window_open: Arc<AtomicBool>,
}
impl EmulatorWindow {
    pub const SIZE: f32 = 125.0;
    pub const SCALE: f32 = 5.0;

    pub fn new() -> Self {
        let buf1: Box<[u8]> = vec![0; 16385].into_boxed_slice();
        let buf2: Box<[u8]> = vec![0; 16385].into_boxed_slice();

        let mem1: Box<[u8]> = vec![0; 65536].into_boxed_slice();
        let mem2: Box<[u8]> = vec![0; 65536].into_boxed_slice();

        Self {
            mem_window_open: Arc::new(AtomicBool::new(false)),
            vram_window_open: Arc::new(AtomicBool::new(false)),
            mem: Arc::new(DoubleBuffer::new(mem1, mem2)),
            vram: Arc::new(DoubleBuffer::new(buf1, buf2)),
            controller: Arc::new(ArcSwap::from_pointee(Controller::new(
                ControllerButtons::new(),
            ))),
        }
    }
}

impl eframe::App for EmulatorWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let buf = self.vram.read();
            let painter = ui.painter();

            PixelPaintMode::paint_pixels(painter, buf, Self::SCALE, Self::SIZE, None);
        });

        let buttons = KeyboardInputs::controller_buttons(ctx);
        self.controller.store(Arc::new(Controller::new(buttons)));

        ctx.request_repaint_after(Duration::from_millis(16));

        MemoryWindow::show_memory(
            "VRAM",
            Arc::clone(&self.vram),
            0x8000,
            Arc::clone(&self.vram_window_open),
            ctx,
        );
        MemoryWindow::show_memory(
            "Memory",
            Arc::clone(&self.mem),
            0,
            Arc::clone(&self.mem_window_open),
            ctx,
        );
    }
}

pub fn create_window(mut emu: Emulator) -> eframe::Result {
    let app = EmulatorWindow::new();
    let buf_clone = Arc::clone(&app.vram);
    let mem_clone = Arc::clone(&app.mem);
    let controller_clone = Arc::clone(&app.controller);
    thread::spawn(move || {
        loop {
            emu.cycle(false);
            buf_clone.write(|f| {
                f.copy_from_slice(&mut emu.memory.banks[1]);
            });
            mem_clone.write(|f| {
                f.copy_from_slice(&mut emu.memory.memory);
            });
            let contr = controller_clone.load_full();
            emu.memory[0xFFFB] = contr.byte;

            if emu.registers.is_halted() {
                break;
            }
        }
    });

    let inner_size = EmulatorWindow::SIZE * EmulatorWindow::SCALE;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([inner_size, inner_size]),
        ..Default::default()
    };
    eframe::run_native("8-bit Emulator", options, Box::new(|_cc| Ok(Box::new(app))))
}
