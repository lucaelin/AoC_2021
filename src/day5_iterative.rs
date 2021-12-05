use std::cmp;
use std::fmt;
use std::str::FromStr;

type Position = (usize, usize);
type Direction = (i8, i8);
const WIDTH: usize = 1000;
const HEIGHT: usize = WIDTH;
struct Line {
    start: Position,
    end: Position,
}

#[derive(Debug)]
struct LineError {}
impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Encountered an invalid line when parsing input!")
    }
}
impl std::error::Error for LineError {}

impl FromStr for Line {
    type Err = LineError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(" -> ").ok_or(LineError {})?;

        let (left_x_str, left_y_str) = left.split_once(",").ok_or(LineError {})?;
        let left_x = left_x_str.parse().map_err(|_| LineError {})?;
        let left_y = left_y_str.parse().map_err(|_| LineError {})?;

        let (right_x_str, right_y_str) = right.split_once(",").ok_or(LineError {})?;
        let right_x = right_x_str.parse().map_err(|_| LineError {})?;
        let right_y = right_y_str.parse().map_err(|_| LineError {})?;

        Ok(Line {
            start: (left_x, left_y),
            end: (right_x, right_y),
        })
    }
}

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

fn parse(contents: &String) -> Vec<Line> {
    let lines = contents.lines().count();
    let mut vents: Vec<Line> = Vec::with_capacity(lines);
    for line in contents.lines() {
        vents.push(
            line.parse()
                .expect(format!("Invalid line {}", vents.len()).as_str()),
        );
    }
    vents
}

fn coords(pos: Position) -> usize {
    pos.1 * WIDTH + pos.0
}
fn dir(start: Position, end: Position) -> Direction {
    let x: i32 = end.0 as i32 - start.0 as i32;
    let y: i32 = end.1 as i32 - start.1 as i32;

    (
        cmp::min(1, cmp::max(x, -1)) as i8,
        cmp::min(1, cmp::max(y, -1)) as i8,
    )
}
fn is_diag(dir: Direction) -> bool {
    dir.0 != 0 && dir.1 != 0
}
fn move_pos(pos: Position, dir: Direction) -> Position {
    let x = (pos.0 as i32 + dir.0 as i32) as usize;
    let y = (pos.1 as i32 + dir.1 as i32) as usize;
    (x, y)
}
fn is_eq(pos1: Position, pos2: Position) -> bool {
    pos1.0 == pos2.0 && pos1.1 == pos2.1
}

fn solve(contents: &String, diagonals: bool) -> u32 {
    let vents = parse(contents);

    let mut matrix = vec![0; WIDTH * HEIGHT];

    for vent in vents {
        //println!("vector {:?}", (start, end));
        let mut pos = vent.start;
        let dir = dir(vent.start, vent.end);
        if !diagonals && is_diag(dir) {
            continue;
        }
        //println!("direction {:?}", dir);
        while !is_eq(pos, vent.end) {
            //println!("setting {:?}", pos);
            matrix[coords(pos)] += 1;
            pos = move_pos(pos, dir);
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
    let mut sum = 0;
    for v in matrix {
        if v > 1 {
            sum += 1
        };
    }
    sum
}

pub fn part_1(contents: &String) -> u32 {
    solve(contents, false)
}
pub fn part_2(contents: &String) -> u32 {
    solve(contents, true)
}
