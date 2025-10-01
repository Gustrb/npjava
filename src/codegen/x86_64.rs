use crate::{bytecode::attribute::CodeInstruction, codegen::Assembly};

#[derive(Debug, Default)]
pub struct DataSection {
    offset: usize,
    elements: Vec<String>,
}

pub fn codegen(parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    let mut asm = Assembly::new();

    let mut ds = DataSection::default();

    for method in &parsed_bytecode.methods {
        let name = parsed_bytecode
            .constant_pool
            .find_utf8_constant_pool_entry(method.name_index)?;
        if name.bytes != "main" {
            continue;
        }

        asm.emit_section_text();
        asm.emit_global_main();
        asm.emit_function_start("_main");

        let descriptor = parsed_bytecode
            .constant_pool
            .find_utf8_constant_pool_entry(method.descriptor_index)?;

        for attribute in &method.attributes {
            let name = parsed_bytecode
                .constant_pool
                .find_utf8_constant_pool_entry(attribute.name_index)?;

            if name.bytes == "Code" {
                let code_attribute = attribute.into_code_attribute()?;
                let code_instructions = code_attribute.into_code_instructions()?;

                for instruction in code_instructions {
                    match instruction {
                        CodeInstruction::Ldc(index) => emit_ldc(&mut asm, index, parsed_bytecode, &mut ds)?,
                        CodeInstruction::Aload0 => todo!("implement aload0"),
                        CodeInstruction::InvokeVirtual(index) => emit_invoke_virtual(&mut asm, index, parsed_bytecode)?,
                        CodeInstruction::InvokeSpecial(_) => todo!("implement invoke special"),
                        CodeInstruction::GetStatic(_) => {},
                        CodeInstruction::Return => emit_ret(&mut asm, parsed_bytecode)?,
                    }
                }
            }
        }
    }

    // Emit data section

    asm.emit_section_data();
    asm.emit_global_data_section_elements();
    asm.emit_data_section_elements();

    for element in ds.elements {
        asm.emit_db(&element);
    }

    println!("{}", asm.code.join(""));

    Ok(())
}

fn emit_ldc(asm: &mut Assembly, index: u8, parsed_bytecode: &crate::bytecode::ParsedBytecode, ds: &mut DataSection) -> Result<(), String> {
    asm.emit_mov("rsi", &format!("qword [data_section_elements + {}]", ds.offset));
    ds.offset += 8;

    let str = parsed_bytecode.constant_pool.find_string_constant_pool_entry(index.into())?;
    ds.elements.push(parsed_bytecode.constant_pool.find_utf8_constant_pool_entry(str.string_index.into())?.bytes.clone());

    
    asm.emit_mov("rdx", &parsed_bytecode.constant_pool.find_utf8_constant_pool_entry(str.string_index.into())?.bytes.len().to_string());
    Ok(())
}

fn emit_invoke_virtual(asm: &mut Assembly, index: u16, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_call("runtime$println");

    Ok(())
}

fn emit_ret(asm: &mut Assembly, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_mov("rax", "0x2000001");
    asm.emit_mov("rdi", "0");
    asm.emit_syscall();

    Ok(())
}
