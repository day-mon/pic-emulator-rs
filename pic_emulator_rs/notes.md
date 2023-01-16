```rs
fn decode(U12: instruction)
// {
//     // use only last two bits to 
//     // 0000 | 0000 | 0000 | 0000
//     // 1100 | 0000 | 0000 | 0000

//     match (U12 & 0xC000) {
//         0x0000 => {
//             match !U12 & 0x03E0 {
//                 0 => {
//                     //miscellanous instructions
//                 }
//                 _ => {
//                     //ALU operations
//                 }
//             }
//         }
//         // 000X | XXXX | XXXX | XXXX
//         PICInstruction::BitOperation => {
//             //Bit operations
//         }
//         //  010X | XXXX | XXXX | XXXX
//         0x4000 => {
//             //Control transfers
//         }
//         //  1000 | 0000 | 0000 | 0000
        
//         0x8000 => {
//             //Operations with W
//         }
//         0xC000 => {

//         }
//     }


    

//     //if instruction[11] NAND insturction[10]
//         //if insturction [bit 9 downto bit 5] = 00000
//         //AKA if ( (instruction & ) == 0 )
        
//             //Miscellanous instructions 
//                 //if (instruction & 0x03E0) == 0
//                 //state = 1
//         //else
//             //ALU operations
//             //The only difference between these two is 
//                 //state = 2
    
//     //if !instruction[11] && instruction[10]
//         //Bit operations
//             //state 3

//     //if instruction[11] && !instruction[10]
//         //Control transfers
//             //state 4

//     //if insturction[11] && instruction[10]
//         //Operations with W
//             //state 5
// }


  //0 1 1 1 | 1 1 1 1| ...
    let u2_ max = (1 << 2) - 1;
    // u128
    // (1 << 127) - 1
    // 0 0 0 1
    // 1 1 1 1 = 1 | 0 0 0 1 - 1
    // (1 << 4) - 1
    // 1 << 3
    // 1 0 0 0
```

