use std::fmt;

pub fn sample() -> String {
    r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"#
    .trim()
    .to_string()
}

struct Cell {
    number: u8,
    mark: bool,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mark {
            write!(f, " {:02} ", self.number)
        } else {
            write!(f, "({:02})", self.number)
        }
    }
}
struct Board {
    rows: Vec<Cell>,
    winner: bool,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, cell) in self.rows.iter().enumerate() {
            if i % 5 == 0 {
                write!(f, "\n").unwrap();
            }
            write!(f, "{}", cell).unwrap();
        }
        write!(f, "")
    }
}
impl Board {
    fn mark(&mut self, number: u8) {
        for e in self.rows.iter_mut() {
            if e.number == number {
                e.mark = true;
            }
        }
    }
    fn build(input: Vec<u8>) -> Board {
        let rows: Vec<Cell> = input
            .iter()
            .map(|v| Cell {
                number: *v,
                mark: false,
            })
            .collect();
        Self {
            rows,
            winner: false,
        }
    }
    fn score(&self, number: u8) -> u32 {
        number as u32
            * self
                .rows
                .iter()
                .fold(0u32, |p, c| p + if c.mark { 0u32 } else { c.number as u32 })
    }
    fn bingo(&self) -> bool {
        // rows
        for r in 0..5usize {
            for c in 0..5usize {
                let i = 5usize * r + c;
                if !self.rows[i].mark {
                    break;
                }
                if c == 4 {
                    println!("BINGO row {}!", r);
                    return true;
                }
            }
        }
        // columns

        for c in 0..5usize {
            for r in 0..5usize {
                let i = 5usize * r + c;
                if !self.rows[i].mark {
                    break;
                }
                if r == 4 {
                    println!("BINGO column {}!", c);
                    return true;
                }
            }
        }
        false
    }
}

fn parse(contents: &String) -> (Vec<u8>, Vec<Board>) {
    let mut entries: Vec<String> = contents.split("\n\n").map(|v| v.to_string()).collect();
    let numberline: Vec<String> = entries.drain(0..1).collect();
    let nums: Vec<u8> = numberline[0]
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let boards: Vec<Board> = entries
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<u8>().unwrap())
                .collect()
        })
        .map(|b| Board::build(b))
        .collect();

    (nums, boards)
}

pub fn part_1(contents: &String) -> u32 {
    let (numbers, mut boards) = parse(contents);
    //println!("numbers: {:?}", numbers);
    //println!("boards: {}", boards);

    for number in &numbers {
        println!("drawing {}", number);
        for board in &mut boards {
            board.mark(*number);
        }
        for board in &mut boards {
            if board.bingo() {
                println!("BINGO ON {}", board);
                println!("SCORE: {}", board.score(*number));
                return board.score(*number);
            }
        }
    }
    0
}

pub fn part_2(contents: &String) -> u32 {
    let (numbers, mut boards) = parse(contents);
    //println!("numbers: {:?}", numbers);
    //println!("boards: {}", boards);

    for number in &numbers {
        println!("drawing {}", number);
        for board in &mut boards {
            board.mark(*number);
        }

        let loosers = boards
            .iter()
            .fold(0u32, |p, c| p + if !c.winner { 1 } else { 0 });
        for board in &mut boards {
            if !board.winner && board.bingo() {
                if loosers == 1 {
                    println!("LAST BINGO ON {}", board);
                    println!("SCORE: {}", board.score(*number));
                    return board.score(*number);
                }
                board.winner = true;
            }
        }
    }
    0
}
