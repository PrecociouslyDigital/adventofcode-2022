#![feature(iter_next_chunk)]

use advent_of_code::helpers::err::{self, TokenError};
use std::{error, collections::VecDeque};
use itertools::Itertools;

pub fn part_one(input: &str) -> Result<u32, impl error::Error> {
    const SEARCH_SIZE: usize = 3;
    let mut iter = unicode_segmentation::UnicodeSegmentation::graphemes(input, true);
    match iter.next_chunk::<SEARCH_SIZE>() {
        Ok(first) => {
            let mut last_seen = VecDeque::from(first);
            for (i, c) in iter.enumerate() {
                if last_seen.contains(&c) {
                    println!("{last_seen:?} contains {c}");
                    last_seen.pop_front();
                    last_seen.push_back(c);
                } else {
                    println!("{last_seen:?} does not contain {c}");
                    for n in 0..SEARCH_SIZE * SEARCH_SIZE {
                        if (n%SEARCH_SIZE != n/SEARCH_SIZE) && (last_seen[n%SEARCH_SIZE] == last_seen[n/SEARCH_SIZE]) {
                            println!("but does contain a dupe");
                            last_seen.pop_front();
                            last_seen.push_back(c);
                            break;
                        }
                        if n == SEARCH_SIZE * SEARCH_SIZE - 1 {
                            return Ok((i + SEARCH_SIZE + 1) as u32);
                        }
                    }
                }
            }
            return Err(TokenError{
                token:input.to_string(),
                reason: format!("no block of {SEARCH_SIZE} without repeats")
            });
        },
        Err(e) => {
            return Err(TokenError { token: input.to_string(), reason: "expected any characters at all".to_string() })
        }
        
    }
    
}

pub fn part_two(input: &str) -> Result<u32, impl error::Error> {
    const SEARCH_SIZE: usize = 13;
    let mut iter = unicode_segmentation::UnicodeSegmentation::graphemes(input, true);
    match iter.next_chunk::<SEARCH_SIZE>() {
        Ok(first) => {
            let mut last_seen = VecDeque::from(first);
            for (i, c) in iter.enumerate() {
                if last_seen.contains(&c) {
                    println!("{last_seen:?} contains {c}");
                    last_seen.pop_front();
                    last_seen.push_back(c);
                } else {
                    println!("{last_seen:?} does not contain {c}");
                    for n in 0..SEARCH_SIZE * SEARCH_SIZE {
                        if (n%SEARCH_SIZE != n/SEARCH_SIZE) && (last_seen[n%SEARCH_SIZE] == last_seen[n/SEARCH_SIZE]) {
                            println!("but does contain a dupe");
                            last_seen.pop_front();
                            last_seen.push_back(c);
                            break;
                        }
                        if n == SEARCH_SIZE * SEARCH_SIZE - 1 {
                            return Ok((i + SEARCH_SIZE + 1) as u32);
                        }
                    }
                }
            }
            return Err(TokenError{
                token:input.to_string(),
                reason: format!("no block of {SEARCH_SIZE} without repeats")
            });
        },
        Err(e) => {
            return Err(TokenError { token: input.to_string(), reason: "expected any characters at all".to_string() })
        }
        
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 6)
        };
    }
    #[test]
    fn test_part_one() {
        part_one(get_input!()).expect("First set of inputs resolves to a value.");
    }

    #[test]
    fn test_part_two() {
        part_two(get_input!()).expect("Second set of inputs resolves to a value.");
    }
}

