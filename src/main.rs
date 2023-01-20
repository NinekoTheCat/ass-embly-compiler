use std::{fs};

use clap::Parser;
use log::debug;

use crate::{
    code_gen::{generate_code, raw_instructions_to_string},
    parsing::parse::parse_string,
};
mod code_gen;
mod parsing;
#[derive(Parser,Debug)]
struct Args {
    #[arg(short,default_value_t=false)]
    debug: bool,
    /// file to input
    file_input: String,
    /// file to output
    file_output: String
}
fn main() -> Result<(), u64> {
    
    let args = Args::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    pretty_env_logger::init();
    let input = replace_windows_with_unix(fs::read_to_string(args.file_input).unwrap());
    let parsed_string = parse_string(input);
    let generated_code = generate_code(parsed_string);
    if let Err(_) = generated_code {
        return Err(2);
    }
    let final_product = raw_instructions_to_string(generated_code.unwrap());
    println!("done");
    fs::write(args.file_output, final_product).unwrap();
    Ok(())
}

fn replace_windows_with_unix(string: String) -> String {
    string.replace("\r", "\n")
}
