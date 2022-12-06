use advent_of_code::helpers::err::TokenError;
use itertools::Itertools;
use std::{error::Error};
use seq_macro::seq;


macro_rules! check_ASCII_bounds {
    ($byte:ident) => {
        if $byte < &b'A' || $byte > &b'z' {
                return match String::from_utf8(vec![*$byte]) {
                    Ok(token) => Err(TokenError{
                        token: format! ("{:?} which maps to {token}", $byte),
                        reason: format!("Expected a value between {} (ascii/utf-8 A) and {} (ascii/utf-8 z)", b'A', b'z')
                    }),
                    Err(_err) => Err(TokenError{
                        token: format!("{:?}", $byte),
                        reason: format!("Expected a value between {} (ascii/utf-8 A) and {} (ascii/utf-8 z)", b'A', b'z')
                    }),
                }
            }
    };
}

macro_rules! get_priority_from_ascii {
    ($byte:ident) => {
        match $byte {
            b'A'..=b'Z' => Ok(($byte - &b'A' + 27) as u32),
            b'a'..=b'z' => Ok(($byte - &b'a' + 1) as u32),
            _ => Err(TokenError {
                            token: String::from_utf8(vec![*$byte]).expect("We got this byte from another utf string"),
                            reason: format!("Expected a value not within {} (Z) and {} (a) (implies non-alphabetic character)", b'Z', b'a'),
                        })
        }
    };
}

#[inline(always)]
fn get_ascii_shift(&byte: &u8) -> u64 {
    1u64 << (byte-b'A')
}

#[inline(always)]
fn set_ascii_bitfield(field:&u64, byte:&u8) -> u64 {
    field | get_ascii_shift(byte)
}

#[inline(always)]
fn get_ascii_bitfield(field:&u64, byte:&u8) -> bool {
    (field & get_ascii_shift(byte)) != 0
}

pub fn part_one(input: &str) -> Result<u32, impl Error> {
    input.split("\n").map(|line| {
        if line.is_empty() {
            return Ok(0);
        }
        let mut hash_field = 0u64;
        let line_bytes = line.as_bytes();
        for (i, byte) in line_bytes.iter().enumerate(){
            check_ASCII_bounds!(byte);

            if i < line_bytes.len()/2 {
                hash_field = set_ascii_bitfield(&hash_field, byte);
            }else{
                if get_ascii_bitfield(&hash_field, byte){
                    return get_priority_from_ascii!(byte);
                }
            }
        }
        return Err(TokenError { token: line.to_string(), reason: "Expected at least one duplicate across halves".to_string() })
    }).fold_ok(0, std::ops::Add::add)
}

pub fn part_two(input: &str) -> Result<u32, impl Error> {
    input.split("\n").chunks(3).into_iter().map(|chunk| {
        let elfs = chunk.collect_vec();
        if elfs.len() == 1 && elfs[0].is_empty() {
            return Ok(0);
        }
        assert_eq!(elfs.len(), 3, "Should have a chunk of size 3");
        seq!(n in 1..3 {
            let mut bit_field_~n = 0u64;
            for byte in elfs[n-1].as_bytes() {
                check_ASCII_bounds!(byte);
                bit_field_~n = set_ascii_bitfield(&bit_field_~n, byte);
            }
        });
        for byte in elfs[2].as_bytes() {
            if bit_field_1 & bit_field_2 & get_ascii_shift(byte) != 0 {
                return get_priority_from_ascii!(byte);
            }
        }
        return Err(TokenError { token: elfs.join("\n"), reason: "Expected at least one commonality across group".to_string() });
        
    }).fold_ok(0, std::ops::Add::add)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 3)
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

