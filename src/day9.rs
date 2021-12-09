pub fn sample() -> String {
    r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#
    .trim()
    .to_string()
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
    let rows: Vec<Vec<u32>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c as u32 - '0' as u32).collect())
        .collect();
    let adjacent: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    //let mut low_points = 0;
    //let mut ties = 0;
    let mut score = 0;
    for y in 0..rows.len() as i32 {
        'outer: for x in 0..rows[y as usize].len() as i32 {
            let h = rows[y as usize][x as usize];
            //println!("{}, {}: {}", x, y, h);
            for (dx, dy) in adjacent {
                let check_y = y + dy;
                let check_x = x + dx;
                if out_of_bounds(check_x, check_y, rows[0].len(), rows.len()) {
                    continue;
                }
                let other_h = rows[check_y as usize][check_x as usize];
                //println!("checking {}, {}: {}", check_x, check_y, other_h);

                if other_h < h {
                    //println!("found lower neighbour!");
                    continue 'outer;
                }
                if other_h == h {
                    //println!("found tie!");
                    //ties += 1;
                    continue 'outer;
                }
            }
            //println!("is lowest point!");
            //low_points += 1;
            score += h + 1;
        }
    }
    /*println!(
        "found {} low points with a total score of {} and {} ties",
        low_points, score, ties
    );*/
    score
}

pub fn part_2(contents: &String) -> u32 {
    let mut rows: Vec<Vec<u32>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c as u32 - '0' as u32).collect())
        .collect();
    let adjacent: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut low_points: Vec<(i32, i32)> = vec![];
    for y in 0..rows.len() as i32 {
        'outer: for x in 0..rows[y as usize].len() as i32 {
            let h = rows[y as usize][x as usize];
            //println!("{}, {}: {}", x, y, h);
            for (dx, dy) in adjacent {
                let check_y = y + dy;
                let check_x = x + dx;
                if out_of_bounds(check_x, check_y, rows[0].len(), rows.len()) {
                    continue;
                }
                if rows[check_y as usize][check_x as usize] <= h {
                    continue 'outer;
                }
            }
            low_points.push((x, y));
        }
    }

    let mut sizes: Vec<i32> = vec![];
    for (x, y) in low_points {
        let count = count_unvisited(&mut rows, x, y);
        sizes.push(count);
        //println!("{}, {}: {}", x, y, count);
    }
    sizes.sort();
    sizes.reverse();
    //println!("sizes: {:?}", sizes);

    (sizes[0] * sizes[1] * sizes[2]) as u32
}

fn count_unvisited(visited: &mut Vec<Vec<u32>>, x: i32, y: i32) -> i32 {
    //println!("counting {}, {}", x, y);
    let adjacent: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    visited[y as usize][x as usize] = 0;
    let mut count = 1;
    for (dx, dy) in adjacent {
        let check_y = y + dy;
        let check_x = x + dx;

        if out_of_bounds(check_x, check_y, visited[0].len(), visited.len()) {
            continue;
        }
        let other = visited[check_y as usize][check_x as usize];
        if other == 0 {
            continue;
        }
        if other == 9 {
            continue;
        }
        count += count_unvisited(visited, check_x, check_y);
    }
    count
}
