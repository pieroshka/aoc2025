use eyre::{Result, eyre};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Floor,
    Paper,
}

impl TryFrom<char> for Item {
    type Error = eyre::ErrReport;
    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Self::Floor),
            '@' => Ok(Self::Paper),
            _ => Err(eyre::format_err!("unknown grid char: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Room {
    grid: Vec<Vec<Item>>,
}

impl Room {
    pub fn from_lines<I>(iter: I) -> eyre::Result<Self>
    where
        I: IntoIterator<Item = String>,
    {
        let grid = iter
            .into_iter()
            .map(|line| line.chars().map(Item::try_from).collect())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { grid })
    }

    fn neighbors(&self, y: usize, x: usize) -> Result<Vec<Item>> {
        let size_y = self.get_size_y();
        let size_x = self.get_size_x()?;

        if x >= size_x || y >= size_y {
            return Err(eyre!("invalid param(s): {x} {y}"));
        }

        let mut out = Vec::new();

        for i in y.saturating_sub(1)..=usize::min(y + 1, size_y - 1) {
            for j in x.saturating_sub(1)..=usize::min(x + 1, size_x - 1) {
                if (i, j) == (y, x) {
                    continue;
                }

                let item = self.grid[i][j];
                out.push(item);
            }
        }

        Ok(out)
    }

    fn neighbor_paper_count(&self, y: usize, x: usize) -> Result<usize> {
        Ok(self
            .neighbors(y, x)?
            .into_iter()
            .filter(|e| *e == Item::Paper)
            .count())
    }

    fn get_size_y(&self) -> usize {
        self.grid.len()
    }

    fn get_size_x(&self) -> Result<usize> {
        Ok(self.grid.first().ok_or(eyre!("board empty"))?.len())
    }

    fn get_item_at(&self, y: usize, x: usize) -> Option<Item> {
        Some(*self.grid.get(y)?.get(x)?)
    }

    fn set_item_at(&mut self, y: usize, x: usize, item: Item) {
        self.grid[y][x] = item;
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() -> Result<()> {
        let room = Room::from_lines(
            include_str!("testinput.txt")
                .lines()
                .map(|line| line.to_string()),
        )?;

        let mut forklift_accessible = 0;
        for i in 0..room.get_size_y() {
            for j in 0..room.get_size_x()? {
                if room.get_item_at(i, j).unwrap() == Item::Paper
                    && room.neighbor_paper_count(i, j)? < 4
                {
                    forklift_accessible += 1;
                }
            }
        }
        assert_eq!(forklift_accessible, 13);
        Ok(())
    }

    #[test]
    fn example_two() -> Result<()> {
        let mut room = Room::from_lines(
            include_str!("testinput.txt")
                .lines()
                .map(|line| line.to_string()),
        )?;

        let size_y = room.get_size_y();
        let size_x = room.get_size_x()?;

        let mut rolls_counter = 0;

        loop {
            let mut cells_to_update = vec![];
            let mut forklift_accessible = 0;

            for i in 0..size_y {
                for j in 0..size_x {
                    let neighbor_paper_count = room.neighbor_paper_count(i, j)?;
                    let item = room.get_item_at(i, j).unwrap();

                    if item == Item::Paper && neighbor_paper_count < 4 {
                        forklift_accessible += 1;
                        cells_to_update.push((i, j));
                    }
                }
            }

            println!("{}", forklift_accessible);

            for (y, x) in cells_to_update {
                room.set_item_at(y, x, Item::Floor);
            }

            if forklift_accessible == 0 {
                break;
            }
            rolls_counter += forklift_accessible;
        }

        assert_eq!(rolls_counter, 43);
        Ok(())
    }

    #[test]
    fn part_one_solution() -> Result<()> {
        let room = Room::from_lines(
            include_str!("input.txt")
                .lines()
                .map(|line| line.to_string()),
        )?;

        let mut forklift_accessible = 0;
        for i in 0..room.get_size_y() {
            for j in 0..room.get_size_x()? {
                if room.get_item_at(i, j).unwrap() == Item::Paper
                    && room.neighbor_paper_count(i, j)? < 4
                {
                    forklift_accessible += 1;
                }
            }
        }
        assert_eq!(forklift_accessible, 1533);
        Ok(())
    }

    #[test]
    fn part_two_solution() -> Result<()> {
        let mut room = Room::from_lines(
            include_str!("input.txt")
                .lines()
                .map(|line| line.to_string()),
        )?;

        let size_y = room.get_size_y();
        let size_x = room.get_size_x()?;

        let mut rolls_counter = 0;

        loop {
            let mut cells_to_update = vec![];
            let mut forklift_accessible = 0;

            for i in 0..size_y {
                for j in 0..size_x {
                    let neighbor_paper_count = room.neighbor_paper_count(i, j)?;
                    let item = room.get_item_at(i, j).unwrap();

                    if item == Item::Paper && neighbor_paper_count < 4 {
                        forklift_accessible += 1;
                        cells_to_update.push((i, j));
                    }
                }
            }

            println!("{}", forklift_accessible);

            for (y, x) in cells_to_update {
                room.set_item_at(y, x, Item::Floor);
            }

            if forklift_accessible == 0 {
                break;
            }
            rolls_counter += forklift_accessible;
        }

        assert_eq!(rolls_counter, 9206);
        Ok(())
    }
}
