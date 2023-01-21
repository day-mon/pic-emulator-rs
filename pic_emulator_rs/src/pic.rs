use crate::{nbitnumber::{u12, u9, BitwiseOperations, NumberOperations, NBitNumber}, data_memory::RegisterFile, program_memory::ProgramMemory, data_memory::SpecialPurposeRegisters, instructions::*};

#[derive(Clone, Copy)]
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


trait TuringMachine {
    fn fetch(&mut self) -> ();
    fn execute(&mut self) -> ();
    fn tick(&mut self) -> ();
    fn decode_mnemonic(&mut self) -> ();
}

trait Programmable {
    fn program_chip(&mut self, new_program: [u12; 0x200]) -> ();
}

impl Programmable for PIC10F200 {
    fn program_chip(&mut self, new_program: [u12; 0x200]) -> () {
        self.program_memory.flash(new_program);
        self.data_memory.flash();
    }
}

impl TuringMachine for PIC10F200 {
    fn fetch(&mut self) -> () {
        //this is just a temprory variable, not the actual PC register
        let PCL = self.data_memory.read(SpecialPurposeRegisters::PCL as u8);

        //translate PC to u9 (we might want to sign extend it for off chip memory
        self.program_counter = u9::new(PCL as u16);
        self.current_instruction = PICInstruction::from_u12(self.program_memory.fetch(self.program_counter));
    }

    fn execute(&mut self) -> () {
        //start the pipeline
        self.decode_mnemonic();
    }

    fn tick(&mut self) -> () {
        //Execute first, per the pipeline flow
        self.execute(); //the first cycle should skip execution, AKA when PCL == RESET_VECTOR
        self.fetch();
    }

    fn decode_mnemonic(&mut self) ->  ()
    {
        match self.current_instruction.intsruction_category {
            PICCategory::ALUOperation => {
                match (self.current_instruction.instruction_raw.bitwise_and_with_16(0x3C0).as_u16()) >> 6 {
                    //4 bit opcode 9 downto 6, right shifted by 6
                    0x000 => MOVEWF(self),
                    0x001 => CLR(self),
                    0x002 => SUBWF(self),
                    0x003 => DECF(self),
                    0x004 => IORWF(self),
                    0x005 => ANDWF(self),
                    0x006 => XORWF(self),
                    0x007 => ADDWF(self),
                    0x008 => MOVF(self),
                    0x009 => COMF(self),
                    0x00A => INCF(self),
                    0x00B => DECF(self),
                    0x00C => RRF(self),
                    0x00D => RLF(self),
                    0x00E => SWAPF(self),
                    0x00F => INCFSZ(self),
                    _ => HALT(self) //There should not be any undefined ALU opearations
                }
            }
            PICCategory::BitOperation => {
                match self.current_instruction.instruction_raw.bitwise_and_with_16(0x300).as_u16() {
                    //2 bit op code bits 9 & 8
                    0x000 => BCF(self),
                    0x100 => BSF(self),
                    0x200 => BTFSC(self),
                    0x300 => BTFSS(self),
                    _ => HALT(self),
                }
            }
            PICCategory::ControlTransfer => {
                match self.current_instruction.instruction_raw.bitwise_and_with_16(0x300).as_u16() {
                    //2 bit opcode bits 9 & 8
                    0x000 => RETLW(self),
                    0x100 => CALL(self),
                    0x200 | 0x300 => GOTO(self),
                    _ => HALT(self)
                }
            }
            PICCategory::Miscellaneous => {
                //5 bit opcode 4 downto 0
                match self.current_instruction.instruction_raw.bitwise_and_with_16(0x01F).as_u16() {
                    0x000 => NOP(self),
                    0x002 => OPTION(self),
                    0x003 => SLEEP(self),
                    0x004 => CLRWDT(self),
                    0x005..=0x007 => TRIS(self),
                    0x010..=0x017 => MOVLB(self),
                    0x001E => RETURN(self),
                    0x001F => RETFIE(self),
                    _ => HALT(self),
                }
            }
            PICCategory::OperationsWithW => {
                match self.current_instruction.instruction_raw.bitwise_and_with_16(0x300).as_u16() {
                    //2 bit opcode 9 & 8
                    0x000 => MOVLW(self),
                    0x100 => IORLW(self),
                    0x200 => ANDLW(self),
                    0x300 => XORLW(self),
                    _ => HALT(self),
                }
            }
        };
    }
}

//Highest level wrapper of the MCU
pub struct PIC10F200 {
    data_memory : RegisterFile,
    program_memory : ProgramMemory,
    program_counter : u9,
    current_instruction : PICInstruction,
}


pub struct PICInstruction  {
    instruction_raw: u12,
    //instruction: Option<PICMnemonic>,
    intsruction_category: PICCategory,
}

impl PICInstruction {
    pub fn from_u12(instruction: u12) -> PICInstruction {
        let mut pic_instruction = PICInstruction {
            instruction_raw: instruction,
            intsruction_category: PICInstruction::decode_category(instruction),
        };    
        //pic_instruction

        return pic_instruction;
    }

    fn decode_category(instruction: u12) -> PICCategory {
        return match instruction.bitwise_and_with_32(0xC000).as_u16() {
            // misc & alu -> 0000 | 0000 | 0000
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
            _ => panic!("TODO")
        };
    }

    }

