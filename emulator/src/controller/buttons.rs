#[derive(Copy, Clone, Default)]
pub struct ControllerButtons {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
impl ControllerButtons {
    pub fn new() -> ControllerButtons {
        ControllerButtons {
            a: false,
            b: false,
            select: false,
            start: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
    pub fn buttons_to_byte(self) -> u8 {
        return (self.a as u8) << 0
            | (self.b as u8) << 1
            | (self.select as u8) << 2
            | (self.start as u8) << 3
            | (self.up as u8) << 4
            | (self.down as u8) << 5
            | (self.left as u8) << 6
            | (self.right as u8) << 7;
    }
}
