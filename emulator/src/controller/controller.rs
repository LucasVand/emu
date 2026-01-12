use crate::controller::buttons::ControllerButtons;

pub struct Controller {
    pub byte: u8,
}

impl Controller {
    pub fn new(b: ControllerButtons) -> Self {
        let byte = Self::buttons_to_byte(b);
        Self { byte: byte }
    }

    /// Called by your input system (keyboard, gamepad, etc.)
    pub fn buttons_to_byte(b: ControllerButtons) -> u8 {
        return (b.a as u8) << 0
            | (b.b as u8) << 1
            | (b.select as u8) << 2
            | (b.start as u8) << 3
            | (b.up as u8) << 4
            | (b.down as u8) << 5
            | (b.left as u8) << 6
            | (b.right as u8) << 7;
    }
}
