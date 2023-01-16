//7 special purpose registers
//16 general purpose registers

pub const REG_FILE_SIZE : u8 = 0x20;
pub const REG_FILE_MAX_ADDR : u8 = REG_FILE_SIZE - 1;
pub const REG_FILE_GP_OFFSET : u8 = 0x10;


#[derive(Clone, Copy)]
struct Register {
    value: u8
}

impl Register {
    pub fn new() -> Self {
        Register { value: 0 }
    }
}


pub enum SpecialPurposeRegisters {
    INDF = 0x00, //Indirect reference
    TMR0, //Timer
    PCL, //Program counter (to NEXT instruction)
    STATUS ,
    FSR, // pointer
    OSCCAL, //oscillator callibration
    GPIO, //general purpose input/output (pins)
    CMCON0 = 0x07,
    // 0x008 -> 0x0F is Unimplemented
    // 0x10 -> 0x1F is General Purpose Registers
}

pub struct RegisterFile {
    registers: [Register; REG_FILE_SIZE as usize]
}

impl RegisterFile {

    pub fn new() -> Self {
        RegisterFile {
            registers: [Register::new(); REG_FILE_SIZE as usize]
        }
    }

    pub fn write(&mut self, address: u8, val : u8) -> () {
        if address == SpecialPurposeRegisters::INDF as u8 {
            //this is not a physical address
            return;
        }
        if (0x08..=0x0F).contains(&address) {
            //unimplented registers in PIC10F200
            return;
        }

        self.registers[address as usize].value = val;
    }
    
    pub fn flash(&mut self) -> () {
        self.write(SpecialPurposeRegisters::PCL as u8, 0x00)
    }

    pub fn read(&self, address: u8) -> u8 {
        //indirect referencing
        if address == SpecialPurposeRegisters::INDF as u8 {
            // grab the the refrence int he fsr register
            let fsr_pointer = self.registers[SpecialPurposeRegisters::FSR as usize].value;
            // grab the value at that address that was in the fsr register
            let value = self.registers[fsr_pointer as usize].value;
            return value;
        }

        
        if (0x08..=0x0F).contains(&address) {
            //unimplented registers in PIC10F200
            return 0x00;
        }

        return self.registers[address as usize].value
    }
}