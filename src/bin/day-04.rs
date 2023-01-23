use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::separated_pair;
use nom::IResult;

type MyRange = RangeInclusive<i32>;

// Like `merge_ranges` but assumes i1 starts first.
fn merged_ranges_inner(i1: MyRange, i2: MyRange) -> (MyRange, Option<MyRange>) {
    if i2.start() > i1.end() {
        // Disjoint ranges.
        return (i1, Some(i2));
    }

    if i2.end() <= i1.end() {
        // `i2` is totally contained by `i1`.
        return (i1, None);
    } else if i1.start() == i2.start() && i1.end() < i2.end() {
        // `i1` is totally contained by `i2`.
        return (i2, None);
    }

    let start = *i1.end() + 1;
    return (i1, Some(start..=*i2.end()));
}

// Merge 2 ranges together into disjoint ranges. This is done by removing the overlap, neither range
// is increased in size.
//
// Returns None if one of the ranges is entirely included in another.
fn merge_ranges(i1: MyRange, i2: MyRange) -> (MyRange, Option<MyRange>) {
    if i1.start() <= i2.start() {
        return merged_ranges_inner(i1, i2);
    } else {
        return merged_ranges_inner(i2, i1);
    }
}

// "X-Y" -> X..=Y.
fn parse_range(input: &str) -> IResult<&str, MyRange> {
    let mut parser = separated_pair(complete::i32, tag("-"), complete::i32);
    let (remainder, (start, end)) = parser(input)?;
    Ok((remainder, start..=end))
}

// Parse both entries in a line and merge the results.
// "A-B,C-D" -> (A..=B, C..=D)
fn parse_line_raw(input: &str) -> IResult<&str, (MyRange, MyRange)> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}

fn parse_line(input: &str) -> IResult<&str, (MyRange, Option<MyRange>)> {
    let (remainder, (r1, r2)) = parse_line_raw(input)?;
    Ok((remainder, merge_ranges(r1, r2)))
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .enumerate()
        .filter(|(i, line)| {
            let (remainder, (_r1, r2)) = parse_line(line).unwrap();
            assert_eq!("", remainder);
            if r2.is_none() {
                println!("{} {line}", i + 1);
                return true;
            }
            false
        })
        .count() as i32
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .enumerate()
        .filter(|(i, line)| {
            let (remainder, (r1, r2)) = parse_line_raw(line).unwrap();
            assert_eq!("", remainder);
            let (merged1, merged2) = merge_ranges(r1.clone(), r2.clone());
            match merged2 {
                None => true,
                Some(m2) => {
                    (r1.clone(), r2.clone()) != (merged1.clone(), m2.clone())
                        && (r1, r2) != (m2, merged1)
                }
            }
        })
        .count() as i32
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-04.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("4");
    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_part1a() {
        assert_eq!((2..=3, Some(4..=5)), merge_ranges(2..=3, 4..=5));
        assert_eq!((2..=8, None), merge_ranges(2..=8, 3..=7));
        assert_eq!((2..=6, Some(7..=8)), merge_ranges(2..=6, 4..=8));
        assert_eq!((25..=54, None), merge_ranges(25..=30, 25..=54));
    }

    #[test]
    fn test_part1b() {
        assert_eq!(2..=3, parse_range("2-3").unwrap().1);
        assert_eq!(4..=5, parse_range("4-5").unwrap().1);
    }

    #[test]
    fn test_part1c() {
        assert_eq!((5..=7, Some(8..=9)), parse_line("5-7,7-9").unwrap().1);
    }

    #[test]
    fn test_part1d() {
        assert_eq!(2, part1(INPUT));
    }
}
