use eframe::egui::{self, Color32, ColorImage, Context, TextureHandle, TextureOptions};

pub struct PixelPaintMode {}

impl PixelPaintMode {
    const TILEMAP_ADDR: usize = 0x1000;
    pub fn create_varm_tile_texture(
        ctx: &Context,
        buf: &Box<[u8]>,
        size: [usize; 2],
    ) -> TextureHandle {
        let tile_map_buf = &buf[0..Self::TILEMAP_ADDR];
        assert_eq!(tile_map_buf.len(), 1024 * 4);
        let mut tile_map_expanded: Box<[u8]> = vec![0; 64 * 4 * 1024].into_boxed_slice();
        let mut x = 0;
        let mut y = 0;

        for tile_id in tile_map_buf {
            let start = Self::TILEMAP_ADDR + ((*tile_id as usize) * 64);
            let end = start + 64;
            let tilemap = &buf[start..end];
            for pixel in tilemap {
                tile_map_expanded[x + (size[0] * 8 * y)] = *pixel;
                x += 1;
                if x % 8 == 0 {
                    x -= 8;
                    y += 1;
                }
            }
            x += 8;
            y -= 8;

            if x >= size[0] * 8 {
                x = 0;
                y += 8;
            }
        }

        let pixels: Vec<Color32> = tile_map_expanded
            .iter()
            .map(|pixel| {
                let r = pixel >> 5;
                let g = (pixel & 0b00011100) >> 2;
                let b = pixel & 0b00000011;
                let color = egui::Color32::from_rgb(r * (255 / 7), g * (255 / 7), b * (255 / 3));
                return color;
            })
            .collect();

        let size = [size[0] * 8, size[1] * 8];
        let image = ColorImage { size, pixels };

        let texture = ctx.load_texture("framebufftile", image, TextureOptions::NEAREST);

        return texture;
    }

    pub fn create_vram_pixel_texture(ctx: &Context, buf: &Box<[u8]>, size: usize) -> TextureHandle {
        let width = size;
        let height = size;

        let pixels: Vec<Color32> = buf
            .iter()
            .map(|pixel| {
                let r = pixel >> 5;
                let g = (pixel & 0b00011100) >> 2;
                let b = pixel & 0b00000011;
                let color = egui::Color32::from_rgb(r * (255 / 7), g * (255 / 7), b * (255 / 3));
                return color;
            })
            .collect();

        let image = ColorImage {
            size: [height, width],
            pixels,
        };
        let texture = ctx.load_texture("framebuffpixel", image, TextureOptions::NEAREST);

        return texture;
    }
}
