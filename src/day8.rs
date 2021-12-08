pub fn sample() -> String {
    r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#
    .trim()
    .to_string()
}

/*
0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

fn p1p(wires: &str) -> Vec<u8> {
    match wires.len() {
        1 => vec![],
        2 => vec![1],
        3 => vec![7],
        4 => vec![4],
        5 => vec![2, 3, 5],
        6 => vec![0, 6, 9],
        7 => vec![8],
        _ => vec![],
    }
}

pub fn part_1(contents: &String) -> u32 {
    let entries: Vec<(Vec<&str>, Vec<&str>)> = contents
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(on, out)| (on.split(" ").collect(), out.split(" ").collect()))
        .collect();

    entries.iter().fold(0, |p, (_, out)| {
        p + out
            .iter()
            .fold(0, |p, c| p + if p1p(c).len() == 1 { 1 } else { 0 })
    }) as u32
}

/*
segment    numbers    count
a | 0   2 3   5 6 7 8 9 | 8
b | 0       4 5 6   8 9 | 6
c | 0 1 2 3 4     7 8 9 | 8
d |     2 3 4 5 6   8 9 | 7
e | 0   2       6   8   | 4
f | 0 1   3 4 5 6 7 8 9 | 9
g | 0   2 3   5 6   8 9 | 7
*/
fn p2p(oncount: i32, in_four: bool) -> char {
    match oncount {
        4 => 'e',
        6 => 'b',
        7 => {
            if in_four {
                'd'
            } else {
                'g'
            }
        }
        8 => {
            if in_four {
                'c'
            } else {
                'a'
            }
        }
        9 => 'f',
        _ => ' ',
    }
}

fn string_to_number(st: &String) -> u32 {
    match st.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => 10,
    }
}

pub fn part_2(contents: &String) -> u32 {
    let entries: Vec<(Vec<&str>, Vec<&str>)> = contents
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(on, out)| (on.split(" ").collect(), out.split(" ").collect()))
        .collect();

    let res: Vec<u32> = entries
        .iter()
        .map(|(observed, out)| {
            //println!("{:?} | {:?}", observed, out);
            let four = observed.iter().find(|on| on.len() == 4).unwrap();
            //println!("4 -> {}", four);
            let res: Vec<(char, char)> = ('a'..'h')
                .map(|c| {
                    let oncount = observed
                        .iter()
                        .fold(0, |p, on| p + if on.contains(c) { 1 } else { 0 });
                    //println!("{} -> {}", c, p2p(oncount, four.contains(c)));
                    (c, p2p(oncount, four.contains(c)))
                })
                .collect();
            let digits: Vec<u32> = out
                .iter()
                .map(|on| {
                    let translated: String = on
                        .chars()
                        .map(|c| res.iter().find(|(from, _)| c == *from).unwrap().1)
                        .collect();
                    let mut digit: Vec<char> = translated.chars().collect();
                    digit.sort();
                    let st: String = digit.iter().collect();
                    string_to_number(&st)
                })
                .collect();
            digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3] * 1
        })
        .collect();

    //println!("{:?}", res);
    res.iter().fold(0, |p, c| p + c)
}
