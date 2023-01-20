use std::str::Chars;

pub struct ParsedDEF {
    pub name: String,
    pub value: u64,
}
pub fn parse_def_operator(chars: &mut Chars) -> ParsedDEF {
    let mut determined_if_defenition_is_constant_or_ram = false;
    let mut got_variable_label = false;
    let mut variable_label_storage: String = String::new();
    let mut number_string_storage: String = String::new();
    for char in chars.by_ref() {
        if !determined_if_defenition_is_constant_or_ram {
            match char {
                '?' => {
                    determined_if_defenition_is_constant_or_ram = true;
                }
                ' ' => {}
                _ => {
                    panic!()
                }
            }
            continue;
        }
        if !is_newline(char) && char != ' ' && !got_variable_label {
            variable_label_storage.push(char);
            continue;
        } else if char == ' ' && !got_variable_label {
            got_variable_label = true;
            continue;
        } else if !is_newline(char) && !got_variable_label && number_string_storage.is_empty() {
            panic!()
        }

        if !is_newline(char) && char.is_ascii_digit() {
            number_string_storage.push(char);
        } else if is_newline(char) || char == ' ' {
            return ParsedDEF {
                name: variable_label_storage,
                value: number_string_storage.parse().unwrap(),
            };
        }
    }
    panic!("{}, {}",variable_label_storage, number_string_storage)
}

pub struct Parsed1ArgOperator {
    pub name: String,
}
pub fn parse_1_arg_operator(chars: &mut Chars) -> Parsed1ArgOperator {
    let mut determined_if_defenition_is_constant_or_ram = false;
    let mut variable_label_storage: String = String::new();
    for char in chars.by_ref() {
        if !determined_if_defenition_is_constant_or_ram {
            match char {
                '?' => {
                    determined_if_defenition_is_constant_or_ram = true;
                }
                ' ' => {}
                _ => {
                    panic!()
                }
            }
            continue;
        }
        if !is_newline(char) && char != ' ' {
            variable_label_storage.push(char);
            continue;
        } else if (is_newline(char) || char == ' ') && variable_label_storage.is_empty() {
            panic!()
        } else if is_newline(char) || char == ' ' {
            return Parsed1ArgOperator {
                name: variable_label_storage,
            };
        }
    }
    Parsed1ArgOperator {
        name: variable_label_storage,
    }
}

pub struct Parsed3ArgOperator {
    pub a_name: String,
    pub b_name: String,
    pub c_name: String,
}
pub fn parse_3_argument_operator(chars: &mut Chars) -> Parsed3ArgOperator {
    let mut variable_label_storage = [String::new(), String::new(), String::new()];
    let mut index = 0;
    let mut found_first_variable = false;
    for char in chars.by_ref() {
        match char {
            '?' => {
                if found_first_variable {
                    index += 1;
                }

                found_first_variable = true;
            }
            ' ' => {
                if index == 2 {
                    break;
                }
            }
            '\n' => {
                if index == 2 {
                    break;
                }
            }
            _ => {
                if found_first_variable {
                    variable_label_storage[index].push(char);
                }
            }
        }
    }
    Parsed3ArgOperator {
        a_name: variable_label_storage[0].clone(),
        b_name: variable_label_storage[1].clone(),
        c_name: variable_label_storage[2].clone(),
    }
}
pub struct ParsedJMPOperator {
    pub label: String,
}
pub fn parse_jmp_argument_operator(chars: &mut Chars) -> ParsedJMPOperator {
    let mut determined_if_defenition_is_constant_or_ram = false;
    let mut variable_label_storage: String = String::new();
    for char in chars.by_ref() {
        if !determined_if_defenition_is_constant_or_ram {
            match char {
                ':' => {
                    determined_if_defenition_is_constant_or_ram = true;
                }
                ' ' => {}
                _ => {
                    panic!()
                }
            }
            continue;
        }
        if !is_newline(char) && char != ' ' {
            variable_label_storage.push(char);

            continue;
        } else if (is_newline(char) || char == ' ') && variable_label_storage.is_empty() {
            panic!()
        } else if is_newline(char) || char == ' ' {
            return ParsedJMPOperator {
                label: variable_label_storage,
            };
        }
    }
    if variable_label_storage.is_empty() {
        panic!();
    }
    ParsedJMPOperator {
        label: variable_label_storage,
    }
}

pub struct Parsed2ArgWithLabelOperator {
    pub a_name: String,
    pub b_name: String,
    pub c_label_name: String,
}
pub fn parse_2_arg_with_label_operator(chars: &mut Chars) -> Parsed2ArgWithLabelOperator {
    let mut variable_label_storage = [String::new(), String::new(), String::new()];
    let mut index = 0;
    let mut found_first_variable = false;
    for char in chars.by_ref() {
        match char {
            '?' => {
                if found_first_variable {
                    index += 1;
                }

                found_first_variable = true;
            }
            ':' => {
                if index != 1 {
                    panic!()
                }
                index += 1;
            }
            ' ' => {
                if index == 2 {
                    break;
                }
            }
            '\n' => {
                if index == 2 {
                    break;
                }
            }
            _ => {
                if found_first_variable {
                    variable_label_storage[index].push(char);
                }
            }
        }
    }
    Parsed2ArgWithLabelOperator {
        a_name: variable_label_storage[0].to_owned(),
        b_name: variable_label_storage[1].to_owned(),
        c_label_name: variable_label_storage[2].to_owned(),
    }
}
pub struct Parsed2ArgOperator {
    pub a_name: String,
    pub b_name: String,
}
pub fn parse_2_arg_operator(chars: &mut Chars) -> Parsed2ArgOperator {
    let mut variable_label_storage = [String::new(), String::new()];
    let mut index = 0;
    let mut found_first_variable = false;
    for char in chars.by_ref() {
        match char {
            '?' => {
                if found_first_variable {
                    index += 1;
                }

                found_first_variable = true;
            }
            ' ' => {
                if index == 1 {
                    break;
                }
            }
            '\n' => {
                if index == 1 {
                    break;
                }
            }
            _ => {
                if found_first_variable {
                    variable_label_storage[index].push(char);
                }
            }
        }
    }
    Parsed2ArgOperator {
        a_name: variable_label_storage[0].clone(),
        b_name: variable_label_storage[1].clone(),
    }
}

pub fn is_newline(character: char) -> bool {
    character == '\n'
}
