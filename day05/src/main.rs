use eyre::{Result, eyre};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Id(u64);

impl Id {
    fn try_from_str(value: &str) -> Result<Self> {
        Ok(Self(value.parse()?))
    }
}

#[derive(Copy, Clone, Debug)]
struct Range<T> {
    from: T,
    to: T,
}

impl Range<Id> {
    fn from_line(value: &str) -> Result<Self> {
        let mut it = value.split('-');

        let from = it.next().ok_or(eyre!("first value missing"))?;
        let to = it.next().ok_or(eyre!("second value missing"))?;

        Ok(Self {
            from: Id::try_from_str(from)?,
            to: Id::try_from_str(to)?,
        })
    }
}

fn merge_ranges<T: Ord + Copy + Clone + std::fmt::Debug>(
    ranges: Vec<Range<T>>,
    new: Range<T>,
) -> Vec<Range<T>> {
    if ranges.is_empty() {
        return vec![new];
    }

    let mut ranges: Vec<Range<T>> = ranges.clone();
    let mut new = new;
    loop {
        let mut merged = false;
        for (i, range) in ranges.clone().iter().enumerate() {
            if new.from >= range.from && new.from <= range.to {
                new.from = range.from;
                new.to = new.to.max(range.to);
                ranges.swap_remove(i);
                merged = true;
                break;
            }

            if new.to >= range.from && new.to <= range.to {
                new.from = new.from.min(range.from);
                new.to = range.to;
                ranges.swap_remove(i);
                merged = true;
                break;
            }

            if new.from < range.from && new.to > range.to {
                merged = true;
                ranges.swap_remove(i);
                break;
            }
        }

        if ranges.is_empty() {
            return vec![new];
        }

        if !merged {
            ranges.push(new);
            break;
        }
    }

    ranges
}

#[derive(Debug)]
struct Database {
    fresh_ranges: Vec<Range<Id>>,
    ids: Vec<Id>,
}

impl Database {
    fn from_input(value: &str) -> Result<Self> {
        let mut lines_iter = value.lines();
        let ranges = lines_iter
            .by_ref()
            .map_while(|line| Range::from_line(line).ok())
            .collect();
        let ids = lines_iter
            .map(Id::try_from_str)
            .collect::<Result<Vec<Id>>>()?;

        Ok(Self {
            fresh_ranges: ranges,
            ids,
        })
    }

    fn is_fresh(&self, id: Id) -> bool {
        self.fresh_ranges
            .iter()
            .any(|range| id >= range.from && id <= range.to)
    }
}

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() -> Result<()> {
        let db = Database::from_input(include_str!("testinput.txt"))?;
        let res = db.ids.iter().filter(|id| db.is_fresh(**id)).count();
        assert_eq!(res, 3);
        Ok(())
    }

    #[test]
    fn part_one_solution() -> Result<()> {
        let db = Database::from_input(include_str!("input.txt"))?;
        let res = db.ids.iter().filter(|id| db.is_fresh(**id)).count();
        assert_eq!(res, 525);
        Ok(())
    }

    #[test]
    fn part_two_example() -> Result<()> {
        let db = Database::from_input(include_str!("testinput.txt"))?;

        let mut ranges = vec![];
        for range in db.fresh_ranges {
            ranges = merge_ranges(ranges, range);
        }

        let res = ranges
            .iter()
            .map(|range| range.to.0 - range.from.0 + 1)
            .sum::<u64>();

        assert_eq!(res, 14);
        Ok(())
    }

    #[test]
    fn part_two_solution() -> Result<()> {
        let db = Database::from_input(include_str!("input.txt"))?;

        let mut ranges = vec![];
        for range in db.fresh_ranges {
            ranges = merge_ranges(ranges, range);
        }

        let res = ranges
            .iter()
            .map(|range| range.to.0 - range.from.0 + 1)
            .sum::<u64>();

        assert_eq!(res, 333892124923577);
        Ok(())
    }
}
