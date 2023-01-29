#[cfg(test)]
mod test {
    // Test max values for each type to ensure they can be read
    #[test]
    pub fn test_nbit_2_new_max() {
        let nbit2 = crate::nbitnumber::u2::new(0x3);
        assert_eq!(nbit2.get(), 0x3);
    }

    #[test]
    pub fn test_nbit_3_new_max(){
        let nbit3 = crate::nbitnumber::u3::new(0x7);
        assert_eq!(nbit3.get(), 0x7);
    }

    #[test]
    pub fn test_nbit_5_new_max() {
        let nbit5 = crate::nbitnumber::u5::new(0x1F);
        assert_eq!(nbit5.get(), 0x1F);
    }
    
    #[test]
    pub fn test_nbit_7_new_max() {
        let nbit7 = crate::nbitnumber::u7::new(0x7F);
        assert_eq!(nbit7.get(), 0x7F);
    }

    #[test]
    pub fn test_nbit_9_new_max() {
        let nbit9 = crate::nbitnumber::u9::new(0x1FF);
        assert_eq!(nbit9.get(), 0x1FF);
    }

    #[test]
    pub fn test_nbit_12_new_max() {
        let nbit12 = crate::nbitnumber::u12::new(0xFFF);
        assert_eq!(nbit12.get(), 0xFFF);
    }

    #[test]
    pub fn test_nbit_5_and() {
        let nbit5 = crate::nbitnumber::u5::new(0x1F);
        let res = nbit5.get() & 0x0Fu16;
        assert_eq!(res, 0x0F);
    }

    #[test]
    pub fn test_nbit_12_or() {
        let nbit12 = crate::nbitnumber::u12::new(0xFF0);
        let res = nbit12.get() | 0xFu16;
        assert_eq!(res, 0xFFF);
    }

    #[test]
    pub fn test_nbit_9_xor() {
        let nbit9 = crate::nbitnumber::u9::new(0x1FF);
        let res = nbit9.get() ^ 0x1FEu16;
        assert_eq!(res, 1);
    }

    #[test]
    pub fn test_nbit_9_too_big(){
        let nbit9 = crate::nbitnumber::u9::new(0x200);
        assert_eq!(nbit9.get(), 0);
    }

    #[test]
    pub fn test_nbit_12_and_32() {
        let nbit12 = crate::nbitnumber::u12::new(0xFFF);
        let res = nbit12.get() as u32 & 0x1Fu32;
        assert_eq!(res, 0x1Fu32);
    }

    #[test]
    pub fn test_nbit_12_and_usize() {
        let nbit12 = crate::nbitnumber::u12::new(0xFFF);
        let res = nbit12.get() as usize & 0x1F as usize;
        assert_eq!(res, 0x1F as usize);
    }

    #[test] 
    pub fn test_nbit_12_negate() {
        let nbit12 = crate::nbitnumber::u12::new(0xF0F);
        // Note that we need to wrap in a new() call to get the correct value
        // because bitwise negation over a wider bit width will result in a
        // bunch of 1s in the MSBs
        let res = crate::nbitnumber::u12::new(!nbit12.get());
        assert_eq!(res.get(), 0x0F0);
    }

    #[test]
    pub fn test_nbit_12_shl() {
        let nbit12 = crate::nbitnumber::u12::new(0x0F0);
        let res = nbit12.get() << 4;
        assert_eq!(res, 0xF00);
    }

    #[test]
    pub fn test_nbit_12_get_max() {
        use crate::nbitnumber::NumberOperations;
        assert_eq!(crate::nbitnumber::u12::get_max().get(), 0xFFF);
    }
}