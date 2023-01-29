use pic_emulator_rs::nbitnumber;
use pic_emulator_rs::pic;

use std::collections::HashMap;

pub enum PICCategory {
    Miscellaneous,
    BitOperation,
    ControlTransfer,
    OperationsWithW,
    ALUOperation,
}

pub struct INSTRUCTION {
    pub mnemonic: String,
    pub category: PICCategory,
    pub opcode: u12,
    pub operand1_length: NBitNumber,
}

//create implementation for the INSTRUCTION set
impl INSTRUCTION {
    pub fn new(mnemonic: String, category: PICCategory, opcode: u12, operand1_length: NBitNumber) -> INSTRUCTION {
        INSTRUCTION {
            mnemonic,
            category,
            opcode,
            operand1_length,
        }
    }
}



pub const INSTRUCTION_TABLE: HashMap<&str, u12> = [
    //Miscellaneous Instructions
    ("NOP", 0x000),
    ("OPTION", 0x002),
    ("SLEEP", 0x003),
    ("CLRWDT", 0x004),
    ("TRIS", 0x004),
    ("MOVLB", 0x010),
    ("RETURN", 0x01E),
    ("RETFIE", 0x01F),

    //ALU Operations
    ("MOVWF", 0x001 << 0x5),
    ("CLR", 0x001 << 0x6),
    ("SUBWF", 0x002 << 0x6),
    ("DECF", 0x003 << 0x6),
    ("IORWF", 0x004 << 0x6),
    ("ANDWF", 0x005 << 0x6),
    ("XORWF", 0x006 << 0x6),
    ("ADDWF", 0x007 << 0x6),
    ("MOVF", 0x008 << 0x6),
    ("COMF", 0x009 << 0x6),
    ("INCF", 0x00A << 0x6),
    ("DECFSZ", 0x00B << 0x6),
    ("RRF", 0x00C << 0x6),
    ("RLF", 0x00D << 0x6),
    ("SWAPF", 0x00E << 0x6),
    ("INCFSZ", 0x00F << 0x6),

    //Bit Operations
    ("BCF", 0x400),
    ("BSF", 0x500),
    ("BTFSC", 0x600),
    ("BTFSS", 0x700),

    //Control Transfer
    ("RETLW", 0x800),
    ("CALL", 0x900),
    ("GOTO", 0xA00),
].iter().cloned().collect();

//RETURN THE LENGTH OF THE FIRST OPERAND in bits
// pub const OPERAND_TABLE: HashMap <&str, u8>

pub fn assemble_pic_file(pic_file: &str) -> ([u12 ; 0x200]) {
    //let pic_file = read_pic(pic_file);
    let mut pic_program_data = [u12 ; 0x200];

    for line in pic_file {
        //Translate the line of PIC code into machine code and save it to the array
        let machine_code = encode(line);
        pic_program_data.push(machine_code);
    }

    // Return the machine code
    return pic_program_data;
}

//Translate a line of PIC code into machine code
pub fn encode(pic_line : &str) -> u12 {
    let PNEUMONIC = pic_line[0];
    let OPERAND1 = pic_line[1];
    let OPERAND2 = pic_line[2];
    
    //get the opcode from the instruction table
    let opcode = INSTRUCTION_TABLE[PNEUMONIC];

    //Add the first operand to the opcode
    let machine_code = opcode + OPERAND1;

    //Add the second operand to the opcode
    //the second operand should be shifted left by the length of the first operand
    machine_code = machine_code + OPERAND2 << 0;

    //Return the machine code
}

// pub fn read_pic(pic_file: &str) -> ([str ; 0x200]){
//     //Split the PIC code to 2D array of strings, deliminated lines and spaces
//     program_line : [str ; 0x200];
//     for line in pic_file {
//         program_line.push(line);
//     }

//     return pic_str;
// }

pub fn program_pic(pic_file: &str, pic : PIC10F200) -> () {
    //Generate the machine code
    assemble_picry_file(pic_file);

    //Upload the machine code to the PIC with flash()
    pic.program_chip();
}