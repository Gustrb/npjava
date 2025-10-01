pub trait ByteOrder {
    fn read_u8(bytecode: &Vec<u8>, offset: usize) -> Result<u8, String>;
    fn read_u16(bytecode: &Vec<u8>, offset: usize) -> Result<u16, String>;
    fn read_u32(bytecode: &Vec<u8>, offset: usize) -> Result<u32, String>;
}

pub(crate) struct BigEndianByteOrder;

impl ByteOrder for BigEndianByteOrder {
    fn read_u8(bytecode: &Vec<u8>, offset: usize) -> Result<u8, String> {
        if offset + 1 > bytecode.len() {
            return Err(format!("Offset out of bounds: {}", offset));
        }
        Ok(bytecode[offset])
    }

    fn read_u16(bytecode: &Vec<u8>, offset: usize) -> Result<u16, String> {
        if offset + 2 > bytecode.len() {
            return Err(format!("Offset out of bounds: {}", offset));
        }

        let mut val: u16 = 0;
        for i in 0..2 {
            val |= (bytecode[offset + i] as u16) << (8 * (1 - i));
        }
        Ok(val)
    }

    fn read_u32(bytecode: &Vec<u8>, offset: usize) -> Result<u32, String> {
        if offset + 4 > bytecode.len() {
            return Err(format!("Offset out of bounds: {}", offset));
        }

        let mut val: u32 = 0;
        for i in 0..4 {
            val |= (bytecode[offset + i] as u32) << (8 * (3 - i));
        }
        Ok(val)
    }
}
