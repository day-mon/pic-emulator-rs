use crate::nbitnumber::{NumberOperations};
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
    TMR0, //Timer
    PCL, //Program counter (to NEXT instruction)
    STATUS ,
    FSR, // pointer
    OSCCAL, //oscillator calibration
    GPIO, //general purpose input/output (pins)
    CMCON0 = 0x07,
    // 0x008 -> 0x0F is Unimplemented
    // 0x10 -> 0x1F is General Purpose Registers
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