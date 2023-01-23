use std::str::Lines;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{self, char};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn new(amount: u32, from: u32, to: u32) -> Self {
        Instruction {
            amount: amount as usize,
            from: from as usize,
            to: to as usize,
        }
    }
}

// Move `amount` crates from stack `from` to stack `to`.
// If `reverse` is true, treats the process like popping and pushing from a stack. If false, the
// order of the boxes is maintained during the move.
fn cranelift(boxes: &mut Vec<Vec<char>>, instruction: Instruction, reverse: bool) {
    let Instruction { amount, from, to } = instruction;
    if from == to {
        return;
    }

    let src;
    let dst;
    if from < to {
        let (split1, split2) = boxes.split_at_mut(from);
        src = &mut split1[from - 1];
        dst = &mut split2[to - from - 1];
    } else {
        let (split1, split2) = boxes.split_at_mut(to);
        dst = &mut split1[to - 1];
        src = &mut split2[from - to - 1];
    }

    if reverse {
        dst.extend(src.drain((src.len() - amount)..).rev());
    } else {
        dst.extend(src.drain((src.len() - amount)..));
    }
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let mut parser = alt((
        tag("   "),
        delimited(char('['), take(1 as usize), char(']')),
    ));
    let (remainder, res) = parser(input)?;
    let out = match res {
        "   " => None,
        v => Some(v.chars().collect::<Vec<_>>()[0]),
    };

    Ok((remainder, out))
}

fn parse_crate_row(line: &str) -> IResult<&str, Vec<Option<char>>> {
    let mut parser = separated_list1(tag(" "), parse_crate);
    parser(line)
}

// Build a the crate stacks from the string input split by lines.
// This will consume the line with stack numbers from `lines`.
fn parse_crate_stack(lines: &mut Lines) -> Vec<Vec<char>> {
    let mut crate_rows = Vec::new();
    while let Some(line) = lines.next() {
        match parse_crate_row(line) {
            Ok((_txt, row)) => crate_rows.push(row),
            Err(_) => break, // Finished parsing crates.
        }
    }

    // Convert the row based crates into column based crates.
    let n_cols = crate_rows.iter().map(|row| row.len()).max().unwrap();
    let mut crate_stacks: Vec<_> = (0..n_cols).into_iter().map(|_| Vec::new()).collect();
    // Reverse the rows since parsing is done from top to bottom, but the stack of crates should go
    // from bottom to top.
    for row in crate_rows.iter().rev() {
        for (i, entry) in row.iter().enumerate() {
            match entry {
                Some(c) => crate_stacks[i].push(c.clone()),
                None => (),
            }
        }
    }

    crate_stacks
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (txt, _) = tag("move ")(input)?;
    let (txt, amount) = complete::u32(txt)?;
    let (txt, _) = tag(" from ")(txt)?;
    let (txt, from) = complete::u32(txt)?;
    let (txt, _) = tag(" to ")(txt)?;
    let (txt, to) = complete::u32(txt)?;
    Ok((txt, Instruction::new(amount, from, to)))
}

fn parse_instructions(lines: &mut Lines) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        let (_txt, instruction) = parse_instruction(line).unwrap();
        instructions.push(instruction);
    }
    instructions
}

fn perform_update(input: &str, reverse: bool) -> Vec<Vec<char>> {
    let mut lines = input.lines();
    let mut boxes = parse_crate_stack(&mut lines);
    assert_eq!(Some(""), lines.next());
    let instructions = parse_instructions(&mut lines);

    for instruction in instructions {
        cranelift(&mut boxes, instruction, reverse);
    }

    boxes
}

fn part1(input: &str) -> String {
    let boxes = perform_update(input, true);
    boxes
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn part2(input: &str) -> String {
    let boxes = perform_update(input, false);
    boxes
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-05.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("5");
    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT1: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3
"#;

    static INPUT2: &str = r#"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_part1a() {
        let mut crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        cranelift(&mut crates, Instruction::new(1, 2, 1), true);
        assert_eq!(vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']], crates);
        cranelift(&mut crates, Instruction::new(3, 1, 3), true);
        assert_eq!(
            vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']],
            crates
        );
        cranelift(&mut crates, Instruction::new(2, 2, 1), true);
        assert_eq!(
            vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']],
            crates
        );
        cranelift(&mut crates, Instruction::new(1, 1, 2), true);
        assert_eq!(vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']], crates);
    }

    #[test]
    fn test_part1b() {
        assert_eq!(Some('D'), parse_crate("[D]").unwrap().1);
        assert_eq!(None, parse_crate("   ").unwrap().1);
    }

    #[test]
    fn test_part1c() {
        assert_eq!(vec![None, Some('D')], parse_crate_row("    [D]").unwrap().1);
        assert_eq!(
            vec![Some('N'), Some('C'), None],
            parse_crate_row("[N] [C]    ").unwrap().1
        );
        assert_eq!(
            vec![Some('Z'), Some('M'), Some('P')],
            parse_crate_row("[Z] [M] [P]").unwrap().1
        );
    }

    #[test]
    fn test_part1d() {
        let mut lines = INPUT1.lines();
        let crate_stack = parse_crate_stack(&mut lines);
        assert_eq!(
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            crate_stack
        );
        assert!(lines.next().is_none()); // Row of numbers is consumed.
    }

    #[test]
    fn test_part1e() {
        let mut lines = "move 1 from 2 to 1".lines();
        let (txt, instruction) = parse_instruction(lines.next().unwrap()).unwrap();
        assert_eq!(Instruction::new(1, 2, 1), instruction);
    }

    #[test]
    fn test_part1f() {
        let mut lines = INPUT2.lines();
        let instructions = parse_instructions(&mut lines);
        assert_eq!(
            vec![
                Instruction::new(1, 2, 1),
                Instruction::new(3, 1, 3),
                Instruction::new(2, 2, 1),
                Instruction::new(1, 1, 2),
            ],
            instructions,
        );
    }

    #[test]
    fn test_part1g() {
        let input = format!("{INPUT1}\n{INPUT2}");
        assert_eq!(
            vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']],
            perform_update(&input, true)
        );
        assert_eq!(
            vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']],
            perform_update(&input, false)
        );
    }

    #[test]
    fn test_part1h() {
        let input = format!("{INPUT1}\n{INPUT2}");
        assert_eq!("CMZ", &part1(&input));
    }

    #[test]
    fn test_part2a() {
        let input = format!("{INPUT1}\n{INPUT2}");
        assert_eq!("MCD", &part2(&input));
    }
}
