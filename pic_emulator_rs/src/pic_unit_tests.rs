mod test {

    use crate::nbitnumber::*;
    use crate::data_memory::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_u12() {
        let test_u12 = u12::new(0x0FFF);
        assert_eq!(test_u12.get(), 0x0FFF);
    }


    #[test]
    fn test_u12_bitwise_and_with_16() {
        let test_u12 = u12::new(0x0FFF);
        let test_u16 = 0x0FFF;
        let result = test_u12.bitwise_and_with_16(test_u16);
        assert_eq!(result.get(), 0x0FFF);
    }

    #[test]
    fn test_u12_bitwise_and_with_32() {
        let test_u12 = u12::new(0x0FFF);
        let test_u32 = 0x0FFF;
        let result = test_u12.bitwise_and_with_32(test_u32);
        assert_eq!(result.get(), 0x0FFF);
    }


    #[test]
    fn test_u9() {
        let test_u9 = u9::new(0x1FF);
        assert_eq!(test_u9.get(), 0x1FF);
    }


    #[test]
    fn test_max_function() {
        let max_u12 = u12::new(0xFFF);
        let max_u12_const = u12::get_max();
        assert_eq!(max_u12.get(), max_u12_const.get());
    }


    // test indirect addressing
    #[test]
    fn test_indirect_addressing() {
        let last_general_purpose_register = 0x1F;
        let random_data = 0x38;

        let mut register_file = RegisterFile::new();

        register_file.write(last_general_purpose_register, random_data);
        register_file.write(SpecialPurposeRegisters::FSR as u8, last_general_purpose_register);


        assert_eq!(register_file.read(SpecialPurposeRegisters::INDF as u8), register_file.read(last_general_purpose_register));
           
    }
    
}