use std::{
    error::Error,
    ops::{Add, Mul},
    str::FromStr,
};

use eyre::{Result, eyre};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<char> for Op {
    type Error = eyre::ErrReport;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => return Err(eyre!("invalid operator")),
        })
    }
}

#[derive(Debug)]
struct MathTask<T> {
    numbers: Vec<T>,
    op: Op,
}

impl<T> MathTask<T> {
    fn solve(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + From<u8> + Copy,
    {
        match self.op {
            Op::Add => self.numbers.iter().fold(T::from(0), |acc, num| acc + *num),
            Op::Mul => self.numbers.iter().fold(T::from(1), |acc, num| acc * *num),
        }
    }
}

#[derive(Debug)]
struct Homework<T> {
    tasks: Vec<MathTask<T>>,
}

impl<T> Homework<T>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
{
    fn from_input_part_one(input: &str) -> Result<Self> {
        let mut lines: Vec<&str> = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let ops_line = lines
            .pop()
            .ok_or(eyre!("need at least one line in input"))?;

        let nums = lines
            .into_iter()
            .map(|l| l.split_whitespace().map(|x| Ok(x.parse::<T>()?)).collect())
            .collect::<Result<Vec<Vec<T>>>>()?;

        let ops = ops_line
            .split_whitespace()
            .map(|tok| {
                Op::try_from(
                    tok.chars()
                        .next()
                        .ok_or(eyre!("need at least one char to parse an operator"))?,
                )
            })
            .collect::<Result<Vec<Op>>>()?;

        let tasks = transpose(nums)
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                Ok(MathTask {
                    numbers: line.to_vec(),
                    op: *ops.get(idx).ok_or(eyre::format_err!(
                        "index {} out of bounds for ops vector",
                        idx
                    ))?,
                })
            })
            .collect::<Result<Vec<MathTask<T>>>>()?;

        Ok(Self { tasks })
    }

    fn from_input_part_two(input: &str) -> Result<Self>
    where
        T: std::fmt::Debug,
    {
        let mut lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

        let ops_line = lines
            .pop()
            .ok_or(eyre!("need at least one line in input"))?;

        #[derive(Debug)]
        enum OpOrSpace {
            Op(Op),
            Space,
        }

        impl TryFrom<char> for OpOrSpace {
            type Error = eyre::ErrReport;

            fn try_from(value: char) -> Result<Self> {
                if let Ok(op) = Op::try_from(value) {
                    return Ok(Self::Op(op));
                }

                match value {
                    ' ' => Ok(Self::Space),
                    _ => Err(eyre::format_err!("unsupported char {}", value)),
                }
            }
        }

        let ops_lines_parsed = ops_line
            .chars()
            .map(OpOrSpace::try_from)
            .collect::<Result<Vec<_>>>()?;

        #[derive(Debug)]
        struct OpWithSize {
            op: Op,
            size: i32,
        }

        let mut ops_with_size: Vec<OpWithSize> = vec![];

        for op_or_space in ops_lines_parsed {
            match op_or_space {
                OpOrSpace::Op(op) => {
                    if let Some(op_with_size) = ops_with_size.last_mut() {
                        op_with_size.size -= 1;
                    }

                    ops_with_size.push(OpWithSize { op, size: 1 })
                }
                OpOrSpace::Space => {
                    ops_with_size
                        .last_mut()
                        .ok_or(eyre!("operations line expected to begin with an operator"))?
                        .size += 1
                }
            }
        }

        let nums_lines = rotate_left(
            lines
                .iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        );

        let mut idx = 0;
        let mut math_tasks = vec![];

        for op_with_size in ops_with_size.iter().rev() {
            let mut numbers = vec![];

            for i in idx..(idx + op_with_size.size) {
                numbers.push(
                    String::from_iter(
                        nums_lines
                            .get(i as usize)
                            .ok_or(eyre::format_err!("index {} out of range", i))?,
                    )
                    .trim()
                    .parse::<T>()?,
                );
            }

            idx += op_with_size.size + 1;

            math_tasks.push(MathTask {
                numbers,
                op: op_with_size.op,
            });
        }

        Ok(Self { tasks: math_tasks })
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return v;
    }

    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn rotate_left<T: Copy + std::fmt::Debug>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return v;
    }

    transpose(
        v.iter()
            .map(|line| line.iter().rev().copied().collect())
            .collect(),
    )
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() -> Result<()> {
        let hw = Homework::<i32>::from_input_part_one(include_str!("testinput.txt"))?;
        let res = hw.tasks.iter().map(|task| task.solve()).sum::<i32>();
        assert_eq!(res, 4277556);
        Ok(())
    }

    #[test]
    fn part_one_solution() -> Result<()> {
        let hw = Homework::<i64>::from_input_part_one(include_str!("input.txt"))?;
        let res = hw.tasks.iter().map(|task| task.solve()).sum::<i64>();
        assert_eq!(res, 5060053676136);
        Ok(())
    }

    #[test]
    fn part_two_example() -> Result<()> {
        let hw = Homework::<i32>::from_input_part_two(include_str!("testinput.txt"))?;
        let res = hw.tasks.iter().map(|task| task.solve()).sum::<i32>();
        assert_eq!(res, 3263827);
        Ok(())
    }

    #[test]
    fn part_two_solution() -> Result<()> {
        let hw = Homework::<i64>::from_input_part_two(include_str!("input.txt"))?;
        let res = hw.tasks.iter().map(|task| task.solve()).sum::<i64>();
        assert_eq!(res, 9695042567249);
        Ok(())
    }
}
