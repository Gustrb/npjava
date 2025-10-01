use crate::bytecode::endianness::{BigEndianByteOrder, ByteOrder};

#[derive(Debug)]
pub struct Attribute {
    pub name_index: u16,
    pub length: u32,
    pub info: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Default)]
pub struct CodeAttribute {
    pub name_index: u16,
    pub length: u32,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

const LDC: u8 = 18;
const ALOAD_0: u8 = 42;
const INVOKE_VIRTUAL: u8 = 182;
const INVOKE_SPECIAL: u8 = 183;
const RETURN: u8 = 177;
const GET_STATIC: u8 = 178;

impl CodeAttribute {
    pub fn into_code_instructions(&self) -> Result<Vec<CodeInstruction>, String> {
        let mut code_instructions = Vec::new();
        let mut offset = 0;

        while offset < self.code_length as usize {
            let opcode = BigEndianByteOrder::read_u8(&self.code, offset)?;
            offset += 1;

            // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html#jvms-6.5
            match opcode {
                LDC => {
                    let index = BigEndianByteOrder::read_u8(&self.code, offset)?;
                    offset += 1;
                    code_instructions.push(CodeInstruction::Ldc(index));
                },
                ALOAD_0 => {
                    code_instructions.push(CodeInstruction::Aload0);
                },
                INVOKE_VIRTUAL => {
                    let index = BigEndianByteOrder::read_u16(&self.code, offset)?;
                    offset += 2;
                    code_instructions.push(CodeInstruction::InvokeVirtual(index));
                },
                INVOKE_SPECIAL => {
                    let index = BigEndianByteOrder::read_u16(&self.code, offset)?;
                    offset += 2;
                    code_instructions.push(CodeInstruction::InvokeSpecial(index));
                },
                RETURN => {
                    code_instructions.push(CodeInstruction::Return);
                },
                GET_STATIC => {
                    let index = BigEndianByteOrder::read_u16(&self.code, offset)?;
                    offset += 2;
                    code_instructions.push(CodeInstruction::GetStatic(index));
                },
                _ => todo!("Implement parsing a code instruction, opcode: {}", opcode),
            }
        }

        Ok(code_instructions)
    }
}

#[derive(Debug)]
pub enum CodeInstruction {
    Ldc(u8),
    Aload0,
    InvokeVirtual(u16),
    InvokeSpecial(u16),
    GetStatic(u16),
    Return,
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

impl Attribute {
    pub fn into_code_attribute(&self) -> Result<CodeAttribute, String> {
        let mut code_attribute = CodeAttribute::default();
        let mut offset = 0;

        code_attribute.name_index = self.name_index;
        code_attribute.length = self.length;

        code_attribute.max_stack = BigEndianByteOrder::read_u16(&self.info, offset)?;
        offset += 2;
        code_attribute.max_locals = BigEndianByteOrder::read_u16(&self.info, offset)?;
        offset += 2;
        code_attribute.code_length = BigEndianByteOrder::read_u32(&self.info, offset)?;
        offset += 4;

        code_attribute.code = self.info[offset..offset + code_attribute.code_length as usize].to_vec();
        offset += code_attribute.code_length as usize;

        code_attribute.exception_table_length = BigEndianByteOrder::read_u16(&self.info, offset)?;
        offset += 2;

        code_attribute.exception_table.reserve(code_attribute.exception_table_length as usize);
        for _ in 0..code_attribute.exception_table_length {
            let start_pc = BigEndianByteOrder::read_u16(&self.info, offset)?;
            offset += 2;
            let end_pc = BigEndianByteOrder::read_u16(&self.info, offset)?;
            offset += 2;
            let handler_pc = BigEndianByteOrder::read_u16(&self.info, offset)?;
            offset += 2;
            let catch_type = BigEndianByteOrder::read_u16(&self.info, offset)?;
            offset += 2;

            code_attribute.exception_table.push(ExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            });
        }

        code_attribute.attributes_count = BigEndianByteOrder::read_u16(&self.info, offset)?;
        offset += 2;

        code_attribute.attributes.reserve(code_attribute.attributes_count as usize);
        for _ in 0..code_attribute.attributes_count {
            let (attribute, attribute_offset) = parse_attribute(&self.info, offset)?;
            code_attribute.attributes.push(attribute);
            offset = attribute_offset;
        }

        Ok(code_attribute)
    }
}
