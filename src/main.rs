use std::env;

pub mod bytecode;
pub mod codegen;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("No arguments provided");
        return;
    }

    let parsed_bytecode = bytecode::from_file(&args[0]);
    match parsed_bytecode {
        Err(e) => println!("Error: {}", e),
        Ok(parsed_bytecode) => {

            match codegen::x86_64::codegen(&parsed_bytecode) {
                Ok(_) => println!("Codegen successful"),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
