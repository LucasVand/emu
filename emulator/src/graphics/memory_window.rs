use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use eframe::egui::{self, CentralPanel, Context, Pos2, ViewportId};

use crate::graphics::shared_buffer::SharedBuffer;

pub struct MemoryWindow {}

impl MemoryWindow {
    pub fn show_memory(
        title: &str,
        buf: Arc<SharedBuffer>,
        offset: usize,
        open: Arc<AtomicBool>,
        ctx: &Context,
    ) {
        if !open.load(Ordering::Relaxed) {
            return;
        }

        let builder = egui::ViewportBuilder::default()
            .with_title(title)
            .with_position(Pos2::new(0.0, 0.0))
            .with_inner_size([400.0, 400.0])
            .with_resizable(true);

        let id = ViewportId::from_hash_of(title);
        ctx.show_viewport_deferred(id, builder, move |ctx, _class| {
            CentralPanel::default().show(ctx, |ui| {
                if ui.input(|i| i.viewport().close_requested()) {
                    // Tell parent to close us.
                    open.store(false, Ordering::Relaxed);
                }

                let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
                let bytes_per_row = ((ui.min_size().x - 50.0) / 22.0) as usize;
                let memory = buf.read();

                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    memory.len() / bytes_per_row + 1,
                    |ui, row_range| {
                        for row in row_range {
                            let base = row * bytes_per_row;

                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}:", base + offset));

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
                ctx.request_repaint_after(Duration::from_millis(16));
            });
        });
    }
}
