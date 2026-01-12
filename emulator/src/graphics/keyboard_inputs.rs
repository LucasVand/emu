use eframe::egui;

use crate::controller::buttons::ControllerButtons;

pub struct KeyboardInputs {}

impl KeyboardInputs {
    pub fn controller_buttons(ctx: &eframe::egui::Context) -> ControllerButtons {
        return ControllerButtons {
            a: ctx.input(|i| i.key_down(egui::Key::Z)),
            b: ctx.input(|i| i.key_down(egui::Key::X)),
            select: ctx.input(|i| i.key_down(egui::Key::Tab)),
            start: ctx.input(|i| i.key_down(egui::Key::Enter)),
            up: ctx.input(|i| i.key_down(egui::Key::ArrowUp)),
            down: ctx.input(|i| i.key_down(egui::Key::ArrowDown)),
            left: ctx.input(|i| i.key_down(egui::Key::ArrowLeft)),
            right: ctx.input(|i| i.key_down(egui::Key::ArrowRight)),
        };
    }
}

