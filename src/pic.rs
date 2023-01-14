use std::ops::ControlFlow;

use crate::U12::U12;

pub enum PICCategory {
    Miscellaneous,
    BitOperation,
    ControlTransfer,
    OperationsWithW,
    ALUOperation,
}
pub enum PICMnemonic {
    // Miscellaneous
    NOP, CLRWDT, OPTION, RETFIE,
    SLEEP, MOVLB, TRIS, RETURN,
    

    // ALU Operation
    MOVEWF, CLR, SUBWF, DECF,
    IORWF, ANDWF, XORWF, ADDWF,
    MOVF, COMF, INCF, DECFSZ,
    RRF, RLF, SWAPF, INCFSZ,

    // Bit Operation
    BCF, BSF, BTFSC, BTFSS,

    // Control Transfer
    GOTO, CALL, RETLW,

    // Operations with W
    MOVLW, IORLW, ANDLW, XORLW,

    //Undefined Instruction
    UND
}



pub struct PICInstruction  {
    instruction_raw: U12,
    instruction: Option<PICMnemonic>,
    intruction_category: PICCategory,
}

impl PICInstruction {
    pub fn from_U12(instruction: U12) -> PICInstruction {
        PICInstruction {
            instruction_raw: instruction,
            intruction_category: match instruction.bitwise_and_with_32(0xC000).as_u16() {
                // misc & alu -> 0000 | 0000 | 0000 \ 
                // bit  -> 0100 | 0000 | 0000
                // control 1000 | 0000 | 0000 
                // operations = 1100 | 0000 | 0000
                0x0000 => match instruction.bitwise_and_with_16(0x03E0).as_u16() {
                    0x0000 => PICCategory::Miscellaneous, 
                    // 0000001
                    // 1111111
                    _ => PICCategory::ALUOperation,
                }
                0x4000 => PICCategory::BitOperation,
                0x8000 => PICCategory::ControlTransfer,
                0xC000  => PICCategory::OperationsWithW,
                _ => panic!("")
            },
            instruction: None
        }
    }


    fn execute(&self, instruction: &PICMnemonic) -> () {
        todo!()
    }

    fn memory(&self) -> () {
        todo!()
    }

    fn writeBack(&self) -> () {
        todo!()
    }


    /*
        Hex codes are the OPCO
     */
    fn decode(&self) -> Option<PICMnemonic> {
        return match self.intruction_category {
            PICCategory::ALUOperation => {
                match self.instruction_raw.bitwise_and_with_16(0x001F).as_u16() {
                    0x0001 => Some(PICMnemonic::MOVEWF),
                    0x0002 | 0x0003 => Some(PICMnemonic::CLR),
                    0x0004 | 0x0005 => Some(PICMnemonic::SUBWF),
                    0x0006 | 0x0007 => Some(PICMnemonic::DECF),
                    0x0008 | 0x0009 => Some(PICMnemonic::IORWF),
                    0x000A | 0x000B => Some(PICMnemonic::ANDWF),
                    0x000C | 0x000D => Some(PICMnemonic::XORWF),
                    0x000E | 0x000F => Some(PICMnemonic::ADDWF),
                    0x0010 | 0x0011 => Some(PICMnemonic::MOVF),
                    0x0012 | 0x0013 => Some(PICMnemonic::COMF),
                    0x0014 | 0x0015 => Some(PICMnemonic::INCF),
                    0x0016 | 0x0017 => Some(PICMnemonic::DECF),
                    0x0018 | 0x0019 => Some(PICMnemonic::RRF),
                    0x001A | 0x001B => Some(PICMnemonic::RLF),
                    0x001C | 0x001D => Some(PICMnemonic::SWAPF),
                    0x001E | 0x001F => Some(PICMnemonic::INCFSZ),
                    _ => Some(PICMnemonic::UND),

               }
            }
            PICCategory::BitOperation => {
                match self.instruction_raw.bitwise_and_with_16(0x3000).as_u16() {
                    0x0000 => Some(PICMnemonic::BCF),
                    0x1000 => Some(PICMnemonic::BSF),
                    0x2000 => Some(PICMnemonic::BTFSC),
                    0x3000 => Some(PICMnemonic::BTFSS),
                    _ => Some(PICMnemonic::UND)
                }
            }
            PICCategory::ControlTransfer => {
                match self.instruction_raw.bitwise_and_with_16(0x3000).as_u16() {
                    0x0000 => Some(PICMnemonic::RETLW),
                    0x1000 => Some(PICMnemonic::CALL),
                    0x2000 => Some(PICMnemonic::GOTO),
                    _ => Some(PICMnemonic::UND)

                }
            }
            PICCategory::Miscellaneous => {
                match self.instruction_raw.bitwise_and_with_16(0x001F).as_u16() {
                    0x0000 => Some(PICMnemonic::NOP),
                    0x0002 => Some(PICMnemonic::OPTION),
                    0x0003 => Some(PICMnemonic::SLEEP),
                    0x0004 => Some(PICMnemonic::CLRWDT),
                    0x0005..=0x07 => Some(PICMnemonic::TRIS),
                    0x0010..=0x17 => Some(PICMnemonic::MOVLB),
                    0x001E => Some(PICMnemonic::RETURN),
                    0x001F => Some(PICMnemonic::RETFIE),
                    _ => Some(PICMnemonic::UND)

            }
        }
        PICCategory::OperationsWithW => {
            match self.instruction_raw.bitwise_and_with_16(0x0300).as_u16() {
                0x0000 => Some(PICMnemonic::MOVLW),
                0x0100 => Some(PICMnemonic::IORLW),
                0x2000 => Some(PICMnemonic::ANDLW),
                0x3000 => Some(PICMnemonic::XORLW),
                _ => Some(PICMnemonic::UND)

                }
            }
        }
    }
}