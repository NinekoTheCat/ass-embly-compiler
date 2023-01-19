use std::{env, fs};

use crate::{
    code_gen::{generate_code, raw_instructions_to_string},
    parsing::parse::parse_string,
};
mod code_gen;
mod parsing;
fn main() -> Result<(), u64> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(1);
    }

    let file_to_read = args[1].to_owned();
    let file_to_write = args[2].to_owned();

    let input = fs::read_to_string(file_to_read).unwrap();
    let parsed_string = parse_string(input);
    let generated_code = generate_code(parsed_string);
    let final_product = raw_instructions_to_string(generated_code);
    println!("done");
    fs::write(file_to_write, final_product).unwrap();
    Ok(())
}
