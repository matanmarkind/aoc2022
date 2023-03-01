use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::str::Lines;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_till,
    character::complete::{alpha1, newline},
    character::is_space,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Cd<'a> {
    Root,
    Parent,
    Child(&'a str),
}

#[derive(Debug, PartialEq)]
enum ListLine<'a> {
    Dir(&'a str),
    File(i64, &'a str),
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls,
}

#[derive(Debug)]
struct Directory {
    pub parent: Weak<Directory>,
    pub children: RefCell<HashMap<String, Rc<Directory>>>,
    pub files: RefCell<HashMap<String, i64>>,
}

impl Directory {
    pub fn root() -> Directory {
        Directory {
            parent: Weak::new(),
            children: RefCell::new(HashMap::new()),
            files: RefCell::new(HashMap::new()),
        }
    }

    pub fn child_of(parent: &Rc<Directory>) -> Directory {
        Directory {
            parent: Rc::downgrade(&parent),
            children: RefCell::new(HashMap::new()),
            files: RefCell::new(HashMap::new()),
        }
    }

    pub fn size(&self) -> usize {
        let fsizes = self.files.borrow().values().sum::<i64>() as usize;
        let dsizes: usize = self.children.borrow().values().map(|dir| dir.size()).sum();
        fsizes + dsizes
    }

    // Iterate over the children of the current directory. Doesn't include self.
    pub fn bfs(&self) -> DirectoryBfsIterator {
        DirectoryBfsIterator {
            upcoming: self
                .children
                .borrow()
                .iter()
                .map(|(name, dir)| (name.clone(), Rc::clone(dir)))
                .collect(),
        }
    }
}

#[derive(Debug)]
struct DirectoryBfsIterator {
    pub upcoming: std::collections::VecDeque<(String, Rc<Directory>)>,
}

// BFS iterator.
impl Iterator for DirectoryBfsIterator {
    type Item = (String, Rc<Directory>);

    fn next(&mut self) -> Option<Self::Item> {
        // We rely on the Directories having a strict tree structure, so no need to watch for
        // cycles.
        let (name, pwd) = self.upcoming.pop_front()?;
        self.upcoming.extend(
            pwd.children
                .borrow()
                .iter()
                .map(|(name, dir)| (name.clone(), Rc::clone(dir))),
        );
        Some((name, pwd))
    }
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (remainder, _) = tag("$ cd ")(input)?;
    let (remainder, dirname) = alt((tag(".."), tag("/"), alpha1))(remainder)?;
    let cd = match dirname {
        ".." => Cd::Parent,
        "/" => Cd::Root,
        name => Cd::Child(name),
    };
    Ok((remainder, Command::Cd(cd)))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (remainder, _) = tag("$ ls")(input)?;
    Ok((remainder, Command::Ls))
}

fn parse_dir(input: &str) -> IResult<&str, ListLine> {
    let (remainder, _) = tag("dir ")(input)?;
    Ok(("", ListLine::Dir(remainder)))
}

fn parse_file(input: &str) -> IResult<&str, ListLine> {
    let (remainder, (size, name)) = separated_pair(
        nom::character::complete::i64,
        tag(" "),
        take_till(char::is_whitespace),
    )(input)?;
    Ok((remainder, ListLine::File(size, name)))
}

fn handle_cd(cd: Cd, pwd: Rc<Directory>) -> Rc<Directory> {
    match cd {
        Cd::Root => {
            let mut root = pwd;
            while let Some(parent) = root.parent.upgrade() {
                root = parent;
            }
            root
        }
        Cd::Parent => pwd.parent.upgrade().unwrap_or(pwd),
        Cd::Child(dname) => Rc::clone(&pwd.children.borrow().get(dname).unwrap()),
    }
}

fn handle_ls(lines: &mut Peekable<Lines>, pwd: &Rc<Directory>) {
    let mut children = pwd.children.borrow_mut();
    let mut files = pwd.files.borrow_mut();
    loop {
        match lines.peek() {
            None => return,
            Some(&line) => {
                if line.starts_with("$") {
                    // Next line is a new command.
                    return;
                }
            }
        };

        let (remainder, line) = alt((parse_dir, parse_file))(lines.next().unwrap()).unwrap();
        match line {
            ListLine::Dir(name) => {
                let _ = children.insert(name.to_owned(), Rc::new(Directory::child_of(&pwd)));
            }
            ListLine::File(size, name) => {
                let _ = files.insert(name.to_owned(), size);
            }
        }
    }
}

fn build_fs(input: &str) -> Rc<Directory> {
    let root = Rc::new(Directory::root());
    let mut pwd = Rc::clone(&root);
    let mut lines = input.lines().peekable();
    assert_eq!(Some("$ cd /"), lines.next());

    while let Some(line) = lines.next() {
        let (remainder, cmd) = alt((parse_cd, parse_ls))(line).unwrap();
        match cmd {
            Command::Cd(cd) => {
                pwd = handle_cd(cd, Rc::clone(&pwd));
            }
            Command::Ls => handle_ls(&mut lines, &pwd),
        }
    }

    root
}

fn part1(input: &str) -> usize {
    let root = build_fs(input);
    // Just get the total and don't hold a map with the directory names. This is because I don't
    // support holding absolute paths and so repeat dirnames on different paths would create a
    // conflict.
    let mut total = if root.size() < 100000 { root.size() } else { 0 };
    for (_name, dir) in root.bfs() {
        if dir.size() <= 100000 {
            total += dir.size();
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let fs_size = 70000000;
    let required = 30000000;

    let root = build_fs(input);
    let lacking = required - (fs_size - root.size());
    let mut closest = root.size();
    for (_name, dir) in root.bfs() {
        if dir.size() >= lacking && (dir.size() - lacking) < (closest - lacking) {
            closest = dir.size()
        }
    }
    closest
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-07.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("7");
    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_parse_cd() {
        assert_eq!(("", Command::Cd(Cd::Root)), parse_cd("$ cd /").unwrap());
        assert_eq!(
            (" ", Command::Cd(Cd::Parent)),
            parse_cd("$ cd .. ").unwrap()
        );
        assert_eq!(
            ("", Command::Cd(Cd::Child("bfbjzfd"))),
            parse_cd("$ cd bfbjzfd").unwrap()
        );
    }

    #[test]
    fn test_parse_ls() {
        assert_eq!(("", Command::Ls), parse_ls("$ ls").unwrap());
    }

    #[test]
    fn test_parse_dir() {
        assert_eq!(
            ("", ListLine::Dir("asfd_12.k")),
            parse_dir("dir asfd_12.k").unwrap()
        );
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(
            ("", ListLine::File(123, "asfd_12.k")),
            parse_file("123 asfd_12.k").unwrap()
        );
    }

    #[test]
    fn test_part1a() {
        assert_eq!(95437, part1(INPUT))
    }
}
