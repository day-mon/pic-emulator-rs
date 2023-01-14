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
                match (self.instruction_raw.bitwise_and_with_16(0x3C0).as_u16()) >> 6 {
                    //4 bit opcode 9 downto 6
                    0x000=> Some(PICMnemonic::MOVEWF),
                    0x001 => Some(PICMnemonic::CLR),
                    0x002 => Some(PICMnemonic::SUBWF),
                    0x003 => Some(PICMnemonic::DECF),
                    0x004 => Some(PICMnemonic::IORWF),
                    0x005 => Some(PICMnemonic::ANDWF),
                    0x006 => Some(PICMnemonic::XORWF),
                    0x007 => Some(PICMnemonic::ADDWF),
                    0x008 => Some(PICMnemonic::MOVF),
                    0x009 => Some(PICMnemonic::COMF),
                    0x00A => Some(PICMnemonic::INCF),
                    0x00B => Some(PICMnemonic::DECF),
                    0x00C => Some(PICMnemonic::RRF),
                    0x00D => Some(PICMnemonic::RLF),
                    0x00E => Some(PICMnemonic::SWAPF),
                    0x00F => Some(PICMnemonic::INCFSZ),
                    _ => Some(PICMnemonic::UND), //There should not be any undefined ALU opearations

               }
            }
            PICCategory::BitOperation => {
                match self.instruction_raw.bitwise_and_with_16(0x300).as_u16() {
                    //2 bit op code bits 9 & 8
                    0x000 => Some(PICMnemonic::BCF),
                    0x100 => Some(PICMnemonic::BSF),
                    0x200 => Some(PICMnemonic::BTFSC),
                    0x300 => Some(PICMnemonic::BTFSS),
                    _ => Some(PICMnemonic::UND)
                }
            }
            PICCategory::ControlTransfer => {
                match self.instruction_raw.bitwise_and_with_16(0x300).as_u16() {
                    //2 bit opcode bits 9 & 8
                    0x000 => Some(PICMnemonic::RETLW),
                    0x100 => Some(PICMnemonic::CALL),
                    0x200 | 0x300 => Some(PICMnemonic::GOTO),
                    _ => Some(PICMnemonic::UND)

                }
            }
            PICCategory::Miscellaneous => {
                //5 bit opcode 4 downto 0
                match self.instruction_raw.bitwise_and_with_16(0x01F).as_u16() {
                    0x000 => Some(PICMnemonic::NOP),
                    0x002 => Some(PICMnemonic::OPTION),
                    0x003 => Some(PICMnemonic::SLEEP),
                    0x004 => Some(PICMnemonic::CLRWDT),
                    0x005..=0x007 => Some(PICMnemonic::TRIS),
                    0x010..=0x017 => Some(PICMnemonic::MOVLB),
                    0x001E => Some(PICMnemonic::RETURN),
                    0x001F => Some(PICMnemonic::RETFIE),
                    _ => Some(PICMnemonic::UND)

            }
        }
        PICCategory::OperationsWithW => {
            match self.instruction_raw.bitwise_and_with_16(0x300).as_u16() {
                //2 bit opcode 9 & 8
                0x000 => Some(PICMnemonic::MOVLW),
                0x100 => Some(PICMnemonic::IORLW),
                0x200 => Some(PICMnemonic::ANDLW),
                0x300 => Some(PICMnemonic::XORLW),
                _ => Some(PICMnemonic::UND)

                }
            }
        }
    }
}