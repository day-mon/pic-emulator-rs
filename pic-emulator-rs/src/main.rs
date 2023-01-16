//PIC10F200 Emulator
//https://web.archive.org/web/20150811030147/http://ww1.microchip.com/downloads/en/DeviceDoc/41239D.pdf
//http://ww1.microchip.com/downloads/en/DeviceDoc/41228C.pdf

//12 bit word

//SPECIAL REGISTERS
//K is a 8 or 9 bit constant (depending on the instruction type)
//C is the carry flag
//Z is the zero flag


//declare state

mod pic;
pub mod nbitnumber;
pub mod program_memory;
pub mod data_memory;
pub mod test;

// let PC = //should point to bootloader in ROM
fn main() {
    loop {
        // fetch_instruction();
        // in a real examble we would fetch here
        // let instruction_raw: U12 = 0x4111;
        // let instruction = PICInstruction::from_U12(instruction_raw);
        // let decoded_instruction = instruction.decode();
        // instruction.execute(decoded_instruction);
        // instruction.memory();
        // instruction.writeBack();
    }
}