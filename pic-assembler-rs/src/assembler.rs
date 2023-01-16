use crate::nbitnumber::{u12, u9, super, NumberOperations};
use crate::PIC10F200;

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
    //Paurse the line of PIC code
    

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