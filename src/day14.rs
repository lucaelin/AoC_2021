use itertools::Itertools;
use std::collections::HashMap;

const A: u32 = 'A' as u32;

pub fn sample() -> String {
    r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#
    .trim()
    .to_string()
}

fn polymerize(input: &Vec<char>, repl: &HashMap<String, char>) -> Vec<char> {
    let mut out: Vec<char> = input
        .windows(2)
        .flat_map(|both| {
            let inpt = String::from_iter(both);
            if repl.contains_key(&inpt) {
                vec![both[0], *repl.get(&inpt).unwrap()]
            } else {
                vec![both[0]]
            }
        })
        .collect();
    out.push(*input.last().unwrap());
    out
}

pub fn part_1(contents: &String) -> u32 {
    let (start, repl) = contents.split_once("\n\n").unwrap();
    let repl: HashMap<String, char> = repl.lines().map(|l| l.split_once(" -> ").unwrap()).fold(
        HashMap::new(),
        |mut m, (from, to)| {
            m.insert(from.to_string(), to.chars().nth(0).unwrap());
            m
        },
    );

    let mut polymer: Vec<char> = start.chars().collect();

    for _ in 0..10 {
        polymer = polymerize(&polymer, &repl);
        //println!("{:?}", polymer);
    }

    polymer.sort();
    let (smallest, largest, _) =
        polymer
            .windows(2)
            .fold((i32::MAX, 0, 0), |(mut s, mut l, mut c), ch| {
                c += 1;
                if ch[0] != ch[1] {
                    if c > l {
                        l = c;
                    }
                    if c < s {
                        s = c;
                    }
                    c = 0;
                }

                (s, l, c)
            });

    (largest - smallest) as u32
}
/*
fn index_2_char(index: u32) -> (char, char) {
    let right = (index % 26) as u32 + A;
    let left = (index / 26) as u32 + A;
    (left as u8 as char, right as u8 as char)
}
 */
fn char_2_index(char1: char, char2: char) -> u32 {
    (char1 as u32 - A) * 26 + (char2 as u32 - A)
}

pub fn part_2(contents: &String) -> u64 {
    let (start, repl) = contents.split_once("\n\n").unwrap();
    let repl: Vec<(u32, u32, u32)> = repl
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(pair, repl)| {
            (
                (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()),
                repl.chars().nth(0).unwrap(),
            )
        })
        .map(|(pair, repl)| {
            let value = char_2_index(pair.0, pair.1);
            let res1 = char_2_index(pair.0, repl);
            let res2 = char_2_index(repl, pair.1);
            (value, res1, res2)
        })
        .collect();

    /*
    for (a, b, c) in &repl {
        let a = index_2_char(*a);
        let b = index_2_char(*b);
        let c = index_2_char(*c);

        print!("{}{}: {}{} {}{}, ", a.0, a.1, b.0, b.1, c.0, c.1);
    }
    println!("\n");
    */

    let mut counts: Vec<u64> = vec![0; 26 * 26];

    for (ch1, ch2) in start.chars().tuple_windows() {
        counts[char_2_index(ch1, ch2) as usize] += 1;
    }

    for _ in 0..40 {
        let oldcounts = counts.clone();
        for rep in &repl {
            let old = oldcounts[rep.0 as usize];
            counts[rep.0 as usize] -= old;
            counts[rep.1 as usize] += old;
            counts[rep.2 as usize] += old;
        }
    }
    /*
    for (index, count) in counts.iter().enumerate() {
        let v = index_2_char(index as u32);
        if v.1 == 'A' {
            println!("");
        }

        print!("{}{}: {}, ", v.0, v.1, count);
    }
    println!("\n");
    */

    let mut letters: Vec<u64> = vec![0; 26];
    for (index, count) in counts.iter().enumerate() {
        letters[index % 26] += *count;
    }
    letters[(start.chars().nth(0).unwrap() as u32 - A) as usize] += 1;
    //println!("{:?}", letters);
    let smallest = letters.iter().filter(|v| **v > 0).min().unwrap();
    let largest = letters.iter().max().unwrap();
    largest - smallest
}
