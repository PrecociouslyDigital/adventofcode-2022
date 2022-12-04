use advent_of_code::helpers::err::{self, TokenError};
use itertools::Itertools;
use std::error;

pub fn part_one(input: &str) -> Result<u32, impl error::Error> {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|line| {
            let elves = line
                .split(",")
                .map(|elf| elf.split("-").map(str::parse::<u32>).collect_vec())
                .collect_vec();
            if elves.len() != 2 {
                return Err(TokenError {
                    token: line.to_string(),
                    reason: "There should be two elves per line".to_string(),
                });
            }

            seq_macro::seq!(n in 0..2{
                if elves[n].len() != 2 {
                    return Err(TokenError{
                        token: format!("{:?}", elves[n]),
                        reason: format!("There should be a start and an end for each elf in {line}")
                    });
                }

            });

            // This is a prime candidate for macrofication
            return match &elves[0][0] {
                Ok(elf_1_start) => match &elves[0][1] {
                    Ok(elf_1_end) => match &elves[1][0] {
                        Ok(elf_2_start) => match &elves[1][1] {
                            Ok(elf_2_end) => {
                                if elf_1_start > elf_1_end {
                                    return Err(TokenError { token: line.to_string(), reason: format!("start ({elf_1_start}) is after end ({elf_1_end})") });
                                }
                                if elf_2_start > elf_2_end {
                                    return Err(TokenError { token: line.to_string(), reason: format!("start ({elf_2_start}) is after end ({elf_2_end})") });
                                } 
                                return Ok(((elf_1_start <= elf_2_start)
                                && (elf_1_end >= elf_2_end)
                                || (elf_1_start >= elf_2_start) && (elf_1_end <= elf_2_end))
                                as u32);
                            },
                            Err(err) => Err(TokenError {
                                token: line.to_string(),
                                reason: err.to_string(),
                            }),
                        },
                        Err(err) => Err(TokenError {
                            token: line.to_string(),
                            reason: err.to_string(),
                        }),
                    },
                    Err(err) => Err(TokenError {
                        token: line.to_string(),
                        reason: err.to_string(),
                    }),
                },
                Err(err) => Err(TokenError {
                    token: line.to_string(),
                    reason: err.to_string(),
                }),
            };
        })
        .fold_ok(0, std::ops::Add::add)
}

pub fn part_two(input: &str) -> Result<u32, impl error::Error> {
        input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|line| {
            let elves = line
                .split(",")
                .map(|elf| elf.split("-").map(str::parse::<u32>).collect_vec())
                .collect_vec();
            if elves.len() != 2 {
                return Err(TokenError {
                    token: line.to_string(),
                    reason: "There should be two elves per line".to_string(),
                });
            }

            seq_macro::seq!(n in 0..2{
                if elves[n].len() != 2 {
                    return Err(TokenError{
                        token: format!("{:?}", elves[n]),
                        reason: format!("There should be a start and an end for each elf in {line}")
                    });
                }

            });

            // This is a prime candidate for macrofication
            return match &elves[0][0] {
                Ok(elf_1_start) => match &elves[0][1] {
                    Ok(elf_1_end) => match &elves[1][0] {
                        Ok(elf_2_start) => match &elves[1][1] {
                            Ok(elf_2_end) => {
                                if elf_1_start > elf_1_end {
                                    return Err(TokenError { token: line.to_string(), reason: format!("start ({elf_1_start}) is after end ({elf_1_end})") });
                                }
                                if elf_2_start > elf_2_end {
                                    return Err(TokenError { token: line.to_string(), reason: format!("start ({elf_2_start}) is after end ({elf_2_end})") });
                                } 
                                if elf_1_start <= elf_2_start {
                                    return Ok((elf_1_end >= elf_2_start) as u32);
                                }else {
                                    return Ok((elf_1_start <= elf_2_end) as u32);
                                }
                            },
                            Err(err) => Err(TokenError {
                                token: line.to_string(),
                                reason: err.to_string(),
                            }),
                        },
                        Err(err) => Err(TokenError {
                            token: line.to_string(),
                            reason: err.to_string(),
                        }),
                    },
                    Err(err) => Err(TokenError {
                        token: line.to_string(),
                        reason: err.to_string(),
                    }),
                },
                Err(err) => Err(TokenError {
                    token: line.to_string(),
                    reason: err.to_string(),
                }),
            };
        })
        .fold_ok(0, std::ops::Add::add)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 4)
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
