fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", solve(&text));

    assert_eq!(
        solve("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n"),
        61
    );
}

fn solve(str: &str) -> u16 {
    let middle = str.find("\n\n").unwrap();
    let (rules_str, updates) = str.split_at(middle);

    let rules = parse_rules(rules_str);

    let mut result: u16 = 0;

    for line in updates.lines() {
        let updates = parse_line(line);
        result += sort_line(&updates, &rules);
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

fn sort_line(line: &[u8], rules: &[(u8, u8)]) -> u16 {
    for i_idx in 0..line.len() {
        for j_idx in (i_idx + 1)..line.len() {
            assert!(j_idx > i_idx);
            let i = line[i_idx];
            let j = line[j_idx];

            for rule in rules.iter() {
                if *rule == (j, i) {
                    return 0;
                }
            }
        }
    }

    if let Some(result) = line.get(line.len() / 2) {
        *result as u16
    } else {
        0
    }
}
