use advent_of_code::helpers::err::{self, TokenError};
use enum_derive::EnumFromInner;
use itertools::Itertools;
use replace_with::replace_with_or_abort;
use std::error;
use std::fs;
use std::io;
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;
use custom_derive::custom_derive;
use std::collections::LinkedList;

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

#[derive(Debug, Clone)]
pub struct EmptyStackError;
#[derive(Debug, Clone)]

pub struct InvalidStackError {
    stack: usize,
    max: usize,
}
#[derive(Debug, Clone)]
pub struct MoveTooDeepError{
    stack_height: u32,
    stack: String,
}


impl std::fmt::Display for EmptyStackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Stack is entirely empty!")
    }
}
impl std::fmt::Display for InvalidStackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("Stack {} out of range (max: {})!", self.stack, self.max))
    }
}
impl std::fmt::Display for MoveTooDeepError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("Can't move {} containers; not enough! Stack is {}", self.stack_height, self.stack))
    }
}

custom_derive! {
    #[derive(EnumFromInner, Debug)]
    pub enum Error {
        EmptyStackError(EmptyStackError),
        Error(std::io::Error),
        TokenError(TokenError),
        InvalidStackError(InvalidStackError),
        MoveTooDeepError(MoveTooDeepError)
    }
}

impl error::Error for EmptyStackError {

}


fn get_stacks_from_text(input: Vec<String>) -> Result<Vec<Vec<String>>, Error> {
    let mut stacks: Vec<Vec<String>>;
    let mut lines = input.into_iter().rev();
    match lines.next() {
        Some(line) => {
            let line_parts = line.split(" ");
            stacks = Vec::with_capacity(line_parts.size_hint().0);
            for (i, token) in line_parts.enumerate() {
                if i % 3 == 1 {
                    if token != (1 + (i / 3)).to_string() {
                        return Err(Error::TokenError(TokenError {
                            token: token.to_string(),
                            reason: format!(
                                "Error parsing {line}, expecting {} but got {token}",
                                (i / 3) + 1
                            ),
                        }));
                    }
                    stacks.push(vec![]);
                } else {
                    if !token.is_empty() {
                        return Err(Error::TokenError(TokenError {
                            token: token.to_string(),
                            reason: format!(
                                "Error parsing {line}, expecting empty space but got {token}"
                            ),
                        }));
                    }
                }
            }
        }
        None => {
            return Err(Error::TokenError(TokenError {
                token: "".to_string(),
                reason: "expected stack layout, but file looks empty!".to_string(),
            }));
        }
    }
    while let Some(line) = lines.next() {
        for (i, c) in UnicodeSegmentation::graphemes(line.as_str(), true).enumerate() {
            if (i / 4) >= stacks.len() {
                return Err(Error::TokenError(TokenError {
                    token: line.to_string(),
                    reason: format!(
                        "container at {i} is outside the ship with length {}",
                        stacks.len()
                    ),
                }));
            }
            match i % 4 {
                0 => {
                    if c != "[" && c != " " {
                        return Err(Error::TokenError(TokenError {
                            token: c.to_string(),
                            reason: format!("Error reading {line}, expecting ["),
                        }));
                    }
                }
                1 => {
                    if c != " " {
                        stacks[i/4].push(c.to_string());
                    }
                }
                2 => {
                    if c != "]" && c != " " {
                        return Err(Error::TokenError(TokenError {
                            token: c.to_string(),
                            reason: format!("Error reading {line}, expecting ["),
                        }));
                    }
                }
                3 => {
                    if c != " " {
                        return Err(Error::TokenError(TokenError {
                            token: c.to_string(),
                            reason: format!("Error reading {line}, expecting space"),
                        }));
                    }
                }
                _ => panic!("hey wtf")
            }
        }
    }
    return Ok(stacks);
}

pub fn part_one<T: std::io::Read>(input: io::BufReader<T>) -> Result<String, Error> {
    let mut lines = input.lines();

    let mut starting_positions: Vec<String> = vec![];

    while let Some(stacks) = lines.next() {
        match stacks {
            Ok(stacks) => {
                if stacks.is_empty() {
                    break;
                }
                starting_positions.push(stacks.clone());
            },
            Err(stacks) => return Err(Error::Error(stacks))
,        }
    }
    let mut stacks = get_stacks_from_text(starting_positions)?;
    
    
    while let Some(line) = lines.next() {
        let line_ok = line?;
        let parts = line_ok.split(" ").collect_vec();
        if (parts.len() != 6) || (parts[0] != "move") || (parts[2] != "from") || (parts[4] != "to") {
            return Err(Error::TokenError(TokenError{
                token: line_ok.to_string(),
                reason: "expected a command of move _ from _ to _".to_string(),
            }));
        }
        match parts[1].parse::<u32>() {
            Ok(stack_height) => match parts[3].parse() {
                Ok(from_stack) => match parts[5].parse() {
                    Ok(to_stack) => {
                        println!("{},{},{}", stack_height, from_stack, to_stack);
                        println!("from_stack: {}", stacks[(from_stack - 1) as usize].iter().join(","));
                        println!("to_stack: {}", stacks[(to_stack-1)  as usize].iter().join(","));

                        if from_stack > stacks.len() {
                            return Err(Error::InvalidStackError(InvalidStackError{
                                stack: from_stack,
                                max: stacks.len(),
                            }));
                        }
                        if to_stack > stacks.len() {
                            return Err(Error::InvalidStackError(InvalidStackError{
                                stack: to_stack,
                                max: stacks.len(),
                            }));
                        }
                        if stack_height > 0 {
                            let split_index = stacks[from_stack-1].len() - stack_height as usize;
                            let new_stuff = stacks[from_stack-1].split_off(split_index);
                            stacks[to_stack-1].extend(new_stuff.iter().rev().map(String::from));

                            println!("from_stack_after: {}", stacks[(from_stack - 1) as usize].iter().join(","));
                            println!("to_stack_after: {}", stacks[(to_stack-1)  as usize].iter().join(","));

                        }
                    }
                    Err(e) => return Err(Error::TokenError(TokenError {
                        token: parts[5].to_string(),
                        reason: format!("couldn't read move amount because {:?}",e.kind()),
                    })),
                }
                Err(e) => return Err(Error::TokenError(TokenError {
                    token: parts[3].to_string(),
                    reason: format!("couldn't read move amount because {:?}",e.kind()),
                })),
            }, 
            Err(e) => return Err(Error::TokenError(TokenError {
                token: parts[1].to_string(),
                reason: format!("couldn't read move amount because {:?}",e.kind()),
            })),

        }
    }

    println!("{:?}", stacks);

    return Ok(stacks.iter().map(|a| a.last().unwrap()).join(""));
}

pub fn part_two<T: std::io::Read>(input: io::BufReader<T>) -> Result<String, Error> {
    let mut lines = input.lines();

    let mut starting_positions: Vec<String> = vec![];

    while let Some(stacks) = lines.next() {
        match stacks {
            Ok(stacks) => {
                if stacks.is_empty() {
                    break;
                }
                starting_positions.push(stacks.clone());
            },
            Err(stacks) => return Err(Error::Error(stacks))
,        }
    }
    let mut stacks = get_stacks_from_text(starting_positions)?;
    
    
    while let Some(line) = lines.next() {
        let line_ok = line?;
        let parts = line_ok.split(" ").collect_vec();
        if (parts.len() != 6) || (parts[0] != "move") || (parts[2] != "from") || (parts[4] != "to") {
            return Err(Error::TokenError(TokenError{
                token: line_ok.to_string(),
                reason: "expected a command of move _ from _ to _".to_string(),
            }));
        }
        match parts[1].parse::<u32>() {
            Ok(stack_height) => match parts[3].parse() {
                Ok(from_stack) => match parts[5].parse() {
                    Ok(to_stack) => {
                        println!("{},{},{}", stack_height, from_stack, to_stack);
                        println!("from_stack: {}", stacks[(from_stack - 1) as usize].iter().join(","));
                        println!("to_stack: {}", stacks[(to_stack-1)  as usize].iter().join(","));

                        if from_stack > stacks.len() {
                            return Err(Error::InvalidStackError(InvalidStackError{
                                stack: from_stack,
                                max: stacks.len(),
                            }));
                        }
                        if to_stack > stacks.len() {
                            return Err(Error::InvalidStackError(InvalidStackError{
                                stack: to_stack,
                                max: stacks.len(),
                            }));
                        }
                        if stack_height > 0 {
                            let split_index = stacks[from_stack-1].len() - stack_height as usize;
                            let new_stuff = stacks[from_stack-1].split_off(split_index);
                            stacks[to_stack-1].extend(new_stuff);

                            println!("from_stack_after: {}", stacks[(from_stack - 1) as usize].iter().join(","));
                            println!("to_stack_after: {}", stacks[(to_stack-1)  as usize].iter().join(","));

                        }
                    }
                    Err(e) => return Err(Error::TokenError(TokenError {
                        token: parts[5].to_string(),
                        reason: format!("couldn't read move amount because {:?}",e.kind()),
                    })),
                }
                Err(e) => return Err(Error::TokenError(TokenError {
                    token: parts[3].to_string(),
                    reason: format!("couldn't read move amount because {:?}",e.kind()),
                })),
            }, 
            Err(e) => return Err(Error::TokenError(TokenError {
                token: parts[1].to_string(),
                reason: format!("couldn't read move amount because {:?}",e.kind()),
            })),

        }
    }

    println!("{:?}", stacks);

    return Ok(stacks.iter().map(|a| a.last().unwrap()).join(""));
}
fn main() {
    use advent_of_code::{ANSI_BOLD, ANSI_RESET};
    let input1 = advent_of_code::open_file_buffer("inputs", 5).unwrap();
    let input2 = advent_of_code::open_file_buffer("inputs", 5).unwrap();
    println!("🎄 {ANSI_BOLD}Part {}{ANSI_RESET} 🎄", "1");
    println!("{}", part_one(input1).unwrap());
    println!("🎄 {ANSI_BOLD}Part {}{ANSI_RESET} 🎄", "2");    
    println!("{}", part_two(input2).unwrap());

}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            advent_of_code::open_file_buffer("inputs", 5)
        };
    }
    #[test]
    fn test_part_one() {
        part_one(get_input!().unwrap()).expect("First set of inputs resolves to a value.");
    }

    #[test]
    fn test_part_two() {
        part_two(get_input!().unwrap()).expect("Second set of inputs resolves to a value.");
    }
}
