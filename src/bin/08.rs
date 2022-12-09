use advent_of_code::helpers::err::TokenError;
use itertools::Itertools;
use std::backtrace::Backtrace;
use std::{
    error,
    fmt::{self, Display},
};
use strum_macros;

#[macro_use]
extern crate enum_display_derive;

#[derive(Debug, Display)]
pub enum Error {
    TokenError(TokenError),
    IndexOutOfBoundsError(IndexOutOfBoundsError),
}
struct Point2D {
    x: usize,
    y: usize,
}

#[derive(strum_macros::Display, Debug, Clone)]
enum Dimension {
    X,
    Y,
}

#[derive(Debug, Clone)]
pub struct IndexOutOfBoundsError {
    got: usize,
    max: usize,
    trace: String,
    dimension: Dimension,
}
impl IndexOutOfBoundsError {
    fn new(got: usize, max: usize, dimension: Dimension) -> Self {
        IndexOutOfBoundsError {
            got,
            max,
            trace: Backtrace::capture().to_string(),
            dimension: dimension,
        }
    }
}
impl fmt::Display for IndexOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Index {} in dimension {} was out of bounds: min: 0, max:{}\nTrace:{}",
            &self.got, &self.dimension, &self.max, &self.trace
        ))
    }
}
impl error::Error for IndexOutOfBoundsError {}

trait Grid2D<V: Clone> {
    fn set_raw(&mut self, x: usize, y: usize, v: V) -> Result<V, Error>;
    fn set(&mut self, point: Point2D, v: V) -> Result<V, Error> {
        return self.set_raw(point.x, point.y, v);
    }
    fn get_raw(&self, x: usize, y: usize) -> Result<V, Error>;
    fn get(&self, point: Point2D) -> Result<V, Error> {
        return self.get_raw(point.x, point.y);
    }

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn check_bounds(&self, x: usize, y: usize) -> Result<(), Error> {
        if x >= self.width() {
            return Err(Error::IndexOutOfBoundsError(IndexOutOfBoundsError::new(
                x,
                self.width(),
                Dimension::X,
            )));
        }
        if y >= self.height() {
            return Err(Error::IndexOutOfBoundsError(IndexOutOfBoundsError::new(
                y,
                self.height(),
                Dimension::Y,
            )));
        }
        return Ok(());
    }

    fn point_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width() + x
    }
}
#[derive(Debug)]
struct BooleanGrid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}
impl BooleanGrid {
    fn count(&self) -> u128 {
        self.data.iter().map(|a| a.count_ones() as u128).sum()
    }
    #[inline]
    fn arr_index_and_shift(&self, index: usize) -> (usize, u8) {
        return (index / 8, 1u8 << (index % 8));
    }
    fn new(width: usize, height: usize) -> Self {
        let size = ((width * height) / 8) + 1;
        BooleanGrid {
            data: vec![0; size],
            width,
            height,
        }
    }
}
trait BooleanGridImpl {}

struct FastBooleanArray {
    length: usize,
    data: Vec<u8>,
    index: usize,
}

impl FastBooleanArray {
    fn new(size: usize) -> Self {
        FastBooleanArray {
            length: size,
            data: vec![0u8; size / 8 + 1],
            index: 0,
        }
    }
    #[inline]
    fn get(&self, index: usize) -> bool {
        return (self.data[index / 8] & 1u8 << index % 8) != 0;
    }

    #[inline]
    fn set(&mut self, index: usize) {
        self.data[index / 8] = self.data[index / 8] | 1u8 << index % 8;
    }
}

impl Iterator for FastBooleanArray {
    type Item = (usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None;
        } else {
            self.index += 1;
            return Some((self.index - 1, self.get(self.index - 1)));
        }
    }
}

struct U8Grid {
    data: Vec<u8>,
    width: usize,
}

impl Grid2D<bool> for BooleanGrid {
    fn set_raw(&mut self, x: usize, y: usize, v: bool) -> Result<bool, Error> {
        self.check_bounds(x, y)?;
        let (array_index, searcher) = self.arr_index_and_shift(self.point_to_index(x, y));
        let orig = (self.data[array_index] & searcher) != 0;
        if v {
            self.data[array_index] = self.data[array_index] | searcher;
        } else {
            self.data[array_index] = self.data[array_index] & !searcher;
        }
        return Ok(orig);
    }

    fn get_raw(&self, x: usize, y: usize) -> Result<bool, Error> {
        self.check_bounds(x, y)?;
        let (array_index, searcher) = self.arr_index_and_shift(self.point_to_index(x, y));
        return Ok((self.data[array_index] & searcher) != 0);
    }

    #[inline]
    fn width(&self) -> usize {
        self.width
    }
    #[inline]
    fn height(&self) -> usize {
        self.height
    }
}

impl Display for BooleanGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get_raw(x, y) {
                    Err(_) => panic!("aaa"),
                    Ok(true) => f.write_str("1")?,
                    Ok(false) => f.write_str("0")?,
                };
            }
            f.write_str("\n")?;
        }
        return Ok(());
    }
}

impl Display for U8Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get_raw(x, y) {
                    Err(_) => panic!("aaa"),
                    Ok(v) => f.write_fmt(format_args!("{v}"))?,
                };
            }
            f.write_str("\n")?;
        }
        return Ok(());
    }
}

impl Display for U32Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get_raw(x, y) {
                    Err(_) => panic!("aaa"),
                    Ok(v) => f.write_fmt(format_args!("{v:02},"))?,
                };
            }
            f.write_str("\n")?;
        }
        return Ok(());
    }
}

impl Grid2D<u8> for U8Grid {
    fn set_raw(&mut self, x: usize, y: usize, v: u8) -> Result<u8, Error> {
        self.check_bounds(x, y)?;
        let index = self.point_to_index(x, y);
        return Ok(std::mem::replace(&mut self.data[index], v));
    }

    fn get_raw(&self, x: usize, y: usize) -> Result<u8, Error> {
        self.check_bounds(x, y)?;
        return Ok(self.data[self.point_to_index(x, y)]);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }
}

struct U32Grid {
    data: Vec<u32>,
    width: usize,
}
impl U32Grid {
    fn new(width: usize, height: usize) -> Self {
        U32Grid {
            data: vec![0u32; width * height],
            width,
        }
    }
}
impl Grid2D<u32> for U32Grid {
    fn set_raw(&mut self, x: usize, y: usize, v: u32) -> Result<u32, Error> {
        self.check_bounds(x, y)?;
        let index = self.point_to_index(x, y);
        return Ok(std::mem::replace(&mut self.data[index], v));
    }

    fn get_raw(&self, x: usize, y: usize) -> Result<u32, Error> {
        self.check_bounds(x, y)?;
        return Ok(self.data[self.point_to_index(x, y)]);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }
}

impl U8Grid {
    fn new(width: usize, height: usize) -> Self {
        U8Grid {
            data: vec![0; width * height],
            width,
        }
    }
}
impl TryFrom<&str> for U8Grid {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        let lines = value.split("\n").filter(|a| !a.is_empty()).collect_vec();
        println!("{lines:?}");
        if lines.len() == 0 {
            return Err(Error::TokenError(TokenError {
                token: value.to_string(),
                reason: "Expected any lines at all".to_string(),
            }));
        }
        let exp_width = lines[0].len();
        if (exp_width == 0) {
            return Err(Error::TokenError(TokenError {
                reason: "Can't have width of 0".to_string(),
                token: value.to_string(),
            }));
        }
        let mut data = Vec::with_capacity(exp_width * lines.len());

        for line in lines {
            let chars = line.bytes();
            if chars.len() != exp_width {
                return Err(Error::TokenError(TokenError {
                    token: line.to_string(),
                    reason: format!("expected a width of exactly {exp_width}"),
                }));
            }
            for char in chars {
                if char < b'0' || char > b'9' {
                    return Err(Error::TokenError(TokenError {
                        token: char.to_string(),
                        reason: format!(
                            "expected a digit (byte between 48 (ascii 0) and 58 (ascii 9)"
                        ),
                    }));
                }
                data.push(char - b'0');
            }
        }

        return Ok(Self {
            data,
            width: exp_width,
        });
    }
}

fn left_right_search(
    width: usize,
    get: &dyn Fn(usize) -> Result<u8, Error>,
) -> Result<FastBooleanArray, Error> {
    let mut results = FastBooleanArray::new(width);
    let mut height_left = get(0)?;
    let mut height_right = get(width - 1)?;
    results.set(0);
    results.set(width - 1);

    for i in 1..width - 1 {
        if height_left < get(i)? {
            height_left = get(i)?;
            results.set(i);
        }
        let right_i = width - i - 1;
        if height_right < get(right_i)? {
            height_right = get(right_i)?;
            results.set(right_i);
        }
    }

    return Ok(results);
}

pub fn part_one(input: &str) -> Result<u128, Error> {
    let grid = U8Grid::try_from(input)?;
    let mut results: BooleanGrid = BooleanGrid::new(grid.width(), grid.height());
    for y in 0..grid.height() {
        for (i, res) in left_right_search(grid.width(), &|i| grid.get_raw(i, y))? {
            if res {
                results.set_raw(i, y, true)?;
            }
        }
    }

    for x in 0..grid.width() {
        for (i, res) in left_right_search(grid.height(), &|i| grid.get_raw(x, i))? {
            if res {
                results.set_raw(x, i, true)?;
            }
        }
    }

    return Ok(results.count());
}

pub fn part_two(input: &str) -> Result<u32, Error> {
    let grid = U8Grid::try_from(input)?;
    let mut results = U32Grid::new(grid.width(), grid.height());
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let height = grid.get_raw(x, y)?;
            let mut left = 0u32;
            for n in (0..x).rev() {
                left += 1;
                if grid.get_raw(n, y)? < height {
                } else {
                    break;
                }
            }
            let mut right = 0u32;
            for n in x + 1..grid.width() {
                right += 1;
                if grid.get_raw(n, y)? < height {
                } else {
                    break;
                }
            }
            let mut up = 0u32;
            for n in (0..y).rev() {
                up += 1;

                if grid.get_raw(x, n)? < height {
                } else {
                    break;
                }
            }
            let mut down = 0u32;
            for n in y + 1..grid.height() {
                down += 1;

                if grid.get_raw(x, n)? < height {
                } else {
                    break;
                }
            }
            results.set_raw(x, y, left * right * up * down)?;
        }
    }
    println!("{results}");
    match results.data.iter().max() {
        Some(v) => return Ok(*v as u32),
        None => panic!("aa"),
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    let examples = &advent_of_code::read_file("examples", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_input {
        () => {
            &advent_of_code::read_file("inputs", 8)
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
