use crate::{bytecode::attribute::CodeInstruction, codegen::Assembly};

fn reconstruct_double(high_bytes: u32, low_bytes: u32) -> f64 {
    // Combine high and low bytes into a 64-bit integer
    let combined = ((high_bytes as u64) << 32) | (low_bytes as u64);
    // Convert to f64 using bit manipulation
    f64::from_bits(combined)
}

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

                let mut instruction_index = 0;
                for instruction in code_instructions {
                    match instruction {
                        CodeInstruction::Ldc(index, offset) => emit_ldc(&mut asm, index, parsed_bytecode, offset, &mut ds)?,
                        CodeInstruction::Aload0(offset) => todo!("implement aload0"),
                        CodeInstruction::InvokeVirtual(index, offset) => emit_invoke_virtual(&mut asm, index, offset, parsed_bytecode)?,
                        CodeInstruction::InvokeSpecial(index, offset) => todo!("implement invoke special"),
                        CodeInstruction::GetStatic(index, offset) => emit_get_static(&mut asm, index, offset, parsed_bytecode)?,
                        CodeInstruction::Return(offset) => emit_ret(&mut asm, offset, parsed_bytecode)?,
                        CodeInstruction::InvokeStatic(index, offset) => emit_invoke_static(&mut asm, index, offset, parsed_bytecode)?,
                        CodeInstruction::Ldc2W(index, offset) => emit_ldc2w(&mut asm, index, offset, parsed_bytecode, &mut ds)?,
                        CodeInstruction::Ifge(offset, off) => emit_ifge(&mut asm, offset, off, parsed_bytecode)?,
                        CodeInstruction::Dcmpg(offset) => emit_dcmpg(&mut asm, offset, parsed_bytecode)?,
                        CodeInstruction::Goto(offset, off) => emit_goto(&mut asm, offset, off, parsed_bytecode)?,
                        _ => todo!("implement {:?}", instruction),
                    }
                    
                    instruction_index += 1;
                }
            }
        }
    }

    asm.emit_runtime_println();

    // Emit data section
    asm.emit_section_data();
    asm.emit_global_data_section_elements();
    asm.emit_data_section_elements();

    for element in ds.elements {
        if element.starts_with("double_") {
            asm.emit_line(&element);
        } else {
            asm.emit_db(&element);
        }
    }

    println!("{}", asm.code.join(""));

    Ok(())
}

fn emit_ldc(asm: &mut Assembly, index: u8, parsed_bytecode: &crate::bytecode::ParsedBytecode, offset: usize, ds: &mut DataSection) -> Result<(), String> {
    let str = parsed_bytecode.constant_pool.find_string_constant_pool_entry(index.into())?;
    ds.elements.push(parsed_bytecode.constant_pool.find_utf8_constant_pool_entry(str.string_index.into())?.bytes.clone());

    asm.emit_line(&format!("L{}:", offset));

    asm.emit_push(&format!("qword {}", ds.elements.last().unwrap().len()));
    asm.emit_lea("r10", &format!("[rel data_section_elements + {}]", ds.offset));
    asm.emit_push("r10");

    ds.offset += 8;
    
    Ok(())
}

fn emit_ldc2w(asm: &mut Assembly, index: u16, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode, ds: &mut DataSection) -> Result<(), String> {
    // For now lets just support double values
    let dv = parsed_bytecode.constant_pool.find_double_constant_pool_entry(index.into())?;
    
    // Convert the high and low bytes back to a double value
    let double_value = reconstruct_double(dv.high_bytes, dv.low_bytes);
    ds.elements.push(format!("double_{}: .double {}", ds.offset / 8, double_value));

    asm.emit_line(&format!("L{}:", offset));
    asm.emit_push(&format!("qword [rel data_section_elements + {}]", ds.offset));

    ds.offset += 8;

    Ok(())
}

fn emit_dcmpg(asm: &mut Assembly, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", offset));
    asm.emit_call("runtime$dcmpg");
    Ok(())
}

fn emit_invoke_virtual(asm: &mut Assembly, index: u16, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", offset));
    asm.emit_call("runtime$println");
    Ok(())
}

fn emit_invoke_static(asm: &mut Assembly, index: u16, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", offset));
    asm.emit_call("runtime$drandom");
    Ok(())
}

fn emit_ifge(asm: &mut Assembly, branch_offset: u16, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", offset));

    // For now, we'll use relative jumps - you can fix this later
    asm.emit_jge(&format!("L{}", offset + branch_offset as usize));
    Ok(())
}

fn emit_goto(asm: &mut Assembly, offset: u16, off: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", off));
    // For now, we'll use relative jumps - you can fix this later  
    asm.emit_jmp(&format!("L{}", off + offset as usize));
    Ok(())
}

fn emit_ret(asm: &mut Assembly, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", offset));

    asm.emit_mov("rax", "0x2000001");
    asm.emit_mov("rdi", "0");
    asm.emit_syscall();

    Ok(())
}

fn emit_get_static(asm: &mut Assembly, index: u16, offset: usize, parsed_bytecode: &crate::bytecode::ParsedBytecode) -> Result<(), String> {
    asm.emit_line(&format!("L{}:", offset));
    asm.emit_line(";; todo: implement get static");
    Ok(())
}
