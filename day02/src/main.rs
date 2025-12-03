use std::num::ParseIntError;

use eyre::{Result, eyre};
use itertools::Itertools;

#[derive(Debug)]
struct IdRange {
    // raw_from: String,
    // raw_to: String,
    from: i64,
    to: i64,
}

impl TryFrom<String> for IdRange {
    type Error = eyre::ErrReport;

    fn try_from(value: String) -> Result<Self> {
        let mut iter = value.split('-');
        let from = iter.next().ok_or(eyre!("invalid format"))?;
        let to = iter.next().ok_or(eyre!("invalid format"))?;

        Ok(IdRange {
            from: from.trim().parse()?,
            to: to.trim().parse()?,
        })
    }
}

impl IdRange {
    fn invalid_id_from_half(half: i64) -> i64 {
        let ord = f64::log10(half as f64) as u32;
        half * i64::pow(10, ord + 1) + half
    }

    fn get_ord(num: i64) -> usize {
        f64::log10(num as f64) as usize
    }

    fn get_invalid_ids_part1(&self) -> Vec<i64> {
        let ord_from = Self::get_ord(self.from);
        let ord_to = Self::get_ord(self.to);

        (self.from / i64::pow(10, ((ord_from / 2) + 1) as u32)
            ..=(self.to / i64::pow(10, ((ord_to / 2) + (ord_to % 2)) as u32)))
            .into_iter()
            .filter_map(|elem| {
                let invalid_id = Self::invalid_id_from_half(elem);

                // for odd ords like 12345 the id will be 12 and 12 falls out of this range
                (invalid_id >= self.from && invalid_id <= self.to).then_some(invalid_id)
            })
            .collect()
    }

    fn get_invalid_ids_part2(&self) -> Vec<i64> {
        let ord_from = Self::get_ord(self.from);
        let ord_to = Self::get_ord(self.to);

        let divisors: Vec<i32> = (1..ord_from + 1)
            .filter_map(|e| ((ord_from + 1) % e == 0).then_some(e as i32))
            .chain((1..ord_to + 1).filter_map(|e| ((ord_to + 1) % e == 0).then_some(e as i32)))
            .unique()
            .collect();

        let mut all_guesses = vec![];

        for divisor in divisors {
            let digits: Vec<char> = "0123456789".chars().collect();

            let parts = std::iter::repeat_with(|| digits.iter().copied())
                .take(divisor as usize)
                .multi_cartesian_product()
                .filter_map(|elem| {
                    let s = String::from_utf8_lossy(
                        elem.iter()
                            .map(|e| *e as u8)
                            .collect::<Vec<u8>>()
                            .as_slice(),
                    )
                    .to_string();

                    (!s.starts_with("0")).then_some(s)
                })
                // .map(|elem| elem.repeat(ord_to + 1 / divisor as usize))
                .collect::<Vec<String>>();

            for ord in ord_from..=ord_to + 1 {
                if ord == divisor as usize
                    || (ord) % divisor as usize != 0
                    || (ord > 2 && ord % 2 == 0 && divisor == 1)
                // could deduplicate further by reducing divisors that are also divisors of larger divisors of ord. it's 3am already though so nope
                {
                    continue;
                }

                all_guesses.extend(
                    parts
                        .iter()
                        .map(|elem| elem.repeat((ord) / divisor as usize))
                        .filter(|elem| !elem.is_empty()),
                );
            }
        }
        all_guesses
            .iter()
            .unique()
            .map(|elem| elem.parse::<i64>().unwrap()) // boom
            .filter(|elem| *elem >= self.from && *elem <= self.to)
            .collect()
    }
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let res = input
        .split(',')
        .map(|range| IdRange::try_from(range.to_string()))
        .collect::<Result<Vec<IdRange>>>()?
        .iter()
        .flat_map(|range| range.get_invalid_ids_part1())
        .map(|elem| elem as i64)
        .sum::<i64>();

    println!("part1: {:#?}", res);

    let res = input
        .split(',')
        .map(|range| IdRange::try_from(range.to_string()))
        .collect::<Result<Vec<IdRange>>>()?
        .iter()
        // .inspect(|e| println!("{:?}", e))
        .flat_map(|range| range.get_invalid_ids_part2())
        .unique()
        .sum::<i64>();

    println!("part2: {:#?}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_get_invalid_ids_part1() -> Result<(), Box<dyn Error>> {
        let input = include_str!("testinput.txt");

        let res = input
            .split(',')
            .map(|range| IdRange::try_from(range.to_string()))
            .collect::<Result<Vec<IdRange>>>()?
            .iter()
            // .inspect(|e| println!("{:?}", e))
            .flat_map(|range| range.get_invalid_ids_part1())
            .map(|elem| elem as i64)
            .sum::<i64>();

        assert_eq!(res, 1227775554);
        Ok(())
    }

    #[test]
    fn test_get_invalid_ids_part2() -> Result<(), Box<dyn Error>> {
        let input = include_str!("testinput.txt");

        let res = input
            .split(',')
            .map(|range| IdRange::try_from(range.to_string()))
            .collect::<Result<Vec<IdRange>>>()?
            .iter()
            .flat_map(|range| range.get_invalid_ids_part2())
            .unique()
            // .inspect(|e| println!("{:?}", e))
            .sum::<i64>();

        assert_eq!(res, 4174379265);
        Ok(())
    }
}
