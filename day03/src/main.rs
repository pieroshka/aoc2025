
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

    fn find_strongest_batteries_12(&self) -> Result<u64> {
        let mut result_battery = vec![];
        let bank_size = self.batteries.len();
        let mut cutoff = None;
        for cell_level in (0..=9).rev() {
            let mut cells_for_level = self
                .batteries
                .iter()
                .enumerate()
                .filter(|(_, cell)| **cell == cell_level)
                .filter(|(idx, _)| *idx >= cutoff.unwrap_or(0))
                .collect::<Vec<(usize, &u8)>>();

            if cutoff.is_some() {
                cells_for_level.reverse();
            }

            for (level_cell_bank_idx, level_cell) in cells_for_level.iter() {
                let numbers_to_the_right = bank_size - level_cell_bank_idx - 1;
                // once we lock onto the biggest num that can be the first digit of a 12digit num, only consider cells
                // to the right
                if numbers_to_the_right >= 11 && cutoff.is_none() {
                    cutoff = Some(*level_cell_bank_idx);
                }

                let mut inserted = false;

                for (res_idx, (result_cell_bank_idx, _)) in result_battery.iter().enumerate() {
                    if *level_cell_bank_idx < *result_cell_bank_idx {
                        result_battery.insert(res_idx, (*level_cell_bank_idx, *level_cell));
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    result_battery.push((*level_cell_bank_idx, *level_cell));
                }

                if result_battery.len() >= 12 {
                    return Ok(result_battery
                        .iter()
                        .map(|(_, cell)| cell)
                        .join("")
                        .parse::<u64>()?);
                }
            }
        }

        Err(eyre::format_err!(
            "couldn't find 12 digits among {} digits (math broke)",
            bank_size
        ))
    }
}

fn main() -> Result<()> {
    let res = include_str!("testinput.txt")
        .lines()
        .map(BatteryBank::from)
        .map(|bank| bank.find_strongest_batteries_12())
        .inspect(|e| println!("{:#?}", e))
        .collect::<Result<Vec<u64>>>()?
        .iter()
        .sum::<u64>();

    println!("{:#?}", res);
    Ok(())
}

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
    fn part_two_example() -> Result<()> {
        let res = include_str!("testinput.txt")
            .lines()
            .map(|line| BatteryBank::from(line))
            .map(|bank| bank.find_strongest_batteries_12())
            .collect::<Result<Vec<u64>>>()?
            .iter()
            .sum::<u64>();

        assert_eq!(res, 3121910778619);
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_two_solution() -> Result<()> {
        Err(eyre!("no solution yet"))
    }
}
