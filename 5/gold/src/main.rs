fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", solve(&text));
}

fn solve(str: &str) -> u16 {
    let middle = str.find("\n\n").unwrap();
    let (rules_str, updates) = str.split_at(middle);

    let rules = parse_rules(rules_str);

    let mut result: u16 = 0;

    for line in updates.lines() {
        let mut updates = parse_line(line);
        result += sort_line(&mut updates, &rules);
    }

    result
}

fn parse_rules(rules_str: &str) -> Vec<(u8, u8)> {
    let mut rules = Vec::<(u8, u8)>::new();

    for line in rules_str.lines() {
        if let Some((first, second)) = line.split_once('|') {
            let first = first.parse::<u8>().unwrap();
            let second = second.parse::<u8>().unwrap();

            rules.push((first, second));
        }
    }

    rules
}

fn parse_line(line: &str) -> Vec<u8> {
    let mut result = Vec::<u8>::new();

    for str in line.split(',') {
        if str.is_empty() {
            break;
        }

        result.push(str.parse::<u8>().unwrap());
    }

    result
}

fn sort_line(line: &mut [u8], rules: &[(u8, u8)]) -> u16 {
    let mut incorrect = false;

    for i_idx in 0..line.len() {
        for j_idx in (i_idx + 1)..line.len() {
            assert!(j_idx > i_idx);
            let i = line[i_idx];
            let j = line[j_idx];

            for rule in rules.iter() {
                if *rule == (j, i) {
                    line.swap(i_idx, j_idx);
                    incorrect = true;
                }
            }
        }
    }

    if let Some(result) = line.get(line.len() / 2) {
        if incorrect {
            *result as u16
        } else {
            0
        }
    } else {
        0
    }
}
