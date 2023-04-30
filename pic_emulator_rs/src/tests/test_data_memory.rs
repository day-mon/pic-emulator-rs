#[cfg(test)]
mod test {
    use crate::data_memory::{RegisterFile, SpecialPurposeRegisters};
    use crate::nbitnumber::*;

    /// Test indirect addressing
    #[test]
    pub fn test_indirect_addressing() {
        let last_general_purpose_register = 0x1F;
        let random_data = 0x38;

        let mut register_file = RegisterFile::new();

        register_file.write(u5::new(last_general_purpose_register), random_data);
        register_file.write(u5::new(SpecialPurposeRegisters::FSR as u16), last_general_purpose_register as u8);


        assert_eq!(
            register_file.read(u5::new(SpecialPurposeRegisters::INDF as u16)),
            register_file.read(u5::new(last_general_purpose_register as u16))
        );
           
    }

    #[test]
    pub fn test_empty_register() {
        let register_file = RegisterFile::new();
        // 0x10 starts general purpose registers
        assert_eq!(register_file.read(u5::new(0x10)), 0x00);
    }

    #[test]
    pub fn test_unsupported_register_read() {
        let register_file = RegisterFile::new();
        // 0x08 is the first unsupported register
        assert_eq!(register_file.read(u5::new(0x08)), 0x00);
    }

    #[test]
    pub fn test_unsupported_regsister_write() {
        let mut register_file = RegisterFile::new();
        register_file.write(u5::new(0x08), 0x01);
        // Writing should have had no effect
        // Impossible to test whether the value was actually changed
        // because register file is private, but a user should not be able 
        // to read a new value at minimum.
        assert_eq!(register_file.read(u5::new(0x08)), 0x00);
    }
    
    #[test]
    pub fn test_register_write() {
        let mut register_file = RegisterFile::new();
        register_file.write(u5::new(0x10), 0xBEu8);
        assert_eq!(register_file.read(u5::new(0x10)), 0xBEu8);
    }

    #[test]
    pub fn test_multiple_register_write() {
        let mut register_file = RegisterFile::new();
        register_file.write(u5::new(0x10), 0xBEu8);
        register_file.write(u5::new(0x11), 0xEFu8);
        assert_eq!(register_file.read(u5::new(0x10)), 0xBEu8);
        assert_eq!(register_file.read(u5::new(0x11)), 0xEFu8);
    }

    #[test]
    pub fn test_register_overwrite() {
        let mut register_file = RegisterFile::new();
        register_file.write(u5::new(0x10), 0xBEu8);
        register_file.write(u5::new(0x10), 0xEFu8);
        assert_eq!(register_file.read(u5::new(0x10)), 0xEFu8);
    }

    #[test]
    pub fn test_flash() {
        let mut register_file = RegisterFile::new();
        register_file.write(u5::new(0x10), 0xEFu8);
        register_file.write(u5::new(0x0F), 0xBEu8);
        register_file.flash();
        assert_eq!(register_file.read(u5::new(0x10)), 0x00u8);
        assert_eq!(register_file.read(u5::new(0x0F)), 0x00u8);
    }
}