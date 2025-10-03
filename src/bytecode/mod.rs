pub mod constantpool;
pub mod method;
pub mod attribute;
pub mod endianness;

use std::{fs::File, io::Read};
use crate::bytecode::attribute::Attribute;
use crate::bytecode::endianness::{BigEndianByteOrder, ByteOrder};
use crate::bytecode::constantpool::ConstantPool;
use crate::bytecode::method::Method;

#[derive(Debug, Default)]
pub struct ParsedBytecode {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub methods_count: u16,
    pub methods: Vec<Method>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

pub fn from_file(path: &str) -> Result<ParsedBytecode, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut output = Vec::new();

    // TODO: Use a buffer, no need to read the whole bytecode
    file.read_to_end(&mut output).map_err(|e| e.to_string())?;

    return parse_bytecode(&output);
}

pub fn parse_bytecode(bytecode: &Vec<u8>) -> Result<ParsedBytecode, String> {
    let mut parsed_bytecode = ParsedBytecode::default();
    let mut offset = 0;
    let magic = BigEndianByteOrder::read_u32(bytecode, offset)?;
    if magic != 0xCAFEBABE {
        return Err(format!("Invalid magic number: {}", magic));
    }
    offset += 4;

    parsed_bytecode.minor_version = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    parsed_bytecode.major_version = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    parsed_bytecode.constant_pool_count = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;
 
    offset = parse_constant_pool(&mut parsed_bytecode, bytecode, offset)?;

    parsed_bytecode.access_flags = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    parsed_bytecode.this_class = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    parsed_bytecode.super_class = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    parsed_bytecode.interfaces_count = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    offset = parse_interfaces(&mut parsed_bytecode, bytecode, offset)?;

    parsed_bytecode.fields_count = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    offset = parse_fields(&mut parsed_bytecode, bytecode, offset)?;

    parsed_bytecode.methods_count = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    offset = parse_methods(&mut parsed_bytecode, bytecode, offset)?;

    parsed_bytecode.attributes_count = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    parse_attributes(&mut parsed_bytecode, bytecode, offset)?;

    Ok(parsed_bytecode)
}

fn parse_constant_pool(parsed_bytecode: &mut ParsedBytecode, bytecode: &Vec<u8>, mut offset: usize) -> Result<usize, String> {
    // For some dumb reason, the constant pool count is 1 indexed.
    parsed_bytecode.constant_pool.entries.reserve(parsed_bytecode.constant_pool_count as usize - 1);

    let mut i = 1;
    while i < parsed_bytecode.constant_pool_count {
        let (entry, entry_offset) = constantpool::parse_constant_pool_entry(bytecode, offset)?;
        println!("Parsed constant pool entry: {:?}", entry);
        
        parsed_bytecode.constant_pool.entries.push(entry.clone());
        offset = entry_offset;
        
        // Double and Long entries take up two slots in the constant pool
        if matches!(entry, constantpool::ConstantPoolEntry::Double(_) | constantpool::ConstantPoolEntry::Long(_)) {
            parsed_bytecode.constant_pool.entries.push(constantpool::ConstantPoolEntry::Dummy);
            i += 1;
        }

        i += 1;
    }

    return Ok(offset);
}

fn parse_interfaces(parsed_bytecode: &mut ParsedBytecode, bytecode: &Vec<u8>, mut offset: usize) -> Result<usize, String> {
    parsed_bytecode.interfaces.reserve(parsed_bytecode.interfaces_count as usize);
    for _ in 0..parsed_bytecode.interfaces_count {
        let interface = BigEndianByteOrder::read_u16(bytecode, offset)?;
        parsed_bytecode.interfaces.push(interface);
        offset += 2;
    }

    return Ok(offset);
}

fn parse_fields(parsed_bytecode: &mut ParsedBytecode, bytecode: &Vec<u8>, mut offset: usize) -> Result<usize, String> {
    if parsed_bytecode.fields_count == 0 {
        return Ok(offset);
    }

    todo!("Implement parsing fields");
}

fn parse_methods(parsed_bytecode: &mut ParsedBytecode, bytecode: &Vec<u8>, mut offset: usize) -> Result<usize, String> {
    if parsed_bytecode.methods_count == 0 {
        return Ok(offset);
    }

    parsed_bytecode.methods.reserve(parsed_bytecode.methods_count as usize);
    for _ in 0..parsed_bytecode.methods_count {
        let (method, method_offset) = method::parse_method(bytecode, offset)?;
        parsed_bytecode.methods.push(method);
        offset = method_offset;
    }

    return Ok(offset);
}

fn parse_attributes(parsed_bytecode: &mut ParsedBytecode, bytecode: &Vec<u8>, mut offset: usize) -> Result<usize, String> {
    if parsed_bytecode.attributes_count == 0 {
        return Ok(offset);
    }

    parsed_bytecode.attributes.reserve(parsed_bytecode.attributes_count as usize);
    for _ in 0..parsed_bytecode.attributes_count {
        let (attribute, attribute_offset) = attribute::parse_attribute(bytecode, offset)?;
        parsed_bytecode.attributes.push(attribute);
        offset = attribute_offset;
    }

    return Ok(offset);
}

pub fn print_bytecode_methods(parsed_bytecode: &ParsedBytecode) -> Result<(), String> {
    for method in &parsed_bytecode.methods {
        let name = parsed_bytecode.constant_pool.find_utf8_constant_pool_entry(method.name_index)?;
        let descriptor = parsed_bytecode.constant_pool.find_utf8_constant_pool_entry(method.descriptor_index)?;
        println!("Method: {} {}", name.bytes, descriptor.bytes);

        for attribute in &method.attributes {
            let name = parsed_bytecode.constant_pool.find_utf8_constant_pool_entry(attribute.name_index)?;
            println!("Attribute: {}", name.bytes);

            if name.bytes == "Code" {
                let code_attribute = attribute.into_code_attribute()?;
                println!("Code attribute: {:?}", code_attribute);
                let code_instructions = code_attribute.into_code_instructions()?;
                println!("Code instructions: {:?}", code_instructions);
            }
        }
    }

    Ok(())
}
