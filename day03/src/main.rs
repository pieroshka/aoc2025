use eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<u8>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        Self {
            batteries: value
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect(),
        }
    }
}

impl BatteryBank {
    fn find_strongest_batteries_2(&self) -> u8 {
        self.batteries
            .iter()
            .enumerate()
            .cartesian_product(self.batteries.clone().iter().enumerate())
            .filter(|((i, _), (j, _))| i < j)
            .map(|((_, x), (_, y))| x * 10 + y)
            .max()
            .unwrap() // boom
    }

    fn find_strongest_batteries_12(&self) -> u8 {
        self.batteries
            .iter()
            .enumerate()
            .cartesian_product(self.batteries.clone().iter().enumerate())
            .filter(|((i, _), (j, _))| i < j)
            .map(|((_, x), (_, y))| x * 10 + y)
            .max()
            .unwrap() // boom
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use eyre::eyre;

    #[test]
    fn part_one_solution() -> Result<()> {
        let res: u32 = include_str!("input.txt")
            .lines()
            .map(|line| BatteryBank::from(line))
            .map(|bank| bank.find_strongest_batteries_2() as u32)
            .inspect(|elem| println!("{:?}", elem))
            .sum::<u32>();

        assert_eq!(res, 17244);
        Ok(())
    }

    #[test]
    fn part_two_solution() -> Result<()> {
        Err(eyre!("no solution yet"))
    }
}
