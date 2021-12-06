use std::fmt;

#[derive(Debug)]
struct Fish {
    reproduce_in: u8,
}
impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reproduce_in)
    }
}

pub fn sample() -> String {
    r#"
3,4,3,1,2
"#
    .trim()
    .to_string()
}

pub fn part_1(contents: &String) -> u64 {
    let mut population = contents
        .split(",")
        .map(|r| Fish {
            reproduce_in: r.parse().unwrap(),
        })
        .collect::<Vec<Fish>>();

    let mut offspring: Vec<Fish> = vec![];

    for g in 0..80 {
        println!("Generation: {}", g);
        for mut fish in population.iter_mut() {
            if fish.reproduce_in == 0 {
                offspring.push(Fish { reproduce_in: 8 });
                fish.reproduce_in = 7;
            }
            fish.reproduce_in -= 1;
        }
        population.append(&mut offspring);
        offspring.clear();
        //for fish in &population {
        //    print!("{}, ", fish.reproduce_in);
        //}
        println!("Population size {}", population.len());
    }
    population.len() as u64
}
pub fn part_2(contents: &String) -> u64 {
    let mut fishdays = vec![0u64; 7];

    let population = contents
        .split(",")
        .map(|r| r.parse().unwrap())
        .collect::<Vec<usize>>();

    for fish in population {
        fishdays[fish] += 1;
    }

    let mut baby_day3 = 0;
    let mut baby_day2 = 0;
    let mut baby_day1 = 0;

    println!("Starting day 0");
    //println!("Population {:?} + {} + {}", fishdays, baby_day1, baby_day2);
    let size = fishdays.iter().fold(0, |p, c| p + c) + baby_day1 + baby_day2;
    println!("Size {:?}", size);

    for d in 0..256 {
        let day = d % 7;
        println!("Day {} ({})", d + 1, day);
        fishdays.rotate_left(1);
        fishdays[6] += baby_day1;
        baby_day1 = baby_day2;
        baby_day2 = baby_day3;
        baby_day3 = fishdays[0];
        //println!("Population {:?} + {} + {}", fishdays, baby_day1, baby_day2);
        let size = fishdays.iter().fold(0, |p, c| p + c) + baby_day1 + baby_day2;
        println!("Size {:?}", size);
    }

    fishdays.iter().fold(0, |p, c| p + c) + baby_day1 + baby_day2
}
