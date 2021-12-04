pub fn sample() -> String {
  r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010    
    "#
  .trim()
  .to_string()
}

pub fn vec_to_int(vvec: Vec<u8>) -> u32 {
  let vstr = vvec
    .iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join("");

  u32::from_str_radix(&vstr, 2).unwrap()
}

pub fn part_1(contents: &String) -> u32 {
  let bin = contents
    .lines()
    .map(|l| l.chars().map(|v| v.to_string().parse().unwrap()).collect())
    .collect::<Vec<Vec<u8>>>();

  let count = bin.iter().fold(vec![0u32; bin[0].len()], |mut p, c| {
    for (i, v) in c.iter().enumerate() {
      p[i] += *v as u32;
    }
    p
  });
  let mut gammavec = vec![0; bin[0].len()];
  let mut epsilvec = vec![0; bin[0].len()];

  let mincount: u32 = bin.len() as u32 / 2;
  for (i, c) in count.iter().enumerate() {
    gammavec[i] = if c > &mincount { 1 } else { 0 };
    epsilvec[i] = if c < &mincount { 1 } else { 0 };
  }
  let gamma = vec_to_int(gammavec);
  let epsil = vec_to_int(epsilvec);
  gamma * epsil
}

fn count_pos(input: &Vec<Vec<u8>>, pos: usize) -> (u32, u32) {
  let ones = input.iter().fold(0u32, |p, v| p + v[pos] as u32);
  let zeros: u32 = input.len() as u32 - ones;
  return (ones, zeros);
}
fn filter_pos(input: &Vec<Vec<u8>>, pos: usize, value: u8) -> Vec<Vec<u8>> {
  input
    .iter()
    .filter(|m| m[pos] == value)
    .map(|v| v.clone()) // TODO: get rid of
    .collect()
}

pub fn part_2(contents: &String) -> u32 {
  let bin = contents
    .lines()
    .map(|l| l.chars().map(|v| v.to_string().parse().unwrap()).collect())
    .collect::<Vec<Vec<u8>>>();

  let oxy = {
    let oxyvec = (0..12usize).fold(bin.clone(), |matches, i| {
      if matches.len() == 1 {
        return matches;
      }
      //println!("step {:?} remaining {:?}", i, matches);
      let (ones, zeros) = count_pos(&matches, i);
      //println!("step {:?} found {:?} ones {:?} zeros", i, ones, zeros);
      let v = if ones >= zeros { 1 } else { 0 };
      //println!("step {:?} v {:?}", i, v);
      filter_pos(&matches, i, v)
    });
    //println!("result {:?}", oxyvec);
    vec_to_int(oxyvec[0].clone())
  };
  println!("{:?}", oxy);
  let co2 = {
    let co2vec = (0..12usize).fold(bin.clone(), |matches, i| {
      if matches.len() == 1 {
        return matches;
      }
      //println!("step {:?} remaining {:?}", i, matches);
      let (ones, zeros) = count_pos(&matches, i);
      //println!("step {:?} found {:?} ones {:?} zeros", i, ones, zeros);
      let v = if ones < zeros { 1 } else { 0 };
      //println!("step {:?} v {:?}", i, v);
      filter_pos(&matches, i, v)
    });
    //println!("result {:?}", co2vec);
    vec_to_int(co2vec[0].clone())
  };

  println!("{:?}", co2);

  oxy * co2
}
