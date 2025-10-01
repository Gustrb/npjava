use crate::bytecode::endianness::{BigEndianByteOrder, ByteOrder};

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    ClassInfo(ClassInfoConstantPoolEntry),
    Fieldref(FieldrefConstantPoolEntry),
    Methodref(MethodrefConstantPoolEntry),
    InterfaceMethodref(InterfaceMethodrefConstantPoolEntry),
    String(StringConstantPoolEntry),
    Integer(IntegerConstantPoolEntry),
    Float(FloatConstantPoolEntry),
    Long(LongConstantPoolEntry),
    Double(DoubleConstantPoolEntry),
    NameAndType(NameAndTypeConstantPoolEntry),
    Utf8(Utf8ConstantPoolEntry),
    MethodHandle(MethodHandleConstantPoolEntry),
    MethodType(MethodTypeConstantPoolEntry),
    InvokeDynamic(InvokeDynamicConstantPoolEntry)
}

#[derive(Debug, Clone)]
pub struct ClassInfoConstantPoolEntry {
    pub tag: u8,
    pub name_index: u16,
}

#[derive(Debug, Clone)]
pub struct FieldrefConstantPoolEntry {
    pub tag: u8,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct MethodrefConstantPoolEntry {
    pub tag: u8,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct InterfaceMethodrefConstantPoolEntry {
    pub tag: u8,
    pub class_index: u16,
    pub name_and_type_index: u16,
}
    
#[derive(Debug, Clone)]
pub struct StringConstantPoolEntry {
    pub tag: u8,
    pub string_index: u16,
}

#[derive(Debug, Clone)]
pub struct IntegerConstantPoolEntry {
    pub tag: u8,
    pub bytes: u32,
}

#[derive(Debug, Clone)]
pub struct FloatConstantPoolEntry {
    pub tag: u8,
    pub bytes: u32,
}

#[derive(Debug, Clone)]
pub struct LongConstantPoolEntry {
    pub tag: u8,
    pub low_bytes: u32,
    pub high_bytes: u32,
}

#[derive(Debug, Clone)]
pub struct DoubleConstantPoolEntry {
    pub tag: u8,
    pub low_bytes: u32,
    pub high_bytes: u32,
}

#[derive(Debug, Clone)]
pub struct NameAndTypeConstantPoolEntry {
    pub tag: u8,
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct Utf8ConstantPoolEntry {
    pub tag: u8,
    pub length: u16,
    pub bytes: String,
}

#[derive(Debug, Clone)]
pub struct MethodHandleConstantPoolEntry {
    pub tag: u8,
    pub reference_kind: u8,
    pub reference_index: u16,
}

#[derive(Debug, Clone)]
pub struct MethodTypeConstantPoolEntry {
    pub tag: u8,
    pub descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct InvokeDynamicConstantPoolEntry {
    pub tag: u8,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

const CONSTANT_UTF8: u8 = 1;
const CONSTANT_CLASS_INFO: u8 = 7;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_FIELD_REF: u8 = 9;
const CONSTANT_METHOD_REF: u8 = 10;
const CONSTANT_NAME_AND_TYPE: u8 = 12;

pub fn parse_class_info_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let name_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    Ok((ConstantPoolEntry::ClassInfo(ClassInfoConstantPoolEntry {
        tag: CONSTANT_CLASS_INFO,
        name_index,
    }), offset))
}

pub fn parse_method_ref_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let class_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;
    let name_and_type_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    Ok((ConstantPoolEntry::Methodref(MethodrefConstantPoolEntry {
        tag: CONSTANT_METHOD_REF,
        class_index,
        name_and_type_index,
    }), offset))
}

pub fn parse_name_and_type_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let name_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;
    let descriptor_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    Ok((ConstantPoolEntry::NameAndType(NameAndTypeConstantPoolEntry {
        tag: CONSTANT_NAME_AND_TYPE,
        name_index,
        descriptor_index,
    }), offset))
}

pub fn parse_utf8_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let length = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    let bytes = String::from_utf8(bytecode[offset..offset + length as usize].to_vec()).map_err(|e| e.to_string())?;
    offset += length as usize;

    Ok((ConstantPoolEntry::Utf8(Utf8ConstantPoolEntry {
        tag: CONSTANT_UTF8,
        length,
        bytes,
    }), offset))
}

pub fn parse_field_ref_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let class_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;
    let name_and_type_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    Ok((ConstantPoolEntry::Fieldref(FieldrefConstantPoolEntry {
        tag: CONSTANT_FIELD_REF,
        class_index,
        name_and_type_index,
    }), offset))
}

pub fn parse_string_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let string_index = BigEndianByteOrder::read_u16(bytecode, offset)?;
    offset += 2;

    Ok((ConstantPoolEntry::String(StringConstantPoolEntry {
        tag: CONSTANT_STRING,
        string_index,
    }), offset))
}

#[derive(Debug, Default)]
pub struct ConstantPool {
    // TODO: Maybe use a BTreeMap instead of a Vec?
    pub entries: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn find_utf8_constant_pool_entry(&self, index: u16) -> Result<Utf8ConstantPoolEntry, String> {
        // Constant pool is 1-indexed, so we need to subtract 1 from the index
        let entry_index = (index as usize) - 1;
        
        if entry_index >= self.entries.len() {
            return Err(format!("Constant pool index out of bounds: {}", index));
        }
        
        if let ConstantPoolEntry::Utf8(entry) = &self.entries[entry_index] {
            Ok(entry.clone())
        } else {
            Err(format!("Constant pool entry at index {} is not a UTF8 entry", index))
        }
    }

    pub fn find_string_constant_pool_entry(&self, index: u16) -> Result<StringConstantPoolEntry, String> {
        // Constant pool is 1-indexed, so we need to subtract 1 from the index
        let entry_index = (index as usize) - 1;
        
        if entry_index >= self.entries.len() {
            return Err(format!("Constant pool index out of bounds: {}", index));
        }

        if let ConstantPoolEntry::String(entry) = &self.entries[entry_index] {
            Ok(entry.clone())
        } else {
            Err(format!("Constant pool entry at index {} is not a string entry", index))
        }
    }
}

pub fn parse_constant_pool_entry(bytecode: &Vec<u8>, mut offset: usize) -> Result<(ConstantPoolEntry, usize), String> {
    let tag = BigEndianByteOrder::read_u8(bytecode, offset)?;
    offset += 1;

    match tag {
        CONSTANT_CLASS_INFO => {
            return parse_class_info_constant_pool_entry(bytecode, offset);
        },
        CONSTANT_METHOD_REF => {
            return parse_method_ref_constant_pool_entry(bytecode, offset);
        },
        CONSTANT_NAME_AND_TYPE => {
            return parse_name_and_type_constant_pool_entry(bytecode, offset);
        },
        CONSTANT_UTF8 => {
            return parse_utf8_constant_pool_entry(bytecode, offset);
        },
        CONSTANT_FIELD_REF => {
            return parse_field_ref_constant_pool_entry(bytecode, offset);
        },
        CONSTANT_STRING => {
            return parse_string_constant_pool_entry(bytecode, offset);
        },
        _ => todo!("Implement parsing a constant pool entry, tag: {}", tag),
    }
}
