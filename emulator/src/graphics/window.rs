use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{sync::Arc, time::Duration};

use crate::emulator::Emulator;
use crate::graphics::keyboard_inputs::KeyboardInputs;
use crate::graphics::memory_window::MemoryWindow;
use crate::graphics::pixel_paint_mode::PixelPaintMode;
use crate::graphics::shared_buffer::SharedBuffer;
use crate::memory::Memory;
use eframe::egui::{self, Vec2};
use eframe::emath::History;

pub struct EmulatorWindow {
    pub vram: Arc<SharedBuffer>,
    pub controller: Arc<AtomicU8>,
    pub mem: Arc<SharedBuffer>,
    pub vram_window_open: Arc<AtomicBool>,
    pub mem_window_open: Arc<AtomicBool>,
    pub fps_history: History<usize>,
    pub last_id: usize,
}
impl EmulatorWindow {
    pub const SIZE: f32 = 125.0;
    pub const SCALE: f32 = 5.0;

    pub fn new(emu: &Emulator) -> Self {
        Self {
            last_id: 0,
            fps_history: History::new(2..100, 1.0),
            mem_window_open: Arc::new(AtomicBool::new(false)),
            vram_window_open: Arc::new(AtomicBool::new(false)),
            mem: Arc::new(SharedBuffer::new_with_value(&emu.memory.memory)),
            vram: Arc::new(SharedBuffer::new_with_value(&emu.memory.banks[1])),
            controller: Arc::new(AtomicU8::new(0)),
        }
    }
}

impl eframe::App for EmulatorWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.update_history();
        egui::CentralPanel::default().show(ctx, |ui| {
            // this is the hiding and showing button ui
            ui.horizontal_top(|ui| {
                if let Some(fps) = self.average_fps() {
                    ui.label(format!("{} fps", (1.0 / fps).round()));
                }

                let vram_shown = self.vram_window_open.load(Ordering::Relaxed);
                if vram_shown {
                    if ui.button("Hide VRAM").clicked() {
                        self.vram_window_open.store(false, Ordering::Relaxed);
                    }
                } else {
                    if ui.button("Show VRAM").clicked() {
                        self.vram_window_open.store(true, Ordering::Relaxed);
                    }
                }
                let mem_shown = self.mem_window_open.load(Ordering::Relaxed);
                if mem_shown {
                    if ui.button("Hide Memory").clicked() {
                        self.mem_window_open.store(false, Ordering::Relaxed);
                    }
                } else {
                    if ui.button("Show Memory").clicked() {
                        self.mem_window_open.store(true, Ordering::Relaxed);
                    }
                }
            });

            let buf = self.vram.read();
            let painter = ui.painter();
            PixelPaintMode::paint_pixels(
                painter,
                buf,
                Self::SCALE,
                Self::SIZE,
                Some(Vec2::new(0.0, 35.0)),
            );
        });

        let buttons = KeyboardInputs::controller_buttons(ctx);
        self.controller
            .store(buttons.buttons_to_byte(), Ordering::Release);

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
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
impl EmulatorWindow {
    fn update_history(&mut self) {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        self.fps_history.add(since_the_epoch.as_secs_f64(), 1);
    }
    fn average_fps(&self) -> Option<f32> {
        return self.fps_history.mean_time_interval();
    }
}

pub fn create_window(mut emu: Emulator, print_regs: bool) -> eframe::Result {
    let app = EmulatorWindow::new(&emu);
    let buf_clone = Arc::clone(&app.vram);
    let mem_clone = Arc::clone(&app.mem);
    let controller_clone = Arc::clone(&app.controller);

    thread::spawn(move || {
        // register call back
        emu.register_callback(move |addr, value, bank| {
            if bank == 1 && addr >= Memory::MEM_BANK_LOW && addr <= Memory::MEM_BANK_HIGH {
                buf_clone.update_addr((addr - 0x8000) as usize, value);
                buf_clone.publish();
            } else {
                mem_clone.update_addr(addr as usize, value);
                mem_clone.publish();
            }
        });

        loop {
            emu.cycle(print_regs);
            let contr = controller_clone.load(Ordering::Acquire);
            emu.memory[0xFFFB] = contr;

            if emu.memory.is_halted() {
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
