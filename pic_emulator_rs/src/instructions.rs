use crate::nbitnumber::{u12, u9, BitwiseOperations, NumberOperations, NBitNumber};
use crate::pic::PIC10F200;

trait Miscellaneous{
    fn extract_k() -> NBitNumber<3>{
        todo!()
    }

    fn extract_d() -> NBitNumber<1>{
        todo!()
    }
}

trait ALU_Operation{
    fn extract_d() -> NBitNumber<1>{
        todo!()
    }

    fn extract_f() -> NBitNumber<5>{
        todo!()
    }
}

trait Bit_Operation{
    fn extract_k() -> NBitNumber<3>{
        todo!()
    }

    fn extract_b() -> NBitNumber<3>{
        todo!()
    }
}

trait Control_Transfers{
    fn extract_k() -> NBitNumber<11>{
        todo!()
    }
}

trait Operations_with_W{
    fn extract_k() -> NBitNumber<8>{
        todo!()
    }
}

pub fn HALT(PIC: &PIC10F200) -> () {
    todo!()
}
// Miscellaneous

pub fn NOP(PIC: &PIC10F200) -> () {
    return;
}

pub fn OPTION(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn SLEEP(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn CLRWDT(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn TRIS(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn MOVLB(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn RETURN(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn RETFIE(PIC: &PIC10F200) -> () {
    todo!()
}

// ALU Operation

pub fn MOVEWF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn CLR(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn SUBWF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn DECF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn IORWF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn ANDWF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn XORWF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn ADDWF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn MOVF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn COMF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn INCF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn DECFSZ(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn RRF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn RLF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn SWAPF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn INCFSZ(PIC: &PIC10F200) -> () {
    todo!()
}

// Bit Operation

pub fn BCF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn BSF(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn BTFSC(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn BTFSS(PIC: &PIC10F200) -> () {
    todo!()
}

// Control Transfers

pub fn GOTO(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn CALL(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn RETLW(PIC: &PIC10F200) -> () {
    todo!()
}

// Operations with W

pub fn MOVLW(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn IORLW(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn ANDLW(PIC: &PIC10F200) -> () {
    todo!()
}

pub fn XORLW(PIC: &PIC10F200) -> () {
    todo!()
}