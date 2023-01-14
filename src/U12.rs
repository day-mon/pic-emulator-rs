
pub struct U12 {
    pub value: u16,
}


impl U12 {
    fn new(value: u16) -> U12 {
        U12 { value: value & 0x0FFF }
    }

    fn get(&self) -> u16 {
        self.value
    }

    pub fn bitwise_and_with_16(&self, rhs: u16) -> U12 {
        U12::new(self.value & rhs)
    }

    pub fn bitwise_with_u12(&self, rhs: U12) -> U12 {
        U12::new(self.value & rhs.value)
    }

    pub fn as_u16(&self) -> u16 {
        self.value & 0x0FFF
    }

    pub fn bitwise_and_with_32(&self, rhs: u32) -> U12 {
        U12::new(self.value & rhs as u16)
    }
}

impl Clone for U12 {
    fn clone(&self) -> Self {
        U12::new(self.value)
    }
}

impl Copy for U12 {}

