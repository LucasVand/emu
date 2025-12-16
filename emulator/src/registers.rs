use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub h: u8,
    pub l: u8,
    pub z: u8,
    pub f: u8,
}
impl Registers {
    pub fn get_hl(&self) -> u16 {
        return ((self.h as u16) << 8) | (self.l as u16);
    }
    pub fn is_halted(&self) -> bool {
        return (self.f & 0b00000001) != 0;
    }
    pub fn update_zero_less(&mut self, value: u8) {
        let signed = value as i8;
        self.set_zero(value == 0);
        self.set_less(signed < 0);
    }
    pub fn update_carry_borrow_overflow(
        &mut self,
        value1: u8,
        value2: u8,
        carry: u8,
        is_add: bool,
    ) {
        if is_add {
            // if the total is greater then 255 overflow
            let total =
                (value1 as u16).saturating_add((value2 as u16).saturating_add(carry as u16));
            self.set_carry(total > 255);

            let overflow = (value1 >> 7) == (value2 >> 7) && (value1 >> 7) != ((total as u8) >> 7);
            self.set_overflow(overflow);
        } else {
            // if value1 is less then value2 + c then we underflow
            let res = (value1 as u16) < (value2 as u16) + carry as u16;
            let total =
                (value1 as u16).saturating_add((!(value2 as u16)).saturating_add(1 - carry as u16));
            self.set_borrow(res);

            let overflow = (value1 >> 7) != (value2 >> 7) && (value1 >> 7) != ((total as u8) >> 7);
            self.set_overflow(overflow);
        }
    }
    pub fn set_overflow(&mut self, value: bool) {
        let bit = (value as u8) << 3;
        let mask = !bit;
        self.f = self.f & mask;
    }
    pub fn set_carry(&mut self, value: bool) {
        let bit = (value as u8) << 5;
        let mask = !bit;
        self.f = self.f & mask;
    }
    pub fn set_borrow(&mut self, value: bool) {
        let bit = (value as u8) << 4;
        let mask = !bit;
        self.f = self.f & mask;
    }
    pub fn set_zero(&mut self, value: bool) {
        let bit = (value as u8) << 6;
        let mask = !bit;
        self.f = self.f & mask;
    }
    pub fn set_less(&mut self, value: bool) {
        let bit = (value as u8) << 7;
        let mask = !bit;
        self.f = self.f & mask;
    }
    pub fn get_overflow(&mut self) -> bool {
        let mask: u8 = 0b00001000;
        return (self.f & mask) != 0;
    }
    pub fn get_borrow(&mut self) -> bool {
        let mask: u8 = 0b00010000;
        return (self.f & mask) != 0;
    }
    pub fn get_carry(&mut self) -> bool {
        let mask: u8 = 0b00100000;
        return (self.f & mask) != 0;
    }
    pub fn get_zero(&mut self) -> bool {
        let mask: u8 = 0b01000000;
        return (self.f & mask) != 0;
    }
    pub fn get_less(&mut self) -> bool {
        let mask: u8 = 0b10000000;
        return (self.f & mask) != 0;
    }
}
impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
