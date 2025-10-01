use std::env;

pub mod bytecode;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("No arguments provided");
        return;
    }

    let parsed_bytecode = bytecode::from_file(&args[0]);
    match parsed_bytecode {
        Ok(parsed_bytecode) => println!("{:?}", parsed_bytecode),
        Err(e) => println!("Error: {}", e),
    }
}
