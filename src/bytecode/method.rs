use crate::bytecode::endianness::{BigEndianByteOrder, ByteOrder};
use crate::bytecode::attribute::{self, Attribute};

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

pub fn parse_method(bytecode: &Vec<u8>, mut offset: usize) -> Result<(Method, usize), String> {
    let access_flags = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    let name_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    let descriptor_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    let attributes_count = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    let mut attrs = Vec::new();
    attrs.reserve(attributes_count as usize);

    for _ in 0..attributes_count {
        let (attribute, attribute_offset) = attribute::parse_attribute(bytecode, offset)?;
        attrs.push(attribute);
        offset = attribute_offset;
    }

    Ok((Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes: attrs,
    }, offset))
}
