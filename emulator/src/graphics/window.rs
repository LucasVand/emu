use std::thread;
use std::{sync::Arc, time::Duration};

use crate::controller::buttons::Buttons;
use crate::controller::controller::Controller;
use crate::emulator::Emulator;
use crate::graphics::double_buffer::DoubleBuffer;
use arc_swap::ArcSwap;
use eframe::egui;

pub struct EmulatorWindow {
    pub buf: Arc<DoubleBuffer<Box<[u8]>>>,
    pub controller: Arc<ArcSwap<Controller>>,
    pub mem: Arc<DoubleBuffer<Box<[u8]>>>,
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
            mem: Arc::new(DoubleBuffer::new(mem1, mem2)),
            buf: Arc::new(DoubleBuffer::new(buf1, buf2)),
            controller: Arc::new(ArcSwap::from_pointee(Controller::new(Buttons::new()))),
        }
    }
}

impl eframe::App for EmulatorWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let buf = self.buf.read();
            let painter = ui.painter();
            let mut pos_x: f32 = 0.0;
            let mut pos_y: f32 = 0.0;

            for pixel in buf.iter() {
                let r = pixel >> 5;
                let g = (pixel & 0b00011100) >> 2;
                let b = pixel & 0b00000011;
                let color = egui::Color32::from_rgb(r * (255 / 7), g * (255 / 7), b * (255 / 3));
                let rect = egui::Rect::from_min_max(
                    egui::Pos2::new(pos_x, pos_y),
                    egui::Pos2::new(pos_x + Self::SCALE, pos_y + Self::SCALE),
                );
                painter.rect_filled(rect, 0, color);
                pos_x += Self::SCALE;
                if pos_x >= Self::SCALE * Self::SIZE {
                    pos_x = 0.0;
                    pos_y += Self::SCALE;
                }
            }
        });

        let buttons = Buttons {
            a: ctx.input(|i| i.key_down(egui::Key::Z)),
            b: ctx.input(|i| i.key_down(egui::Key::X)),
            select: ctx.input(|i| i.key_down(egui::Key::Tab)),
            start: ctx.input(|i| i.key_down(egui::Key::Enter)),
            up: ctx.input(|i| i.key_down(egui::Key::ArrowUp)),
            down: ctx.input(|i| i.key_down(egui::Key::ArrowDown)),
            left: ctx.input(|i| i.key_down(egui::Key::ArrowLeft)),
            right: ctx.input(|i| i.key_down(egui::Key::ArrowRight)),
        };

        self.controller.store(Arc::new(Controller::new(buttons)));

        ctx.request_repaint_after(Duration::from_millis(16));

        egui::Window::new("VRAM")
            .default_pos((1000.0, 20.0))
            .show(ctx, |ui| {
                let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
                let bytes_per_row = 16;
                let memory = self.buf.read();

                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    memory.len() / bytes_per_row + 1,
                    |ui, row_range| {
                        for row in row_range {
                            let base = row * bytes_per_row;

                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}:", base + 0x8000));

                                for col in 0..bytes_per_row {
                                    let i = base + col;
                                    if i < memory.len() {
                                        ui.label(
                                            egui::RichText::new(format!("{:02X}", memory[i]))
                                                .monospace()
                                                .color(egui::Color32::from_rgb(200, 200, 200)),
                                        );
                                    }
                                }
                            });
                        }
                    },
                );
            });

        egui::Window::new("MEM")
            .default_pos((1000.0, 20.0))
            .show(ctx, |ui| {
                let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
                let bytes_per_row = 16;
                let memory = self.mem.read();

                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    memory.len() / bytes_per_row + 1,
                    |ui, row_range| {
                        for row in row_range {
                            let base = row * bytes_per_row;

                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}:", base));

                                for col in 0..bytes_per_row {
                                    let i = base + col;
                                    if i < memory.len() {
                                        ui.label(
                                            egui::RichText::new(format!("{:02X}", memory[i]))
                                                .monospace()
                                                .color(egui::Color32::from_rgb(200, 200, 200)),
                                        );
                                    }
                                }
                            });
                        }
                    },
                );
            });
    }
}

pub fn create_window(mut emu: Emulator) -> eframe::Result {
    let app = EmulatorWindow::new();
    let buf_clone = Arc::clone(&app.buf);
    let mem_clone = Arc::clone(&app.mem);
    let controller_clone = Arc::clone(&app.controller);
    thread::spawn(move || {
        loop {
            emu.cycle(true);
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
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([inner_size + 500.0, inner_size]),
        ..Default::default()
    };
    eframe::run_native("8-bit Emulator", options, Box::new(|_cc| Ok(Box::new(app))))
}
