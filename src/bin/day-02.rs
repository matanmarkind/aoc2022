use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl RpsMove {
    fn points(&self) -> i32 {
        match self {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        }
    }

    fn shoot(&self, other: &Self) -> Outcome {
        match self {
            RpsMove::Rock => match other {
                RpsMove::Rock => Outcome::Draw,
                RpsMove::Paper => Outcome::Lose,
                RpsMove::Scissors => Outcome::Win,
            },
            RpsMove::Paper => match other {
                RpsMove::Rock => Outcome::Win,
                RpsMove::Paper => Outcome::Draw,
                RpsMove::Scissors => Outcome::Lose,
            },
            RpsMove::Scissors => match other {
                RpsMove::Rock => Outcome::Lose,
                RpsMove::Paper => Outcome::Win,
                RpsMove::Scissors => Outcome::Draw,
            },
        }
    }
}

impl FromStr for RpsMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RpsMove::Rock),
            "B" | "Y" => Ok(RpsMove::Paper),
            "C" | "Z" => Ok(RpsMove::Scissors),
            _ => Err("Invalid RPS move".to_string()),
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Invalid RPS outcome".to_string()),
        }
    }
}

fn points(other_move: RpsMove, my_move: RpsMove) -> i32 {
    let cmp_points = match my_move.shoot(&other_move) {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    cmp_points + my_move.points()
}

fn to_moves1(txt: &str) -> Vec<(RpsMove, RpsMove)> {
    // Inputs come in the form (other_move, my_move) and we parse this into a vec (entry per line).
    txt.lines()
        .map(|line| {
            let moves: Vec<RpsMove> = line
                .split(' ')
                .map(|s| s.parse::<RpsMove>().unwrap())
                .collect();
            assert_eq!(moves.len(), 2);
            (moves[0], moves[1])
        })
        .collect()
}

fn part1(txt: &str) -> i32 {
    to_moves1(txt)
        .iter()
        .map(|(other_move, my_move)| points(*other_move, *my_move))
        .sum()
}

fn to_moves2(txt: &str) -> Vec<(RpsMove, RpsMove)> {
    // Inputs come in the form (other_move, outcme) and we parse this into a vec of (other_move,
    // my_move). (one entry per line).
    txt.lines()
        .map(|line| {
            let inputs: Vec<_> = line.split(' ').collect();
            assert_eq!(inputs.len(), 2);
            let other_move: RpsMove = inputs[0].parse().unwrap();
            let outcome: Outcome = inputs[1].parse().unwrap();

            let my_move = match outcome {
                Outcome::Lose => match other_move {
                    RpsMove::Rock => RpsMove::Scissors,
                    RpsMove::Paper => RpsMove::Rock,
                    RpsMove::Scissors => RpsMove::Paper,
                },
                Outcome::Draw => other_move,
                Outcome::Win => match other_move {
                    RpsMove::Rock => RpsMove::Paper,
                    RpsMove::Paper => RpsMove::Scissors,
                    RpsMove::Scissors => RpsMove::Rock,
                },
            };

            (other_move, my_move)
        })
        .collect()
}

fn part2(txt: &str) -> i32 {
    to_moves2(txt)
        .iter()
        .map(|(other_move, my_move)| points(*other_move, *my_move))
        .sum()
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-02.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn test_part1a() {
        assert_eq!(8, points(RpsMove::Rock, RpsMove::Paper));
        assert_eq!(1, points(RpsMove::Paper, RpsMove::Rock));
        assert_eq!(6, points(RpsMove::Scissors, RpsMove::Scissors));
    }

    #[test]
    fn test_part1b() {
        let actual = to_moves1(INPUT);
        let expected = vec![
            (RpsMove::Rock, RpsMove::Paper),
            (RpsMove::Paper, RpsMove::Rock),
            (RpsMove::Scissors, RpsMove::Scissors),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1c() {
        assert_eq!(15, part1(INPUT));
    }

    #[test]
    fn test_part2a() {
        let actual = to_moves2(INPUT);
        let expected = vec![
            (RpsMove::Rock, RpsMove::Rock),
            (RpsMove::Paper, RpsMove::Rock),
            (RpsMove::Scissors, RpsMove::Rock),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2b() {
        assert_eq!(12, part2(INPUT));
    }
}
