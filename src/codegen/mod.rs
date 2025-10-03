pub mod x86_64;

#[derive(Debug, Default)]
pub struct Assembly {
    pub code: Vec<String>,
}

impl Assembly {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit_section_text(&mut self) {
        self.code.push("section .text\n".to_string());
    }

    pub fn emit_global_main(&mut self) {
        self.code.push("global _main\n".to_string());
    }

    pub fn emit_function_start(&mut self, name: &str) {
        self.code.push(format!("{}:\n", name));
    }

    pub fn emit_mov(&mut self, dest: &str, src: &str) {
        self.code.push(format!("\tmov {}, {}\n", dest, src));
    }

    pub fn emit_push(&mut self, src: &str) {
        self.code.push(format!("\tpush {}\n", src));
    }

    pub fn emit_call(&mut self, name: &str) {
        self.code.push(format!("\tcall {}\n", name));
    }

    pub fn emit_syscall(&mut self) {
        self.code.push("\tsyscall\n".to_string());
    }

    pub fn emit_section_data(&mut self) {
        self.code.push("section .data\n".to_string());
    }

    pub fn emit_global_data_section_elements(&mut self) {
        self.code.push("global data_section_elements\n".to_string());
    }

    pub fn emit_data_section_elements(&mut self) {
        self.code.push("data_section_elements:\n".to_string());
    }

    pub fn emit_db(&mut self, value: &str) {
        self.code.push(format!("\tdb \"{}\", 10\n", value));
    }

    pub fn emit_lea(&mut self, dest: &str, src: &str) {
        self.code.push(format!("\tlea {}, {}\n", dest, src));
    }

    pub fn emit_runtime_println(&mut self) {
        self.code.push("\nruntime$println:\n".to_string());
        self.code.push("\nmov rsi, qword [rsp + 8]\n".to_string());
        self.code.push("\tmov rdx, qword [rsp + 16]\n".to_string());
        self.code.push("\tmov rax, 0x2000004\n".to_string());
        self.code.push("\tmov rdi, 1\n".to_string());
        self.code.push("\tsyscall\n".to_string());
        self.code.push("\tret\n".to_string());
        self.code.push("\n".to_string());
    }

    pub fn emit_jge(&mut self, label: &str) {
        self.code.push(format!("\tjge {}\n", label));
    }

    pub fn emit_jmp(&mut self, label: &str) {
        self.code.push(format!("\tjmp {}\n", label));
    }

    pub fn emit_line(&mut self, line: &str) {
        self.code.push(format!("\t{}\n", line));
    }

    pub fn emit_label(&mut self, label: &str) {
        self.code.push(format!("{}:\n", label));
    }
}
