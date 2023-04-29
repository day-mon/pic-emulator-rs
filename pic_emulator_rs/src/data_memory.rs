use crate::nbitnumber::{NumberOperations, NBitNumber};
use crate::nbitnumber::{u3, u5, u7, u9, u12};

//7 special purpose registers
//16 general purpose registers
//0x10 - 0x1F are GP
//0x00 - 0x0F are special + undefined
pub const REG_FILE_SIZE : u8 = 0x20;
pub const REG_FILE_MAX_ADDR : u8 =  0x1F;
pub const REG_FILE_GP_OFFSET : u8 = 0x10;


#[derive(Clone, Copy, Default)]
struct Register {
    value: u8
}

impl Register {
    pub fn new() -> Self {
        Register { value: 0x00 }
    }
}


pub enum SpecialPurposeRegisters {
    INDF = 0x00, //Indirect reference
    TMR0, //Timer: 8-bit RTC
    PCL, //Program counter low
    STATUS , //Status register
    FSR, // pointer
    OSCCAL, //oscillator calibration
    GPIO, //general purpose input/output (pins) 
    CMCON0 = 0x07, // comparator control
    // 0x008 -> 0x0F is Unimplemented
    // 0x10 -> 0x1F is General Purpose Registers
}

pub enum Status_Masks {
    C = 0x00, // carry/borrow flag
    DC = 0x01, // digit carry/borrow flag
    Z = 0x02, // zero flag
    PD = 0x03, // power down flag
    TO = 0x04, // timeout flag
    // 0x05 is Unimplemented
    CWUF = 0x06, // comparator wake up flag
    GPWUF = 0x07, // general purpose wake up flag
}

pub enum OSCCAL_Masks {
    FOSC4 = 0x00, // oscillator frequency select
    CAL0 = 0x01, 
    CAL1 = 0x02, 
    CAL2 = 0x03, 
    CAL3 = 0x04, 
    CAL4 = 0x05, 
    CAL5 = 0x06, 
    CAL6 = 0x07, 
}

pub enum GPIO_Masks {
    GP0 = 0x00,
    GP1 = 0x01,
    GP2 = 0x02,
    GP3 = 0x03,
}

pub enum CMCON0_Masks {
    CWU = 0x00, // comparator wake up flag
    CPREF = 0x01, // comparator positive reference
    CNREF = 0x02, // comparator negative reference
    CMPON = 0x03, // comparator on
    CMPT0CS = 0x04, // comparator timer 0 clock source
    POL = 0x05, // comparator output polarity
    COUTEN = 0x06, // comparator output enable
    COUT = 0x07, // comparator output
}

#[derive(Default)]
pub struct RegisterFile {
    registers: [Register; REG_FILE_SIZE as usize]
}

impl RegisterFile {

    pub fn new() -> Self {
        RegisterFile {
            registers: [Register::new(); REG_FILE_SIZE as usize]
        }
    }

    pub fn write(&mut self, address: u5, val : u8) -> () {
        
        if address.as_usize() == SpecialPurposeRegisters::INDF as usize  {
            // this is not a physical address
            return;
        }
        if (0x08..=0x0F).contains(&address.as_usize()) {
            // unimplemented registers in PIC10F200
            return;
        }

        self.registers[address.as_usize()].value = val;
    }

    pub fn set_flag(&mut self, mask: u3, val: NBitNumber<1>) -> () {
        //this function is used to set a bit of the status register to a value
        if mask.as_u16() > 0x07 {
            panic!("Invalid mask for status register");
        }

        if val == NBitNumber::new(1) {
            self.registers[SpecialPurposeRegisters::STATUS as usize].value |= 1 << mask.as_u16();
        } else {
            self.registers[SpecialPurposeRegisters::STATUS as usize].value &= 0 << mask.as_u16();
        }
    }
    
    pub fn flash(&mut self) {
        // set all memory locations to 0
        self.registers = [Register::new(); REG_FILE_SIZE as usize]
    }

    pub fn read(&self, address: u5) -> u8 {
        // indirect referencing
        if address == u5::new(SpecialPurposeRegisters::INDF as u16) {
            // grab the the reference int he fsr register
            let fsr_pointer = self.registers[SpecialPurposeRegisters::FSR as usize].value;
            // grab the value at that address that was in the fsr register
            let value = self.registers[fsr_pointer as usize].value;
            return value;
        }

        
        if (0x08..=0x0F).contains(&address.as_u16()) {
            // unimplemented registers in PIC10F200
            return 0x00;
        }

        self.registers[address.as_usize()].value
    }
}