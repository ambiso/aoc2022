use std::collections::HashMap;

use nom::Finish;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::{error::Result, util::read_string};

#[derive(Debug)]
struct Path<'a>(&'a str);

impl<'a> From<&'a str> for Path<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

fn parse_path(i: &str) -> IResult<&str, Path> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd<'a>(&'a str);

impl<'a> From<Path<'a>> for Cd<'a> {
    fn from(value: Path<'a>) -> Self {
        Self(value.0)
    }
}

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd::from)(i)
}

#[derive(Debug)]
enum Command<'a> {
    Ls,
    Cd(&'a str),
}

impl<'a> From<Ls> for Command<'a> {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl<'a> From<Cd<'a>> for Command<'a> {
    fn from(cd: Cd<'a>) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry<'a> {
    Dir(Path<'a>),
    File(u64, Path<'a>),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line<'a> {
    Command(Command<'a>),
    Entry(Entry<'a>),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

enum FsNode<'a> {
    File(u64),
    Dir(u64, HashMap<&'a str, Box<FsNode<'a>>>),
}

fn parse_tree(s: &str) -> FsNode {
    let mut tree: HashMap<&str, Box<FsNode>> = HashMap::new();
    let mut cur_dir = Vec::new();

    let mut total_size = 0;
    let lines = s
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    for l in lines {
        match l {
            Line::Command(c) => match c {
                Command::Ls => {}
                Command::Cd(path) => match path {
                    ".." => {
                        cur_dir.pop();
                    }
                    "/" => {
                        cur_dir.clear();
                    }
                    _ => {
                        cur_dir.push(path);
                    }
                },
            },
            Line::Entry(e) => match e {
                Entry::Dir(path) => {
                    insert_dir(&mut tree, &cur_dir, path);
                }
                Entry::File(size, path) => {
                    insert_file(&mut tree, &cur_dir, size, path);
                    total_size += size;
                }
            },
        }
    }
    FsNode::Dir(total_size, tree)
}

pub fn solve_a() -> Result<u64> {
    let s = read_string("inputs/day07a")?;
    let tree = parse_tree(&s);
    let mut dir_sizes = 0;
    walk_dirs(&tree, &mut |n| match n {
        FsNode::Dir(size, _) => {
            if *size <= 100000 {
                dir_sizes += size;
            }
        }
        _ => {}
    });

    Ok(dir_sizes)
}

fn walk_dirs(node: &FsNode, f: &mut impl FnMut(&FsNode) -> ()) {
    f(node);
    match node {
        FsNode::File(_) => {}
        FsNode::Dir(_size, tree) => {
            for (_, n) in tree {
                walk_dirs(n, f);
            }
        }
    }
}

fn insert_dir<'a: 'b + 'c, 'b, 'c>(
    mut tree: &'b mut HashMap<&'a str, Box<FsNode<'a>>>,
    cur_dir: &'c Vec<&'a str>,
    path: Path<'a>,
) {
    for p in cur_dir {
        tree = match &mut **tree.get_mut(p).unwrap() {
            FsNode::Dir(_subtree_size, tree) => tree,
            _ => panic!(),
        };
    }
    tree.insert(path.0, Box::new(FsNode::Dir(0, HashMap::new())));
}

fn insert_file<'a: 'b + 'c, 'b, 'c>(
    mut tree: &'b mut HashMap<&'a str, Box<FsNode<'a>>>,
    cur_dir: &'c Vec<&'a str>,
    size: u64,
    path: Path<'a>,
) {
    for p in cur_dir {
        tree = match &mut **tree.get_mut(p).unwrap() {
            FsNode::Dir(subtree_size, tree) => {
                *subtree_size += size;
                tree
            }
            _ => panic!(),
        };
    }
    tree.insert(path.0, Box::new(FsNode::File(size)));
}

pub fn solve_b() -> Result<u64> {
    let s = read_string("inputs/day07a")?;
    let tree = parse_tree(&s);
    let needed = 30000000;
    let total = 70000000;
    let used = match tree {
        FsNode::Dir(size, _) => size,
        _ => panic!(),
    };
    let unused = total - used;
    let needed = needed - unused;
    let mut smallest_dir = u64::MAX;

    walk_dirs(&tree, &mut |node| match node {
        FsNode::Dir(size, _) => {
            if *size >= needed {
                smallest_dir = smallest_dir.min(*size);
            }
        }
        _ => {}
    });

    Ok(smallest_dir)
}
