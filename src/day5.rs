use std::cmp;

pub fn sample() -> String {
    r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2  
"#
    .trim()
    .to_string()
}

fn parse(contents: &String) -> Vec<((usize, usize), (usize, usize))> {
    contents
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|e| {
                    e.split(",")
                        .map(|c| c.parse().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|e| (e[0], e[1]))
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|e| (e[0], e[1]))
        .collect()
}

const WIDTH: usize = 1000;

fn coords(pos: (usize, usize)) -> usize {
    pos.1 * WIDTH + pos.0
}
fn dir(start: (usize, usize), end: (usize, usize)) -> (i8, i8) {
    let x: i32 = end.0 as i32 - start.0 as i32;
    let y: i32 = end.1 as i32 - start.1 as i32;

    (
        cmp::min(1, cmp::max(x, -1)) as i8,
        cmp::min(1, cmp::max(y, -1)) as i8,
    )
}

fn solve(contents: &String, diagonals: bool) -> u32 {
    let vents = parse(contents);

    let mut matrix = vec![0; WIDTH * WIDTH];

    for (start, end) in vents {
        println!("vector {:?}", (start, end));
        let mut pos = start;
        let dir = dir(start, end);
        if !diagonals && dir.0 != 0 && dir.1 != 0 {
            continue;
        }
        println!("direction {:?}", dir);
        while pos.0 != end.0 || pos.1 != end.1 {
            //println!("setting {:?}", pos);
            matrix[coords(pos)] += 1;
            pos.0 = (pos.0 as i32 + dir.0 as i32) as usize;
            pos.1 = (pos.1 as i32 + dir.1 as i32) as usize;
        }
        //println!("setting {:?}", pos);
        matrix[coords(pos)] += 1;
    }

    /* // print full matrix
    for (i, v) in matrix.iter().enumerate() {
      if i % WIDTH == 0 {
        print!("\n");
      }
      print!(" {:02}", v);
    }
    print!("\n");
    */

    matrix.iter().fold(0, |p, c| p + if *c > 1 { 1 } else { 0 })
}

pub fn part_1(contents: &String) -> u32 {
    solve(contents, false)
}
pub fn part_2(contents: &String) -> u32 {
    solve(contents, true)
}
