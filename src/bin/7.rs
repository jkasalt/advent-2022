use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;
use std::time::Instant;

#[derive(Default)]
struct Node {
    size: u64,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    fn new(input: &str) -> Rc<RefCell<Node>> {
        let root = Rc::new(RefCell::new(Node::default()));
        let mut node = root.clone();
        for line in input.lines() {
            if line.starts_with("$ cd") && line.ends_with('/') {
                continue;
            } else if line.starts_with("$ cd ..") {
                let parent = node.borrow().parent.clone().unwrap();
                node = parent;
            } else if line.starts_with("$ cd") {
                let child_name = line.split_whitespace().nth(2).unwrap().to_owned();
                let child = node
                    .borrow_mut()
                    .children
                    .entry(child_name)
                    .or_default()
                    .clone();
                node = child;
            } else if line.starts_with("dir") {
                let child_name = line.split_whitespace().nth(1).unwrap().to_owned();
                let child = node
                    .borrow_mut()
                    .children
                    .entry(child_name)
                    .or_default()
                    .clone();
                child.borrow_mut().parent = Some(node.clone());
            } else if let Ok(num) = line.split_once(' ').unwrap().0.parse() {
                let child_name = line.split_whitespace().nth(1).unwrap().to_owned();
                let file = node
                    .borrow_mut()
                    .children
                    .entry(child_name)
                    .or_default()
                    .clone();
                file.borrow_mut().size = num;
                file.borrow_mut().parent = Some(node.clone());
            }
        }
        root
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size
    }
}

fn sub_dirs(node: Rc<RefCell<Node>>) -> Box<dyn Iterator<Item = Rc<RefCell<Node>>>> {
    #[allow(clippy::needless_collect)]
    let node_children: Vec<_> = node.borrow().children.values().cloned().collect();
    Box::new(
        std::iter::once(node).chain(
            node_children
                .into_iter()
                .filter_map(|child| {
                    if child.borrow().children.is_empty() {
                        None
                    } else {
                        Some(sub_dirs(child))
                    }
                })
                .flatten(),
        ),
    )
}

fn p1(root: Rc<RefCell<Node>>) -> u64 {
    sub_dirs(root)
        .filter_map(|dir| {
            let total_size = dir.borrow().total_size();
            if total_size <= 100000 {
                Some(total_size)
            } else {
                None
            }
        })
        .sum()
}

fn p2(root: Rc<RefCell<Node>>) -> u64 {
    // let total_space = 70000000;
    // let needed_space = 30000000;
    let total_space = 3000000000;
    let needed_space = 700000000;
    let unused_space = total_space - root.borrow().total_size();
    sub_dirs(root)
        .filter_map(|dir| {
            let dir_size = dir.borrow().total_size();
            if dir_size + unused_space >= needed_space {
                Some(dir_size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let path = "bigboys/7/bigboy.txt";
    // let path = "inputs/7.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let node = Node::new(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));
    println!("-----");

    let i11 = Instant::now();
    let res1 = p1(node.clone());
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(node);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d7 {
    use super::*;

    #[test]
    fn d7t1() {
        let root = dbg!(Node::new(
            "$ cd /
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
7214296 k",
        ));
        assert_eq!(p1(root), 95437);
    }
}
