use std::collections::HashSet;
use std::{fmt::Error, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(Error),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

impl Point {
    fn move_to(&mut self, Point { x, y }: &Point) {
        self.x = *x;
        self.y = *y;
    }

    fn maybe_move_towards(&mut self, Point { x, y }: &Point) {
        if (self.x - x).abs() < 2 && (self.y - y).abs() < 2 {
            return;
        }

        if (self.x - x) >= 2 {
            self.x = x + 1;
        } else if (self.x - x) <= -2 {
            self.x = x - 1;
        } else {
            self.x = *x;
        }

        if (self.y - y) >= 2 {
            self.y = y + 1;
        } else if (self.y - y) <= -2 {
            self.y = y - 1;
        } else {
            self.y = *y;
        }
    }
}

fn parse_row(line: &str) -> (Direction, i32) {
    let v: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(v.len(), 2);
    (Direction::from_str(v[0]).unwrap(), v[1].parse().unwrap())
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input.lines().map(|line| parse_row(line)).collect()
}

fn part1(input: &str) -> HashSet<(i32, i32)> {
    let movements = parse_input(input);
    let (mut hx, mut hy, mut tx, mut ty) = (0 as i32, 0 as i32, 0, 0);
    let mut tail_locations = HashSet::new();
    tail_locations.insert((tx, ty));

    for (direction, steps) in movements {
        for i in 0..steps {
            match direction {
                Direction::Right => hx += 1,
                Direction::Left => hx -= 1,
                Direction::Up => hy += 1,
                Direction::Down => hy -= 1,
            }

            if (tx - hx).abs() < 2 && (ty - hy).abs() < 2 {
                continue;
            }

            if (tx - hx) >= 2 {
                tx = hx + 1;
            } else if (tx - hx) <= -2 {
                tx = hx - 1;
            } else {
                tx = hx;
            }

            if (ty - hy) >= 2 {
                ty = hy + 1;
            } else if (ty - hy) <= -2 {
                ty = hy - 1;
            } else {
                ty = hy;
            }

            tail_locations.insert((tx, ty));
        }
    }

    tail_locations
}

fn part2(input: &str) -> HashSet<(i32, i32)> {
    const ROPE_LEN: usize = 10;
    let movements = parse_input(input);
    let mut tail_locations = HashSet::new();
    tail_locations.insert((0, 0));

    let mut rope: [Point; ROPE_LEN] = core::array::from_fn(|_| Point::default());
    for (direction, steps) in movements {
        for i in 0..steps {
            rope[0] = rope[0].clone() + direction;
            for i in (1..ROPE_LEN) {
                let (lead, follow) = rope.split_at_mut(i);
                follow[0].maybe_move_towards(lead.last_mut().unwrap());
            }

            let tail = rope.last().unwrap();
            tail_locations.insert((tail.x, tail.y));
        }
    }

    tail_locations
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-09.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("9");
    println!("{:?}", part1(&txt).len());
    println!("{:?}", part2(&txt).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![
                (Direction::Right, 4 as i32),
                (Direction::Up, 4),
                (Direction::Down, 1),
                (Direction::Left, 5)
            ],
            parse_input(
                "R 4
U 4
D 1
L 5"
            )
        );
    }

    #[test]
    fn test_part1() {
        let locs = part1(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(13, locs.len());
    }

    #[test]
    fn test_part2a() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part2(INPUT).len(), 1);
    }

    #[test]
    fn test_part2b() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(INPUT).len(), 36);
    }
}
