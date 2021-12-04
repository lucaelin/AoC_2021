pub fn sample() -> String {
  r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
  "#
  .trim()
  .to_string()
}

pub fn part_1(contents: &String) -> i32 {
  let coords = contents
    .split("\n")
    .map(|v| v.split(" "))
    .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap()))
    .fold((0, 0), |(x, z), (c, v)| match c {
      "forward" => (x + v, z),
      "backwards" => (x - v, z),
      "up" => (x, z + v),
      "down" => (x, z - v),
      _ => (x, z),
    });
  coords.0 * -coords.1
}

pub fn part_2(contents: &String) -> i32 {
  let coords = contents
    .split("\n")
    .map(|v| v.split(" "))
    .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap()))
    .fold((0, 0, 0), |(x, z, a), (c, v)| match c {
      "forward" => (x + v, z + v * a, a),
      "backwards" => (x - v, z - v * a, a),
      "up" => (x, z, a - v),
      "down" => (x, z, a + v),
      _ => (x, z, a),
    });
  coords.0 * coords.1
}
