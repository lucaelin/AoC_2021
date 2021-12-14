use std::env;
use std::fs;
use std::time::Instant;

macro_rules! module {
    ($a:ident) => {
        mod $a;
        macro_rules! day {
            ($b:ident) => {
                $a::$b
            };
        }
    };
}

module!(day14);

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = if args.len() == 1 {
        day!(sample)()
    } else {
        let filename = &args[1];
        fs::read_to_string(filename).expect("Something went wrong reading the file")
    };
    let t = Instant::now();
    let res = day!(part_1)(&contents);
    let dt = Instant::now() - t;
    println!("part 1: {} (in {} µs)", res, dt.as_micros());

    let t = Instant::now();
    let res = day!(part_2)(&contents);
    let dt = Instant::now() - t;
    println!("part 2: {} (in {} µs)", res, dt.as_micros());
}
