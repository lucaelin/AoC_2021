pub fn sample() -> String {
    r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#
    .trim()
    .to_string()
}

const ADJACENT: [(i32, i32); 8] = [(-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

struct Octopus {
    level: u32,
    flashes: u32,
    flashed: bool,
}

fn charge(map: &mut Vec<Vec<Octopus>>, x: i32, y: i32) {
    if out_of_bounds(x, y, map[0].len(), map.len()) {return}
    let mut me = &mut map[x as usize][y as usize];
    me.level += 1;
    if me.level > 9 && !me.flashed {
        me.flashed = true;
        me.flashes += 1;
        for (dx, dy) in ADJACENT {
            let x = x + dx;
            let y = y + dy;
            charge(map, x, y);
        }
    }
}

fn out_of_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    if x < 0 || y < 0 {
        return true;
    }
    if x as usize >= width || y as usize >= height {
        return true;
    }
    false
}

pub fn part_1(contents: &String) -> u32 {
    let mut map: Vec<Vec<Octopus>> = contents
        .lines()
        .map(|l| l.chars().map(|c| Octopus {level: c as u32 - '0' as u32, flashes: 0, flashed: false}).collect())
        .collect();

    for _ in 0..100 {
        for y in 0..map.len() as i32 {
            for x in 0..map[y as usize].len() as i32 {
                charge(&mut map, x, y);
            }
        }

        map.iter_mut().for_each(|l| l.iter_mut().for_each(|o| {
            o.flashed = false;
            if o.level > 9 {
                o.level = 0;
            } 
        }));
    }

    map.iter().fold(0, |p, row| p + row.iter().fold(0, |p, o| p + o.flashes))
}

pub fn part_2(contents: &String) -> u32 {
    let mut map: Vec<Vec<Octopus>> = contents
        .lines()
        .map(|l| l.chars().map(|c| Octopus {level: c as u32 - '0' as u32, flashes: 0, flashed: false}).collect())
        .collect();

    for day in 0..10000 {
        for y in 0..map.len() as i32 {
            for x in 0..map[y as usize].len() as i32 {
                charge(&mut map, x, y);
            }
        }
        if map.iter().all(|l| l.iter().all(|o| o.flashed)) {
            return day+1;
        }

        map.iter_mut().for_each(|l| l.iter_mut().for_each(|o| {
            o.flashed = false;
            if o.level > 9 {
                o.level = 0;
            } 
        }));
    }
    0
}
