pub fn sample() -> String {
    r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#
    .trim()
    .to_string()
}
fn run(contents: &String, first: bool) -> Vec<(i32, i32)> {
    let (dots, folds) = contents.split_once("\n\n").unwrap();
    let mut dots: Vec<(i32, i32)> = dots
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<(&str, i32)> = folds
        .lines()
        .map(|l| l.split_once("g ").unwrap().1.split_once("=").unwrap())
        .map(|(x, y)| (x, y.parse().unwrap()))
        .collect();

    for (fold_axis, fold_index) in folds {
        for mut dot in &mut dots {
            if fold_axis == "y" {
                if dot.1 > fold_index {
                    dot.1 = fold_index - (dot.1 - fold_index)
                }
            } else {
                if dot.0 > fold_index {
                    dot.0 = fold_index - (dot.0 - fold_index)
                }
            }
        }
        if first {
            break;
        }
    }
    let mut res = vec![];
    dots.iter().fold(&mut res, |p, c| {
        if !p.contains(c) {
            p.push(*c);
        }
        p
    });
    res
}

pub fn part_1(contents: &String) -> u32 {
    run(contents, true).len() as u32
}

pub fn part_2(contents: &String) -> u32 {
    let dots = run(contents, false);
    let (max, may) = dots.iter().fold((0, 0), |p, c| {
        (
            if p.0 > c.0 { p.0 } else { c.0 },
            if p.1 > c.1 { p.1 } else { c.1 },
        )
    });
    let mut matrix = vec![vec![' '; max as usize + 1]; may as usize + 1];
    for (x, y) in dots {
        matrix[y as usize][x as usize] = 'x';
    }
    for line in matrix {
        for ch in line {
            print!("{}", ch);
        }
        print!("\n");
    }
    0
}
