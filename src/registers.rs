use std::ops::{Index, IndexMut};

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub l: u8,
    pub h: u8,
    pub z: u8,
    pub f: u8,
}
impl Registers {
    pub const REGISTER_LIST: [&'static str; 8] = ["a", "b", "c", "d", "l", "h", "z", "f"];
    pub fn get_hl(&self) -> u16 {
        return ((self.h as u16) << 8) | (self.l as u16);
    }
}
impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            l: 0,
            h: 0,
            z: 0,
            f: 0,
        }
    }
}

impl Index<u8> for Registers {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.l,
            5 => &self.h,
            6 => &self.z,
            7 => &self.f,
            _ => panic!("Invalid register index {}", index),
        }
    }
}
impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            4 => &mut self.l,
            5 => &mut self.h,
            6 => &mut self.z,
            7 => &mut self.f,
            _ => panic!("Invalid register index {}", index),
        }
    }
}
