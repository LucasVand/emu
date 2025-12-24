use std::ops::{Index, IndexMut};

pub struct Memory {
    pub memory: Box<[u8]>,       // size of 65536
    pub banks: Box<[Box<[u8]>]>, // size of 16385 by 256
}
impl Default for Memory {
    fn default() -> Self {
        // this vec stuff is make sure the stack is not overflowed
        let mut mem: Box<[u8]> = vec![0; 65536].into_boxed_slice();
        let bank_boxes = vec![vec![0; 16385]; 256].into_boxed_slice();

        let sph = 0xFC;
        let spl = 0x00;

        mem[Self::SPH as usize] = sph;
        mem[Self::SPL as usize] = spl;

        let banks = bank_boxes
            .iter()
            .map(|b| {
                return b.clone().into_boxed_slice();
            })
            .collect();

        Memory {
            memory: mem,
            banks: banks,
        }
    }
}
impl Memory {
    pub const MEM_BANK_LOW: u16 = 0x8000;
    pub const MEM_BANK_HIGH: u16 = 0xBFFF;
    pub const MEM_BANK_ADDR: u16 = 0xFFFA;

    pub const STACK_LOW: u16 = 0xFC00;
    pub const STACK_HIGH: u16 = 0xFEFF;
    pub const SPH: u16 = 0xFFFC;
    pub const SPL: u16 = 0xFFFD;
    pub const PCH: u16 = 0xFFFE;
    pub const PCL: u16 = 0xFFFF;

    pub fn get_stack(&self) -> u16 {
        let lowbyte = self[Self::SPL];
        let highbyte = self[Self::SPH];

        let value = ((highbyte as u16) << 8) | (lowbyte as u16);

        return value;
    }
    pub fn decrement_stack(&mut self) {
        let stack = self.get_stack() - 1;
        let lowbyte = stack as u8;
        let highbyte = (stack >> 8) as u8;

        self[Self::SPL] = lowbyte;
        self[Self::SPH] = highbyte;
    }
    pub fn incriment_stack(&mut self) {
        let stack = self.get_stack() + 1;
        let lowbyte = stack as u8;
        let highbyte = (stack >> 8) as u8;

        self[Self::SPL] = lowbyte;
        self[Self::SPH] = highbyte;
    }

    pub fn get_membank(&self) -> u8 {
        self.memory[Self::MEM_BANK_ADDR as usize]
    }
    pub fn set_membank(&mut self, new_value: u8) {
        self.memory[Self::MEM_BANK_ADDR as usize] = new_value;
    }
    pub fn set_pc(&mut self, new_value: u16) {
        let high = (new_value >> 8) as u8;
        let low = new_value as u8;

        self.memory[Self::PCH as usize] = high;
        self.memory[Self::PCL as usize] = low;
    }
    pub fn get_pc(&self) -> u16 {
        let high = self.memory[Self::PCH as usize];
        let low = self.memory[Self::PCL as usize];

        return ((high as u16) << 8) | (low as u16);
    }

    pub fn load_instruction(&mut self) -> [u8; 3] {
        let addr = self.get_pc();

        let first = self[addr];
        let second = self[addr + 1];
        let third = self[addr + 2];

        return [first, second, third];
    }
}
impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        if index >= Self::MEM_BANK_LOW && index <= Self::MEM_BANK_HIGH {
            if self.get_membank() == 0 {
                return &self.memory[index as usize];
            } else {
                return &self.banks[self.get_membank() as usize][index as usize];
            }
        }
        return &self.memory[index as usize];
    }
}
impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        if index >= Self::MEM_BANK_LOW && index <= Self::MEM_BANK_HIGH {
            if self.get_membank() == 0 {
                return &mut self.memory[index as usize];
            } else {
                return &mut self.banks[self.get_membank() as usize][index as usize];
            }
        }
        return &mut self.memory[index as usize];
    }
}
