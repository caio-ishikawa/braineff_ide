use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Operations { 
    IncrementPtr,
    DecrementPtr,
    IncrementByte,
    DecrementByte,
    Read,
    Write,
    StartLoop,
    EndLoop
}

// Reads file and returns list of chars containing the brainfuck commands //
#[exec_time]
pub fn lexer(file_name: &String) -> Vec<char> {
    let code_vector: Vec<char> = file_name.chars().filter(|&n| n == '>' || n == '<' || n == '+' || n == '-' || n == '[' || n == ']' || n == '.' || n == ',').collect();
    //parse(code_vector);
    return code_vector;
}

// Turns list of brainfuck commands into a vector of type Operations //
#[exec_time]
pub fn parse(brain_code: Vec<char>) -> Vec<Operations> { 
    let mut code: Vec<Operations> = Vec::new();

    for i in brain_code { 
        let operation = match i { 
            '+' => Operations::IncrementByte,
            '-' => Operations::DecrementByte,
            '>' => Operations::IncrementPtr,
            '<' => Operations::DecrementPtr,
            '.' => Operations::Read,
            ',' => Operations::Write, 
            '[' => Operations::StartLoop,
            ']' => Operations::EndLoop,
            _ => Operations::EndLoop,
        };
        code.push(operation);
    }
    return code;
    //compile(code);
}

// Runs code and prints the memory tape for debugging purposes || WILL CHANGE LATER //
#[exec_time]
pub fn compile(code: Vec<Operations>) -> (String, Vec<u8>) {
    let mut memory: Vec<u8> = vec![0; 30000];
    let mut mem_ptr: usize = 0;
    let mut code_ptr: usize = 0;
    let mut bracket_idx: Vec<usize> = Vec::new();
    let mut output: Vec<u8> = Vec::new();

    while code_ptr < code.len() { 
        let command = code[code_ptr]; 

        match command { 
            Operations::IncrementByte => memory[mem_ptr] = memory[mem_ptr].wrapping_add(1),
            Operations::DecrementByte => memory[mem_ptr] = memory[mem_ptr].wrapping_sub(1),
            Operations::IncrementPtr => mem_ptr += 1,
            Operations::DecrementPtr => mem_ptr -= 1, 
            Operations::Read => output.push(memory[mem_ptr]),   
            Operations::StartLoop => bracket_idx.push(code_ptr), 
            Operations::EndLoop => { 
                if memory[mem_ptr] != 0 {
                    code_ptr = *bracket_idx.last().unwrap()
                }
                else {
                    bracket_idx.pop();
                }
            }, 
            _ => println!("ERROR") 
        };
        code_ptr += 1;
    }
    //println!("{:?}", memory);
    let output:String = log_ptr(output);
    return (output, memory);
}

pub fn run_debug(code: Vec<Operations>) -> String { 
    let (output, memory) = compile(vec![code[0]]);
    println!("{:?}", memory);
    let mut memory_output = String::new();
    for i in memory{ 
        memory_output.push_str(i.to_string().as_str());
    }
    return memory_output;
}

#[exec_time]
fn log_ptr(byte: Vec<u8>) -> String { 
    let int_as_char: Vec<char> = byte.iter().map(|&n| n as char).collect();
    let output_as_str: String = int_as_char.into_iter().collect();
    println!("OUTPUT: {:?}", output_as_str);
    return output_as_str.to_string();
}
