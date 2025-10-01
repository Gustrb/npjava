use crate::bytecode::endianness::{BigEndianByteOrder, ByteOrder};

#[derive(Debug)]
pub struct Attribute {
    pub name_index: u16,
    pub length: u32,
    pub info: Vec<u8>,
}

pub fn parse_attribute(bytecode: &Vec<u8>, mut offset: usize) -> Result<(Attribute, usize), String> {
    let name_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    let length = BigEndianByteOrder::read_u32(bytecode, offset)?;
    offset += 4;

    let info = bytecode[offset..offset + length as usize].to_vec();
    offset += length as usize;

    Ok((Attribute { name_index, length, info }, offset))
}
