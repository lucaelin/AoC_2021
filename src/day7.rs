use std::fmt;

pub fn sample() -> String {
    r#"
16,1,2,0,4,2,7,1,2,14
"#
    .trim()
    .to_string()
}

pub fn part_1(contents: &String) -> u32 {
    let pos = contents
        .split(",")
        .map(|r| r.parse().unwrap())
        .collect::<Vec<i32>>();

    let max = pos.iter().fold(0, |p, c| std::cmp::max(p, *c));
    let costs: Vec<i32> = (0..max)
        .map(|x| pos.iter().fold(0, |p, c| p + i32::abs(x - c)))
        .collect();

    let cheapest =
        costs.iter().enumerate().fold(
            (0, std::i32::MAX),
            |p, c| if p.1 < *c.1 { p } else { (c.0, *c.1) },
        );
    println!("{:?}", cheapest);

    cheapest.1 as u32
}
pub fn part_2(contents: &String) -> u32 {
    let pos = contents
        .split(",")
        .map(|r| r.parse().unwrap())
        .collect::<Vec<i32>>();

    let max = pos.iter().fold(0, |p, c| std::cmp::max(p, *c));
    let costs: Vec<i32> = (0..max)
        .map(|x| {
            pos.iter().fold(0, |p, c| {
                let d = i32::abs(x - c);
                p + { (d * (d + 1)) / 2 }
            })
        })
        .collect();

    let cheapest =
        costs.iter().enumerate().fold(
            (0, std::i32::MAX),
            |p, c| if p.1 < *c.1 { p } else { (c.0, *c.1) },
        );
    println!("{:?}", cheapest);

    cheapest.1 as u32
}
