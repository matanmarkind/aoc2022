fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut per_elf: Vec<u32> = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    per_elf.sort_by_cached_key(|e| std::cmp::Reverse(*e));
    per_elf[0..3].iter().sum()
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-01.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();
    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT1: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn test_part1() {
        assert_eq!(24000, part1(INPUT1));
    }
}
