use eframe::egui::{self, Pos2};
use eframe::egui::{Painter, Vec2};

pub struct PixelPaintMode {}

impl PixelPaintMode {
    pub fn paint_pixels(
        painter: &Painter,
        buf: &Box<[u8]>,
        scale: f32,
        size: f32,
        offset: Option<Vec2>,
    ) {
        let mut pos_x: f32 = 0.0;
        let mut pos_y: f32 = 0.0;

        for pixel in buf.iter() {
            let r = pixel >> 5;
            let g = (pixel & 0b00011100) >> 2;
            let b = pixel & 0b00000011;
            let color = egui::Color32::from_rgb(r * (255 / 7), g * (255 / 7), b * (255 / 3));
            let rect = egui::Rect::from_min_max(
                Pos2::new(pos_x, pos_y),
                Pos2::new(pos_x + scale, pos_y + scale),
            );
            let rect = rect.translate(offset.unwrap_or(Vec2::ZERO));
            painter.rect_filled(rect, 0, color);
            pos_x += scale;
            if pos_x >= scale * size {
                pos_x = 0.0;
                pos_y += scale;
            }
        }
    }
}
