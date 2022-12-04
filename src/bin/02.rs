use advent_of_code::helpers::err::TokenError;
use itertools::Itertools;
use std::{error::Error, iter::Sum};

// We get a tiiiiiny bit of extra performance in L1/L2(?) caches if we reduce the size of these; no point going below a byte because we can't address closer than that though (and the stack needs to remain 16-aligned).
#[repr(u8)]
#[derive(Clone, Copy)]
enum RPS {
    Rock = 0,
    Paper,
    Scissors,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Outcome {
    // We could get a touch more performance if we allowed lose to be 3, but this way it's much easier to calculate
    Lose = 0,
    Tie,
    Win,
}

#[inline]
fn try_from_opp(letter: &str) -> Result<RPS, TokenError> {
    match letter {
        "A" => Ok(RPS::Rock),
        "B" => Ok(RPS::Paper),
        "C" => Ok(RPS::Scissors),
        _ => Err(TokenError {
            token: letter.to_string(),
            reason: "Unexpected token, expecting A B or C".to_string(),
        }),
    }
}

#[inline]
fn try_from_out(letter: &str) -> Result<Outcome, TokenError> {
    match letter {
        "X" => Ok(Outcome::Lose),
        "Y" => Ok(Outcome::Tie),
        "Z" => Ok(Outcome::Win),
        _ => Err(TokenError {
            token: letter.to_string(),
            reason: "Unexpected token, expecting X Y or Z".to_string(),
        }),
    }
}

#[inline]
fn try_from_you(letter: &str) -> Result<RPS, TokenError> {
    match letter {
        "X" => Ok(RPS::Rock),
        "Y" => Ok(RPS::Paper),
        "Z" => Ok(RPS::Scissors),
        _ => Err(TokenError {
            token: letter.to_string(),
            reason: "Unexpected token, expecting X Y or Z".to_string(),
        }),
    }
}

/*
This is kinda dumb but it works and is probably super fast. There's something here about finite groups operations but meh

R R T 0 0 1
R P W 0 1 2
R S L 0 2 0

P R L 1 0 0
P P T 1 1 1
P S W 1 2 2

S R W 2 0 2
S P L 2 1 0
S S T 2 2 0

2 * opp + you + 1 % 3
*/

#[inline(always)]
fn rps(opp: RPS, you: RPS) -> Outcome {
    // Evil hack. Be very careful here. speedy_transmute look safe but is actually unsafe so yannow there's that.
    unsafe { totally_speedy_transmute::speedy_transmute((opp as u8 + opp as u8 + you as u8 + 1u8) % 3u8) }
}

#[inline(always)]
fn solve_rps(opp: RPS, out: Outcome) -> RPS {
    // Again, another evil hack. Be very careful.
    // This is the inverse of RPS, which should be easily derived; we add 8 instead of subtracting 1 because we're in base 3 and we need to make sure that the unsigned int doesn't go negative.
    unsafe { totally_speedy_transmute::speedy_transmute((out as u8 + 8u8 - opp as u8 - opp as u8) % 3u8) }
}

pub fn part_one(input: &str) -> Result<u32, impl Error> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| -> Result<u32, TokenError> {
            let round = line.split(" ").collect_vec();
            if round.len() != 2 {
                return Err(TokenError {
                    token: line.to_string(),
                    reason: format!(
                        "Expected two entries, got {line} which has {} instead",
                        round.len()
                    ),
                });
            }

            return match try_from_opp(round[0]) {
                Err(opp_err) => Err(TokenError {
                    token: line.to_string(),
                    reason: format!("{} is {}", opp_err.token, opp_err.reason),
                }),
                Ok(opp) => match try_from_you(round[1]) {
                    Err(you_err) => Err(TokenError {
                        token: line.to_string(),
                        reason: format!("{} is {}", you_err.token, you_err.reason),
                    }),
                    Ok(you) => Ok(1 + you as u32 + 3 * rps(opp, you) as u32),
                },
            };
        })
        .fold_ok(u32::MIN, std::ops::Add::add)
}

pub fn part_two(input: &str) -> Result<u32, TokenError> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| -> Result<u32, TokenError> {
            let round = line.split(" ").collect_vec();
            if round.len() != 2 {
                return Err(TokenError {
                    token: line.to_string(),
                    reason: format!(
                        "Expected two entries, got {line} which has {} instead",
                        round.len()
                    ),
                });
            }

            return match try_from_opp(round[0]) {
                Err(opp_err) => Err(TokenError {
                    token: line.to_string(),
                    reason: format!("{} is {}", opp_err.token, opp_err.reason),
                }),
                Ok(opp) => match try_from_out(round[1]) {
                    Err(you_err) => Err(TokenError {
                        token: line.to_string(),
                        reason: format!("{} is {}", you_err.token, you_err.reason),
                    }),
                    Ok(out) => Ok(1 + solve_rps(opp, out) as u32 + 3 * out as u32),
                },
            };
        })
        .fold_ok(u32::MIN, std::ops::Add::add)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 2)
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
