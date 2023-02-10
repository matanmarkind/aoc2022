fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-07.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("7");
    // println!("{:?}", part1(&txt));
    // println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {}
}
