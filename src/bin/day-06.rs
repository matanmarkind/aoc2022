use ringbuf::ring_buffer::Container;
use ringbuf::{LocalRb, Rb};

fn letter_to_index(c: char) -> u64 {
    // [1-52]
    match c {
        'a'..='z' => (c as u64) - ('a' as u64) + 1,
        'A'..='Z' => (c as u64) - ('A' as u64) + 27,
        _ => todo!(),
    }
}

fn msg_start(packet: &str, header_size: usize) -> usize {
    let mut buf = LocalRb::new(header_size);
    for (i, c) in packet.char_indices() {
        let bitmapped_index = 1 << letter_to_index(c);
        buf.push_overwrite(bitmapped_index); // No need to pop.
        let fold: u64 = buf.iter().fold(0, |acc, bitmap| acc | bitmap);

        if fold.count_ones() as usize == header_size {
            return i + 1;
        }
    }

    panic!("MSG NEVER STARTS!");
}

fn part1(packet: &str) -> usize {
    msg_start(packet, 4)
}

fn part2(packet: &str) -> usize {
    msg_start(packet, 14)
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-06.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("6");
    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part2a() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
