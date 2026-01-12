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
    pub l: u8,
    pub h: u8,
    pub z: u8,
    pub f: u8,
}
impl Registers {
    pub fn get_16bit_pair(&self, first: u8, second: u8) -> u16 {
        let reg1 = self[first];
        let reg2 = self[second];
        return ((reg1 as u16) << 8) as u16 | (reg2 as u16);
    }
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
            let (plus_carry, _) = (value2 as u16).overflowing_add(carry as u16);
            let (total, _) = (value1 as u16).overflowing_add(plus_carry as u16);
            self.set_carry(total > 255);

            let value1_sign = value1 >> 7;
            let value2_sign = value2 >> 7;
            let total_sign: u8 = (total >> 7) as u8;

            let overflow = value1_sign == value2_sign && value1_sign != total_sign;
            self.set_overflow(overflow);
        } else {
            let (value2_carry, _) = value2.overflowing_add(1);
            // if value1 is less then value2 + c then we underflow
            let res = value1 < value2_carry;
            let (neg, _) = (!value2).overflowing_add(1 - carry);
            let (total, _) = value1.overflowing_add(neg);
            self.set_borrow(res);

            let value1_sign = value1 >> 7;
            let value2_sign = value2 >> 7;
            let total_sign = total >> 7;
            let overflow = value1_sign != value2_sign && value1_sign != total_sign;
            self.set_overflow(overflow);
        }
    }
    pub fn set_f_bit(&mut self, bit: usize, value: bool) {
        let bit = (1 << bit) as u8;
        if value {
            self.f = self.f | bit;
        } else {
            self.f = self.f & !bit;
        }
    }
    pub fn set_overflow(&mut self, value: bool) {
        self.set_f_bit(3, value);
    }
    pub fn set_carry(&mut self, value: bool) {
        self.set_f_bit(5, value);
    }
    pub fn set_borrow(&mut self, value: bool) {
        self.set_f_bit(4, value);
    }
    pub fn set_zero(&mut self, value: bool) {
        self.set_f_bit(6, value);
    }
    pub fn set_less(&mut self, value: bool) {
        self.set_f_bit(7, value);
    }
    pub fn get_overflow(&self) -> bool {
        let mask: u8 = 0b00001000;
        return (self.f & mask) != 0;
    }
    pub fn get_borrow(&self) -> bool {
        let mask: u8 = 0b00010000;
        return (self.f & mask) != 0;
    }
    pub fn get_carry(&self) -> bool {
        let mask: u8 = 0b00100000;
        return (self.f & mask) != 0;
    }
    pub fn get_zero(&self) -> bool {
        let mask: u8 = 0b01000000;
        return (self.f & mask) != 0;
    }
    pub fn get_less(&self) -> bool {
        let mask: u8 = 0b10000000;
        return (self.f & mask) != 0;
    }
}
impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a: {:<4}  b: {:<4}  c: {:<4}  d: {:<4}  h: {:<4}  l: {:<4}  z: {:<4}  f: {:<4}  le: {}  ze: {}  c: {}  b: {}  ov: {}  ab: {:<6}  cd: {:<6} hl: {:<6}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.h,
            self.l,
            self.z,
            self.f,
            self.get_less() as u8,
            self.get_zero() as u8,
            self.get_carry() as u8,
            self.get_borrow() as u8,
            self.get_overflow() as u8,
            self.get_16bit_pair(0, 1),
            self.get_16bit_pair(2, 3),
            self.get_16bit_pair(5, 4)
        )
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
