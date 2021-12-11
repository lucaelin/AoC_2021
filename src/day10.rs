pub fn sample() -> String {
    r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#
    .trim()
    .to_string()
}

const OPENING: [char; 4] = ['(', '[', '{', '<'];

struct ParseError {
    found: char,
    expected: char
}

fn matching_char(open: char) -> Option<char> {
    match open {
        '(' => Some(')'),
        '<' => Some('>'),
        '{' => Some('}'),
        '[' => Some(']'),
        _ => None
    }
}

fn parse(rem: &str) -> Result<(&str, u64), ParseError> {
    //println!("Parsing: \"{}\"", rem);
    let first = rem.chars().next();
    if first.is_none() || !OPENING.contains(&first.unwrap()) {
        return Ok((&rem[0..0], 0));
    } 
    let mut inner_length = 0;
    let mut score = 0;
    loop {
        let (parsed, add_score) = parse(&rem[1+inner_length..rem.len()])?;
        //println!("Parsed inner: \"{}\"", parsed);
        score += add_score * 5;
        if parsed.len() == 0 {break;}
        inner_length += parsed.len();
    }
    let last = rem.chars().nth(1 + inner_length);
    if last.is_none() {
        // incomplete line
        let complete = matching_char(first.unwrap()).unwrap();
        //println!("Completing line with {} adding to score {}", complete, score);
        return Ok((rem, score + match complete {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        }));
    }
    let matching = matching_char(first.unwrap()).unwrap() == last.unwrap();
    if !matching {
        return Err(ParseError{ found: first.unwrap(), expected: last.unwrap() })
    }
    //println!("Parsed matching close");
    
    Ok((&rem[0..inner_length+2], score))
}

pub fn part_1(contents: &String) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut score = 0;
    for line in lines {
        let parsed = parse(&line);
        if parsed.is_err() {
            let err = parsed.unwrap_err();
            println!("Error on line {}: found: {} expected: {}", line, err.found, err.expected);
            score += match err.expected {
                ')' => 3,
                '>' => 25137,
                '}' => 1197,
                ']' => 57,
                _ => 0
            };
        } else {
            println!("line is ok!");
        }
    }
    score
}

pub fn part_2(contents: &String) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut scores = vec!();
    for line in lines {
        let mut rem: &str = line;
        loop {
            let parsed = parse(&rem);
            if parsed.is_ok() {
                let (parsed, add_score) = parsed.ok().unwrap();
                if parsed.len() < rem.len() {
                    rem = &rem[parsed.len()..rem.len()];
                    //println!("continue parsing, line not complete");
                    continue;
                }
                //println!("line needed completion, score: {}", add_score);
                scores.push(add_score);
                break;
            } else {
                //println!("line is invalid!");
                break;
            }
        }
    }
    //println!("scores {:?}", scores);
    scores.sort();
    scores[scores.len() / 2]
}
