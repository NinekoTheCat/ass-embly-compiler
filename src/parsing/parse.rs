use std::str::Chars;

use super::{
    ast::{Address, AssASTTypes, Label},
    operators::{
        parse_1_arg_operator, parse_2_arg_operator, parse_2_arg_with_label_operator,
        parse_3_argument_operator, parse_def_operator, parse_jmp_argument_operator,
    },
};

pub fn parse_string(str: String) -> Vec<AssASTTypes> {
    let mut chars = str.chars();
    let mut string_instruction_storage = String::new();
    let mut asl: Vec<AssASTTypes> = vec![];
    while let Some(char) = chars.next() {
        match char {
            '\n' => {
                string_instruction_storage = String::new();
                continue;
            }
            '#' => {
                ignore_comment(&mut chars);
                continue;
            }
            _ => {}
        }
        print!("{:?}",char);
        string_instruction_storage.push(char);
        match string_instruction_storage.as_str() {
            "DEF" => {
                let def_operator_parsed = parse_def_operator(&mut chars);
                let def_operator_ast = AssASTTypes::Define {
                    name: Label(def_operator_parsed.name),
                    a: def_operator_parsed.value,
                };
                asl.push(def_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "ADD" => {
                let add_operator = parse_3_argument_operator(&mut chars);
                let add_operator_ast = AssASTTypes::Add {
                    a: Address(add_operator.a_name),
                    b: Address(add_operator.b_name),
                    c: Address(add_operator.c_name),
                };
                asl.push(add_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "SUB" => {
                let sub_operator = parse_3_argument_operator(&mut chars);
                let sub_operator_ast = AssASTTypes::Subtract {
                    a: Address(sub_operator.a_name),
                    b: Address(sub_operator.b_name),
                    c: Address(sub_operator.c_name),
                };
                asl.push(sub_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "DIV" => {
                let div_operator = parse_3_argument_operator(&mut chars);
                let div_operator_ast = AssASTTypes::Divide {
                    a: Address(div_operator.a_name),
                    b: Address(div_operator.b_name),
                    c: Address(div_operator.c_name),
                };
                asl.push(div_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "MUL" => {
                let mul_operator = parse_3_argument_operator(&mut chars);
                let mul_operator_ast = AssASTTypes::Multiply {
                    a: Address(mul_operator.a_name),
                    b: Address(mul_operator.b_name),
                    c: Address(mul_operator.c_name),
                };
                asl.push(mul_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "YEET" => {
                let yeet_operator = parse_3_argument_operator(&mut chars);
                let yeet_operator_ast = AssASTTypes::Yeet {
                    a: Address(yeet_operator.a_name),
                    b: Address(yeet_operator.b_name),
                    c: Address(yeet_operator.c_name),
                };
                asl.push(yeet_operator_ast);
                string_instruction_storage = String::new();
            }
            "JHT" => {
                let jht_operator = parse_2_arg_with_label_operator(&mut chars);
                let jht_operator_ast = AssASTTypes::JumpIfHigherThan {
                    a: Address(jht_operator.a_name),
                    b: Address(jht_operator.b_name),
                    c: Label(jht_operator.c_label_name),
                };
                asl.push(jht_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            ":" => {
                let label = parse_label(&mut chars);
                let label_ast = AssASTTypes::LabelDefenition(Label(label));
                asl.push(label_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "EXIT" => {
                let exit_operator = parse_1_arg_operator(&mut chars);
                let exit_operator_ast = AssASTTypes::Exit {
                    code: Address(exit_operator.name),
                };
                asl.push(exit_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "JMP" => {
                let jmp_operator = parse_jmp_argument_operator(&mut chars);
                let jmp_operator_ast = AssASTTypes::Jump {
                    a: Label(jmp_operator.label),
                };
                asl.push(jmp_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "JE" => {
                let je_operator = parse_2_arg_with_label_operator(&mut chars);
                let je_operator_ast = AssASTTypes::Equal {
                    a: Address(je_operator.a_name),
                    b: Address(je_operator.b_name),
                    c: Label(je_operator.c_label_name),
                };
                asl.push(je_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "JNE" => {
                let jne_operator = parse_2_arg_with_label_operator(&mut chars);
                let jne_operator_ast = AssASTTypes::NotEqual {
                    a: Address(jne_operator.a_name),
                    b: Address(jne_operator.b_name),
                    c: Label(jne_operator.c_label_name),
                };
                asl.push(jne_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }
            "CPY" => {
                let cpy_operator = parse_2_arg_operator(&mut chars);
                let cpy_operator_ast = AssASTTypes::Copy {
                    a: Address(cpy_operator.a_name),
                    b: Address(cpy_operator.b_name),
                };
                asl.push(cpy_operator_ast);
                string_instruction_storage = String::new();
                continue;
            }

            _ => {}
        };
    }
    asl
}
fn ignore_comment(chars: &mut Chars) {
    for char in chars.by_ref() {
        match char {
            '\n' => {
                return;
            }
            _ => {
                continue;
            }
        }
    }
}
fn parse_label(chars: &mut Chars) -> String {
    let mut accumulator: String = String::new();
    for char in chars.by_ref() {
        match char {
            '\n' => {
                break;
            }
            ' ' => {
                break;
            }
            _ => {
                accumulator.push(char);
                continue;
            }
        }
    }
    accumulator
}
