use eyre::Result;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = eyre::ErrReport;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(eyre::format_err!("unknown direction: {value}")),
        }
    }
}

struct Lock {
    value: i32,
}

impl Lock {
    fn new() -> Self {
        Lock { value: 50 }
    }

    fn mv(&mut self, dir: Direction, value: i32) -> i32 {
        let dst = ((match dir {
            Direction::Left => self.value - value,
            Direction::Right => self.value + value,
        }) + 100)
            % 100;

        self.value = dst;

        println!("got {:?}{}, moving to {}", dir, value, dst);
        dst
    }

    fn mv_counting(&mut self, dir: Direction, value: i32) -> i32 {
        let mut wraps = i32::abs(value / 100); // 100 any direction will always go through 0
        let relative_move = value % 100;

        let mut dst = match dir {
            Direction::Left => self.value - relative_move,
            Direction::Right => self.value + relative_move,
        };

        if (self.value != 0 && dst <= 0) || dst > 99 {
            wraps += 1;
        }

        dst = (dst + 100) % 100;

        self.value = dst;

        println!(
            "got {:?}{}, moving to {}; wraps: {}",
            dir, value, dst, wraps
        );
        wraps
    }
}

fn main() -> Result<()> {
    let mut l = Lock::new();

    let input = include_str!("input.txt");
    let res: i32 = input
        .lines()
        .map(|line| {
            let (dir_str, num_str) = line.split_at(1);

            let dir = Direction::try_from(dir_str).unwrap();
            let value = num_str.parse::<i32>().unwrap();

            (dir, value)
        })
        .map(|(dir, value)| l.mv(dir, value))
        .filter(|elem| *elem == 0)
        .count() as i32;

    println!("res p1: {}", res);

    let mut l = Lock::new();

    let input = include_str!("input.txt");
    let res: i32 = input
        .lines()
        .map(|line| {
            let (dir_str, num_str) = line.split_at(1);

            let dir = Direction::try_from(dir_str).unwrap();
            let value = num_str.parse::<i32>().unwrap();

            (dir, value)
        })
        .map(|(dir, value)| l.mv_counting(dir, value))
        .sum();

    println!("res p2: {}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_movement() {
        let mut lock = Lock::new();
        assert_eq!(lock.mv(Direction::Left, 10), 40);
        assert_eq!(lock.mv(Direction::Right, 10), 50);
        assert_eq!(lock.mv(Direction::Left, 200), 50);
    }

    #[test]
    fn test_test_data_part_1() {
        let mut lock = Lock::new();
        let test_data = include_str!("testinput.txt");

        let res = test_data
            .lines()
            .map(|line| {
                let (dir_str, num_str) = line.split_at(1);

                let dir = Direction::try_from(dir_str).unwrap();
                let value = num_str.parse::<i32>().unwrap();

                (dir, value)
            })
            .map(|(dir, value)| lock.mv(dir, value))
            .filter(|elem| *elem == 0)
            .count() as i32;

        assert_eq!(res, 3);
    }

    #[test]
    fn test_test_data_part_2() {
        let mut lock = Lock::new();
        let test_data = include_str!("testinput.txt");

        let res: i32 = test_data
            .lines()
            .map(|line| {
                let (dir_str, num_str) = line.split_at(1);

                let dir = Direction::try_from(dir_str).unwrap();
                let value = num_str.parse::<i32>().unwrap();

                (dir, value)
            })
            .map(|(dir, value)| lock.mv_counting(dir, value))
            .sum();

        assert_eq!(res, 6);
    }
}
