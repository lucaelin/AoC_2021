use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

pub fn sample() -> String {
    r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#
    .trim()
    .to_string()
}
type Path<'a> = (Vec<&'a str>, bool);
struct Cave<'a> {
    name: &'a str,
    small: bool,
    connections: Vec<&'a RefCell<Cave<'a>>>,
}

impl<'a> Cave<'a> {
    fn new(name: &'a str) -> Cave<'a> {
        Cave {
            name,
            small: name.chars().all(char::is_lowercase),
            connections: vec![],
        }
    }
}

impl<'a> std::fmt::Debug for Cave<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - ", self.name)?;
        for cave in &self.connections {
            write!(f, "{}, ", cave.deref().borrow().name)?;
        }
        write!(f, "")
    }
}
impl<'a> PartialEq for Cave<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn build_caves<'a>(connections: &'a Vec<(&str, &str)>) -> HashMap<&'a str, RefCell<Cave<'a>>> {
    let mut caves: HashMap<&str, RefCell<Cave>> = HashMap::new();
    for (start, end) in connections {
        if !caves.contains_key(start) {
            caves.insert(start, RefCell::new(Cave::new(start)));
        }
        if !caves.contains_key(end) {
            caves.insert(end, RefCell::new(Cave::new(end)));
        }
    }
    caves
}

pub fn part_1(contents: &String) -> u32 {
    let connections: Vec<(&str, &str)> = contents
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .collect();
    let caves = build_caves(&connections);

    for (start, end) in &connections {
        {
            let mut start_cave = caves.get(start).unwrap().borrow_mut();
            let end_cave = caves.get(end).unwrap();
            start_cave.connections.push(end_cave);
        }
        {
            let start_cave = caves.get(start).unwrap();
            let mut end_cave = caves.get(end).unwrap().borrow_mut();
            end_cave.connections.push(start_cave);
        }
    }
    //println!("{:?}", caves);
    let start = caves.get("start").unwrap().borrow();
    let start_name = start.name;
    let paths = route(start, (vec![start_name], true));
    //println!("{:?}", paths);
    paths.len() as u32
}

fn route<'a, 'b>(cave: Ref<'b, Cave<'a>>, path: Path<'a>) -> Vec<Path<'a>> {
    let mut path = path.clone();
    path.0.push(cave.name);
    let paths: Vec<Vec<Path<'a>>> = cave
        .connections
        .iter()
        .map(|c| c.deref().borrow())
        .filter_map(|destination| {
            let mut path = path.clone();
            if destination.name == "end" {
                let mut path = path.clone();
                path.0.push(destination.name);
                return Some(vec![path]);
            }
            if destination.name == "start" {
                return None;
            }
            if destination.small && path.1 && path.0.contains(&destination.name) {
                return None;
            }
            if destination.small && !path.1 && path.0.contains(&destination.name) {
                path.1 = true;
            }
            Some(route(destination, path))
        })
        .collect();
    let mut ret: Vec<Path<'a>> = vec![];
    for mut path in paths {
        ret.append(&mut path);
    }
    ret
}

pub fn part_2(contents: &String) -> u32 {
    let connections: Vec<(&str, &str)> = contents
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .collect();
    let caves = build_caves(&connections);

    for (start, end) in &connections {
        {
            let mut start_cave = caves.get(start).unwrap().borrow_mut();
            let end_cave = caves.get(end).unwrap();
            start_cave.connections.push(end_cave);
        }
        {
            let start_cave = caves.get(start).unwrap();
            let mut end_cave = caves.get(end).unwrap().borrow_mut();
            end_cave.connections.push(start_cave);
        }
    }
    //println!("{:?}", caves);
    let start = caves.get("start").unwrap().borrow();
    let start_name = start.name;
    let paths = route(start, (vec![start_name], false));
    //println!("{:?}", paths);
    paths.len() as u32
}
