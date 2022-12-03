use advent_of_code::helpers::err::{self, TokenError};
use itertools::Itertools;

pub fn part_one(input: &str) -> Result<u32, impl err::Error> {
    // Does the spec guarentee that blank lines are blank and not just whitespace?
    input
        .split("\n\n")
        .map(|elf_input: &str| {
            elf_input
                .split("\n")
                .filter(|elf| !elf.is_empty())
                .map(|elf_cal| match elf_cal.parse::<u32>() {
                    Err(e) => Err(TokenError {
                        token: elf_cal.to_string(),
                        reason: e.to_string(),
                    }),
                    Ok(cal) => Ok(cal),
                })
                .fold_ok(0u32, std::ops::Add::add)
        })
        .fold_ok(u32::MIN, std::cmp::max)
}

pub fn part_two(input: &str) -> Result<u32, impl err::Error> {
    let mut calories_by_elf = input.split("\n\n").map(|elf_input: &str| {
        elf_input
            .split("\n")
            .filter(|elf| !elf.is_empty())
            .map(|elf_cal| match elf_cal.parse::<u32>() {
                Err(e) => Err(TokenError {
                    token: elf_cal.to_string(),
                    reason: e.to_string(),
                }),
                Ok(cal) => Ok(cal),
            })
            .fold_ok(0u32, std::ops::Add::add)
    });
    match calories_by_elf.clone().find(|a| match a {
        Err(_) => true,
        Ok(_) => false,
    }){
        Some(e) => Err(e.expect_err("calories_by_elf find should match Err")),
        None => Ok(calories_by_elf
                .map(|a| a.expect("All calories should be good"))
                .sorted_by(|a,b| Ord::cmp(b,a))
                .take(3).sum()),
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 1)
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
