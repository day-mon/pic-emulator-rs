use crate::data_memory::SpecialPurposeRegisters;
use crate::nbitnumber::{u12, u9, NumberOperations, NBitNumber};
use crate::pic::PIC10F200;



pub fn HALT(pic: &mut PIC10F200) -> () {
    todo!()
}
// Miscellaneous

pub fn NOP(pic: &mut PIC10F200) -> () {
    return;
}

pub fn OPTION(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn SLEEP(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn CLRWDT(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn TRIS(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn MOVLB(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn RETURN(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn RETFIE(pic: &mut PIC10F200) -> () {
    todo!()
}

// ALU Operation

pub fn MOVWF(pic: &mut PIC10F200) -> () {
    // f <- W
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
}

pub fn CLR(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn SUBWF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn DECF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn IORWF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn ANDWF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn XORWF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn ADDWF(pic: &mut PIC10F200) -> () {
    // dest ← f+W 
    let w = pic.w_register;
    
}

pub fn MOVF(pic: &mut PIC10F200) -> () {
    // The contents of register ‘f’ are
    // moved to destination ‘d’. If ‘d’ is ‘0’,
    // destination is the W register. If ‘d’
    // is ‘1’, the destination is file
    // register ‘f’. ‘d’ = 1 is useful as a
    // test of a file register, since status
    // flag Z is affected.
    
    // Move the contents of the f register to the
    // dest register
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let d = instruction.extract_d();
    let f_value = pic.data_memory.read(f);

    if d.as_u16() == 0 {
        pic.w_register = f_value;
    } else {
        pic.data_memory.write(f.as_u16() as u8, f_value);
    }
    
}

pub fn COMF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn INCF(pic: &mut PIC10F200) -> () {
   // let register = pic.current_instruction.extract_f();
   // pic.data_memory.read(r)
}

pub fn DECFSZ(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn RRF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn RLF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn SWAPF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn INCFSZ(pic: &mut PIC10F200) -> () {
    todo!()
}

// Bit Operation

pub fn BCF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn BSF(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn BTFSC(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn BTFSS(pic: &mut PIC10F200) -> () {
    todo!()
}

// Control Transfers

pub fn GOTO(pic: &mut PIC10F200) -> () {
    // Set the program counter PC to 
    // the 9-bit address specified by the instruction
    // at k using instruction.extract_k_goto()
    let instruction = pic.current_instruction;
    let k = instruction.extract_k_goto();
    pic.program_counter = k;
}

pub fn CALL(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn RETLW(pic: &mut PIC10F200) -> () {
    todo!()
}

// Operations with W

pub fn MOVLW(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn IORLW(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn ANDLW(pic: &mut PIC10F200) -> () {
    todo!()
}

pub fn XORLW(pic: &mut PIC10F200) -> () {
    todo!()
}