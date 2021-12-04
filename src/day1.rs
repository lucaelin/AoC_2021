pub fn sample() -> String {
  r#"
199
200
208
210
200
207
240
269
260
263    
  "#
  .trim()
  .to_string()
}

pub fn part_1(contents: &String) -> u32 {
  let increases =
    contents
      .split("\n")
      .map(|v| v.parse().unwrap())
      .fold(
        (0, std::u32::MAX),
        |(s, p), c| if c > p { (s + 1, c) } else { (s, c) },
      );
  increases.0
}

pub fn part_2(contents: &String) -> u32 {
  let values: Vec<u32> = contents.split("\n").map(|v| v.parse().unwrap()).collect();

  let ret = values
    .windows(3)
    .map(|a| a[0] + a[1] + a[2])
    .fold(
      (0, std::u32::MAX),
      |(s, p), c| if c > p { (s + 1, c) } else { (s, c) },
    );
  ret.0
}
