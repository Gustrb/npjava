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
        self.code.push(format!("mov {}, {}\n", dest, src));
    }

    pub fn emit_call(&mut self, name: &str) {
        self.code.push(format!("call {}\n", name));
    }

    pub fn emit_syscall(&mut self) {
        self.code.push("syscall\n".to_string());
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
        self.code.push(format!("db \"{}\", 10\n", value));
    }
}
