#![feature(iter_array_chunks)]

fn letter_to_index(c: char) -> u64 {
    // [1-52]
    match c {
        'a'..='z' => (c as u64) - ('a' as u64),
        'A'..='Z' => (c as u64) - ('A' as u64) + 26,
        _ => todo!(),
    }
}

// Find the repeat character between the first and second half of the line.
fn find_repeat_char1(txt: &str) -> char {
    // Split the input in half and find the shared element.
    assert!(txt.is_ascii());
    let sack_len = txt.len() / 2;
    assert_eq!(txt.len(), sack_len * 2);
    let chunk1 = &txt[0..sack_len];
    let chunk2 = &txt[sack_len..];

    let mut part1_entries: u64 = 0; // bitmap with 52 entries.
    for c in chunk1.chars() {
        let i = 1 << letter_to_index(c);
        part1_entries |= i;
    }
    for c in chunk2.chars() {
        let i = 1 << letter_to_index(c);
        if part1_entries & i != 0 {
            return c;
        }
    }

    panic!("No repeat found");
}

fn part1(txt: &str) -> u64 {
    txt.lines()
        .inspect(|line| {
            println!(
                "{line} {} {}",
                find_repeat_char1(line),
                letter_to_index(find_repeat_char1(line))
            )
        })
        .map(|line| {
            let c = find_repeat_char1(line);
            1 + letter_to_index(c)
        })
        .sum()
}

// Find the repeat character between each entry.
fn find_repeat_char_index(txt: &[&str]) -> u64 {
    // Use bitmaps to find the repeat entry between lines.
    let badge_index: u64 = txt
        .iter()
        .map(|line| {
            line.chars()
                .fold(0, |acc, c| acc | (1 << letter_to_index(c)))
        })
        .fold(!0, |acc, bm| acc & bm);
    let res = badge_index.trailing_zeros() + 1;
    assert!(
        badge_index.is_power_of_two(), // Exactly one entry found
        "{res}, {badge_index:b}"
    );
    res as u64
}

fn part2(txt: &str) -> u64 {
    txt.lines()
        .array_chunks::<3>()
        .map(|lines| find_repeat_char_index(&lines))
        .sum()
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-03.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_part1a() {
        assert_eq!('p', find_repeat_char1("vJrwpWtwJgWrhcsFMMfFFhFp"));
    }

    #[test]
    fn test_part1b() {
        assert_eq!(157, part1(INPUT));
    }

    #[test]
    fn test_part2a() {
        assert_eq!(
            18,
            find_repeat_char_index(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ])
        );
        assert_eq!(
            52,
            find_repeat_char_index(&[
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ])
        );
    }

    #[test]
    fn test_part2b() {
        assert_eq!(70, part2(INPUT));
    }
}
