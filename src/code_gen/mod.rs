#![allow(unused_doc_comments, dead_code)]

use std::collections::HashMap;

use crate::parsing::ast::AssASTTypes;
struct Variable {
    ram_index: u64,
}
#[derive(Debug)]
pub struct CodeGenerationError {
    pub type_of_error: CodeGenerationErrorType,
    pub instruction_num: usize,
}
#[derive(Debug)]
pub enum CodeGenerationErrorType {
    VariableDoesntExist { name: String },
    LabelDoesntExist { name: String },
}
pub fn generate_code(parsed_types: Vec<AssASTTypes>) -> Vec<RawInstructions> {
    let mut already_existing_variables_registers: HashMap<String, Variable> = HashMap::new();
    let mut already_existing_labels: HashMap<String, u64> = HashMap::new();
    let mut errors: Vec<CodeGenerationError> = vec![];
    let mut ram_registry_index: u64 = calculate_variable_offset(&parsed_types);
    let mut raw_instructions: Vec<RawInstructions> = vec![];
    parsed_types
        .iter()
        .enumerate()
        .for_each(|(line, ast_type)| match ast_type {
            /// LENGTH = 2
            AssASTTypes::Define { name, a } => {
                let var_index =
                    if let Some(varible) = already_existing_variables_registers.get(&name.0) {
                        varible.ram_index
                    } else {
                        ram_registry_index += 1;
                        ram_registry_index - 1
                    };
                already_existing_variables_registers.insert(
                    name.0.clone(),
                    Variable {
                        ram_index: var_index,
                    },
                );
                raw_instructions.push(RawInstructions::ADD { a: *a, b: 0 });
                raw_instructions.push(RawInstructions::WRITE { a: var_index });
            }
            /// LENGTH = 4
            AssASTTypes::Add { a, b, c } => {
                let a_var = already_existing_variables_registers.get_key_value(&a.0);
                let b_var = already_existing_variables_registers.get_key_value(&b.0);
                let c_var = already_existing_variables_registers.get_key_value(&c.0);
                let mut errored = false;
                if a_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: a.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if b_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: b.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if c_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: c.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if errored {
                    return;
                }
                let a = a_var.unwrap();
                let b = b_var.unwrap();
                let c = c_var.unwrap();
                raw_instructions.push(RawInstructions::COPY {
                    a: a.1.ram_index,
                    b: ((raw_instructions.len() + 2) * 4 + 1) as u64,
                });
                raw_instructions.push(RawInstructions::COPY {
                    a: b.1.ram_index,
                    b: ((raw_instructions.len() + 1) * 4 + 2) as u64,
                });
                raw_instructions.push(RawInstructions::ADD {
                    a: a.1.ram_index,
                    b: b.1.ram_index,
                });
                raw_instructions.push(RawInstructions::WRITE { a: c.1.ram_index });
            }
            /// LENGTH = 4
            AssASTTypes::Subtract { a, b, c } => {
                let a_var = already_existing_variables_registers.get_key_value(&a.0);
                let b_var = already_existing_variables_registers.get_key_value(&b.0);
                let c_var = already_existing_variables_registers.get_key_value(&c.0);
                let mut errored = false;
                if a_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: a.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if b_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: b.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if c_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: c.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if errored {
                    return;
                }
                let a = a_var.unwrap();
                let b = b_var.unwrap();
                let c = c_var.unwrap();

                raw_instructions.push(RawInstructions::COPY {
                    a: a.1.ram_index,
                    b: ((raw_instructions.len() + 2) * 4 + 1) as u64,
                });
                raw_instructions.push(RawInstructions::COPY {
                    a: b.1.ram_index,
                    b: ((raw_instructions.len() + 1) * 4 + 2) as u64,
                });
                raw_instructions.push(RawInstructions::SUB {
                    a: a.1.ram_index,
                    b: b.1.ram_index,
                });
                raw_instructions.push(RawInstructions::WRITE { a: c.1.ram_index });
            }
            /// LENGTH = 4
            AssASTTypes::Multiply { a, b, c } => {
                let a_var = already_existing_variables_registers.get_key_value(&a.0);
                let b_var = already_existing_variables_registers.get_key_value(&b.0);
                let c_var = already_existing_variables_registers.get_key_value(&c.0);
                let mut errored = false;
                if a_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: a.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if b_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: b.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if c_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: c.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if errored {
                    return;
                }
                let a = a_var.unwrap();
                let b = b_var.unwrap();
                let c = c_var.unwrap();
                raw_instructions.push(RawInstructions::COPY {
                    a: a.1.ram_index,
                    b: ((raw_instructions.len() + 2) * 4 + 1) as u64,
                });
                raw_instructions.push(RawInstructions::COPY {
                    a: b.1.ram_index,
                    b: ((raw_instructions.len() + 1) * 4 + 2) as u64,
                });
                raw_instructions.push(RawInstructions::MUL {
                    a: a.1.ram_index,
                    b: b.1.ram_index,
                });
                raw_instructions.push(RawInstructions::WRITE { a: c.1.ram_index });
            }
            /// len = 1
            AssASTTypes::Exit { code } => {
                let a_var = already_existing_variables_registers.get_key_value(&code.0);
                if a_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: code.0.to_owned(),
                        },
                    });
                    return;
                }
                let a = a_var.unwrap();
                raw_instructions.push(RawInstructions::EXIT { a: a.1.ram_index })
            }
            /// len = 0
            AssASTTypes::LabelDefenition(label) => {
                already_existing_labels.insert(
                    label.0.to_owned(),
                    ((raw_instructions.len() + 1) * 4) as u64,
                );
            }
            // len = 1
            AssASTTypes::Jump { a } => {
                let a = already_existing_labels.get_key_value(&a.0);
                if a.is_none() {
                    panic!()
                }
                let (_, a) = a.unwrap();
                raw_instructions.push(RawInstructions::JMP { a: *a })
            }
            // len = 1
            AssASTTypes::Copy { a, b } => {
                let a = already_existing_variables_registers.get_key_value(&a.0);
                let b = already_existing_variables_registers.get_key_value(&b.0);
                if !(a.is_some() && b.is_some()) {
                    panic!()
                }
                let a = a.unwrap();
                let b = b.unwrap();

                raw_instructions.push(RawInstructions::COPY {
                    a: a.1.ram_index,
                    b: b.1.ram_index,
                })
            }
            // len = 1
            AssASTTypes::NotEqual { a, b, c } => {
                let a = already_existing_variables_registers.get_key_value(&a.0);
                let b = already_existing_variables_registers.get_key_value(&b.0);
                let c = already_existing_labels.get_key_value(&c.0);
                if !(a.is_some() && b.is_some() && c.is_some()) {
                    panic!()
                }
                let (_, a) = a.unwrap();
                let (_, b) = b.unwrap();
                let (_, c) = c.unwrap();
                raw_instructions.push(RawInstructions::NotEqual {
                    a: a.ram_index,
                    b: b.ram_index,
                    c: *c,
                })
            }
            // len = 1
            AssASTTypes::Equal { a, b, c } => {
                let a = already_existing_variables_registers.get_key_value(&a.0);
                let b = already_existing_variables_registers.get_key_value(&b.0);
                let c = already_existing_labels.get_key_value(&c.0);
                if !(a.is_some() && b.is_some() && c.is_some()) {
                    panic!()
                }
                let (_, a) = a.unwrap();
                let (_, b) = b.unwrap();
                let (_, c) = c.unwrap();
                raw_instructions.push(RawInstructions::Equal {
                    a: a.ram_index,
                    b: b.ram_index,
                    c: *c,
                })
            }
            AssASTTypes::Divide { a, b, c } => {
                let a_var = already_existing_variables_registers.get_key_value(&a.0);
                let b_var = already_existing_variables_registers.get_key_value(&b.0);
                let c_var = already_existing_variables_registers.get_key_value(&c.0);
                let mut errored = false;
                if a_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: a.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if b_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: b.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if c_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: c.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if errored {
                    return;
                }
                let a = a_var.unwrap();
                let b = b_var.unwrap();
                let c = c_var.unwrap();
                raw_instructions.push(RawInstructions::COPY {
                    a: a.1.ram_index,
                    b: ((raw_instructions.len() + 2) * 4 + 1) as u64,
                });
                raw_instructions.push(RawInstructions::COPY {
                    a: b.1.ram_index,
                    b: ((raw_instructions.len() + 1) * 4 + 2) as u64,
                });
                raw_instructions.push(RawInstructions::DIV {
                    a: a.1.ram_index,
                    b: b.1.ram_index,
                });
                raw_instructions.push(RawInstructions::WRITE { a: c.1.ram_index });
            }
            AssASTTypes::Yeet { a, b, c } => {
                let a_var = already_existing_variables_registers.get_key_value(&a.0);
                let b_var = already_existing_variables_registers.get_key_value(&b.0);
                let c_var = already_existing_variables_registers.get_key_value(&c.0);
                let mut errored = false;
                if a_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: a.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if b_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: b.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if c_var.is_none() {
                    errors.push(CodeGenerationError {
                        instruction_num: line,
                        type_of_error: CodeGenerationErrorType::VariableDoesntExist {
                            name: c.0.to_owned(),
                        },
                    });
                    errored = true;
                }
                if errored {
                    return;
                }
                let a = a_var.unwrap();
                let b = b_var.unwrap();
                let c = c_var.unwrap();
                raw_instructions.push(RawInstructions::COPY {
                    a: a.1.ram_index,
                    b: ((raw_instructions.len() + 2) * 4 + 1) as u64,
                });
                raw_instructions.push(RawInstructions::COPY {
                    a: b.1.ram_index,
                    b: ((raw_instructions.len() + 1) * 4 + 2) as u64,
                });
                raw_instructions.push(RawInstructions::COPY {
                    a: c.1.ram_index,
                    b: ((raw_instructions.len() + 1) * 4 + 3) as u64,
                });
                raw_instructions.push(RawInstructions::YEET {
                    a: a.1.ram_index,
                    b: b.1.ram_index,
                    c: c.1.ram_index,
                });
            }
            AssASTTypes::JumpIfHigherThan { a, b, c } => {
                let a = already_existing_variables_registers.get_key_value(&a.0);
                let b = already_existing_variables_registers.get_key_value(&b.0);
                let c = already_existing_labels.get_key_value(&c.0);
                if !(a.is_some() && b.is_some() && c.is_some()) {
                    panic!()
                }
                let (_, a) = a.unwrap();
                let (_, b) = b.unwrap();
                let (_, c) = c.unwrap();
                raw_instructions.push(RawInstructions::JHT {
                    a: a.ram_index,
                    b: b.ram_index,
                    c: *c,
                })
            }
        });
    if !errors.is_empty() {
        panic!("{:#?}", errors);
    }
    raw_instructions
}

fn calculate_variable_offset(instructions: &[AssASTTypes]) -> u64 {
    let mut calc = 0;
    instructions
        .iter()
        .for_each(|parsed_type| match parsed_type {
            AssASTTypes::Add { a: _, b: _, c: _ } => calc += 4,
            AssASTTypes::Define { name: _, a: _ } => {
                calc += 2;
            }
            AssASTTypes::Exit { code: _ } => {
                calc += 1;
            }
            AssASTTypes::Subtract { a: _, b: _, c: _ } => calc += 4,
            AssASTTypes::Multiply { a: _, b: _, c: _ } => calc += 4,
            AssASTTypes::LabelDefenition(_l) => {}
            AssASTTypes::Jump { a: _ } => calc += 1,
            AssASTTypes::Copy { a: _, b: _ } => calc += 1,
            AssASTTypes::NotEqual { a: _, b: _, c: _ } => calc += 1,
            AssASTTypes::Equal { a: _, b: _, c: _ } => calc += 1,
            AssASTTypes::Divide { a: _, b: _, c: _ } => calc += 4,
            AssASTTypes::Yeet { a: _, b: _, c: _ } => calc += 4,
            AssASTTypes::JumpIfHigherThan { a: _, b: _, c: _ } => calc += 4,
        });
    calc * 4
}
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum RawInstructions {
    ADD { a: u64, b: u64 },
    WRITE { a: u64 },
    COPY { a: u64, b: u64 },
    EXIT { a: u64 },
    SUB { a: u64, b: u64 },
    MUL { a: u64, b: u64 },
    JMP { a: u64 },
    Equal { a: u64, b: u64, c: u64 },
    NotEqual { a: u64, b: u64, c: u64 },
    DIV { a: u64, b: u64 },
    YEET { a: u64, b: u64, c: u64 },
    JHT { a: u64, b: u64, c: u64 },
}
pub fn raw_instructions_to_string(instructions: Vec<RawInstructions>) -> String {
    let mut string = String::new();
    instructions
        .iter()
        .for_each(|instruction| string.push_str(&raw_instruction_to_string(instruction)));
    string
}
pub fn raw_instruction_to_string(instruction: &RawInstructions) -> String {
    let mut str = String::new();
    match instruction {
        RawInstructions::ADD { a, b } => {
            str.push_str("101\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str
        }
        RawInstructions::WRITE { a } => {
            str.push_str("103\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str.push_str("0\n");
            str
        }
        RawInstructions::COPY { a, b } => {
            str.push_str("102\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str
        }
        RawInstructions::EXIT { a } => {
            str.push_str("109\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str.push_str("0\n");
            str
        }
        RawInstructions::SUB { a, b } => {
            str.push_str("105\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str
        }
        RawInstructions::MUL { a, b } => {
            str.push_str("106\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str
        }
        RawInstructions::JMP { a } => {
            str.push_str("104\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str.push_str("0\n");
            str
        }
        RawInstructions::Equal { a, b, c } => {
            str.push_str("107\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str(c.to_string().as_str());
            str.push('\n');
            str
        }
        RawInstructions::NotEqual { a, b, c } => {
            str.push_str("108\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str(c.to_string().as_str());
            str.push('\n');
            str
        }
        RawInstructions::DIV { a, b } => {
            str.push_str("112\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str("0\n");
            str
        }
        RawInstructions::YEET { a, b, c } => {
            str.push_str("111\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str(c.to_string().as_str());
            str.push('\n');
            str
        }
        RawInstructions::JHT { a, b, c } => {
            str.push_str("110\n");
            str.push_str(a.to_string().as_str());
            str.push('\n');
            str.push_str(b.to_string().as_str());
            str.push('\n');
            str.push_str(c.to_string().as_str());
            str.push('\n');
            str
        }
    }
}
