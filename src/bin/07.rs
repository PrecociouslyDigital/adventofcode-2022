use advent_of_code::helpers::err::{self, TokenError};
use itertools::Itertools;
use std::{error, collections::{HashMap, HashSet}, borrow::Borrow};

pub fn part_one(input: &str) -> Result<u128, impl error::Error> {
    let mut cwd: Vec<&str> = vec![];
    let mut folders: HashMap<String, u128> = HashMap::new();
    let mut folders_seen: HashSet<String> = HashSet::new();

    let not_empty = |a : &&str| !a.is_empty();

    for command in input.split("$").filter(not_empty){
        let mut command_lines = command.split("\n").filter(not_empty);
        match command_lines.next() {
            Some(user_input) => {
                let parts = user_input.split(" ").filter(not_empty).collect_vec();
                match parts[0] {
                    "cd" => {
                        match parts[1] {
                            ".." => {
                                if cwd.len() == 0 {
                                    return Err(TokenError { token: user_input.to_string(), reason: "can't return from root".to_string() })
                                }else {
                                    cwd.pop();
                                }
                            },
                            "/" => cwd.clear(),
                            new_dir => cwd.push(new_dir)
                        }
                    }
                    "ls" => {
                        let cwd_full = cwd.join("/");
                        if folders_seen.contains(&cwd_full){
                            break;
                        }
                        folders_seen.insert(cwd_full);
                        for line in command_lines {
                            let line_parts = line.split(" ").collect_vec();
                            if line_parts[0] == "dir" {
                                folders.entry(line_parts[1].to_string()).or_insert(0);
                            } else {
                                match line_parts[0].parse::<u128>() {
                                    Ok(size) => {                              
                                        let mut full_folder = String::new();          
                                        for folder in cwd.iter() {
                                            full_folder += "/";
                                            full_folder += folder;
                                            *folders.entry(full_folder.clone()).or_insert(0) += size;
                                        }
                                    }
                                    Err(e) => return Err(TokenError{
                                        token: line_parts[0].to_string(),
                                        reason: format!("{:?}",e.kind())
                                    })
                                }
                            }
                        }
                    },
                    other => {
                        return Err(TokenError {
                            token: other.to_string(), reason: "Not a valid command!".to_string()})
                    }
                }
            }
            None => {return Err(TokenError{
                token: command.to_string(),
                reason: "missing user input".to_string(),
            })}
        }
    }

    return Ok(folders.values()
        .filter(|v| **v <= 100000u128)
        .map(|v| *v)
        .fold(0u128, std::ops::Add::add));
    
}

pub fn part_two(input: &str) -> Result<u128, impl error::Error> {
    let mut cwd: Vec<&str> = vec![];
    let mut folders: HashMap<String, u128> = HashMap::new();
    let mut folders_seen: HashSet<String> = HashSet::new();

    let not_empty = |a : &&str| !a.is_empty();

    for command in input.split("$").filter(not_empty){
        let mut command_lines = command.split("\n").filter(not_empty);
        match command_lines.next() {
            Some(user_input) => {
                let parts = user_input.split(" ").filter(not_empty).collect_vec();
                match parts[0] {
                    "cd" => {
                        match parts[1] {
                            ".." => {
                                if cwd.len() == 0 {
                                    return Err(TokenError { token: user_input.to_string(), reason: "can't return from root".to_string() })
                                }else {
                                    cwd.pop();
                                }
                            },
                            "/" => cwd.clear(),
                            new_dir => cwd.push(new_dir)
                        }
                    }
                    "ls" => {
                        let cwd_full = cwd.join("/");
                        if folders_seen.contains(&cwd_full){
                            break;
                        }
                        folders_seen.insert(cwd_full);
                        for line in command_lines {
                            let line_parts = line.split(" ").collect_vec();
                            if line_parts[0] == "dir" {
                                folders.entry(line_parts[1].to_string()).or_insert(0);
                            } else {
                                match line_parts[0].parse::<u128>() {
                                    Ok(size) => {                              
                                        let mut full_folder = String::new(); 
                                        *folders.entry("/".to_string()).or_insert(0) += size;   
                                        for folder in cwd.iter() {
                                            full_folder += "/";
                                            full_folder += folder;
                                            *folders.entry(full_folder.clone()).or_insert(0) += size;
                                        }
                                    }
                                    Err(e) => return Err(TokenError{
                                        token: line_parts[0].to_string(),
                                        reason: format!("{:?}",e.kind())
                                    })
                                }
                            }
                        }
                    },
                    other => {
                        return Err(TokenError {
                            token: other.to_string(), reason: "Not a valid command!".to_string()})
                    }
                }
            }
            None => {return Err(TokenError{
                token: command.to_string(),
                reason: "missing user input".to_string(),
            })}
        }
    }

    let current_free = 70000000 - folders["/"];

    let mut potential_folders = folders.values()
        .filter(|v| (**v + current_free >= 30000000u128)).sorted();

    match potential_folders.next() {
        Some(v) => return Ok(*v),
        None => return Err(TokenError { token: input.to_string(), reason: "no folder is large enough for delete".to_string() })
    }
    
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 7)
        };
    }
    #[test]
    fn test_part_one() {
        part_one(get_input!()).expect("First set of inputs resolves to a value.");
    }

    #[test]
    fn test_part_two() {
        //part_two(get_input!()).expect("Second set of inputs resolves to a value.");
    }
}

