fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    let lines = text.split('\n');

    let mut safe_lines: u16 = 0;

    for line in lines {
        if analyze_line(line) {
            safe_lines += 1;
        }
    }

    println!("{}", safe_lines);

    assert!(analyze_line("7 6 4 2 1"));
    assert!(!analyze_line("1 2 7 8 9"));
    assert!(!analyze_line("9 7 6 2 1"));
    assert!(analyze_line("1 3 2 4 5"));
    assert!(analyze_line("8 6 4 4 1"));
    assert!(analyze_line("1 3 6 7 9"));
    assert!(!analyze_line("1 5 10"));
    assert!(!analyze_line("10 5 1"));
    assert!(analyze_line("1 2 3"));
    assert!(analyze_line("3 2 1"));
    assert!(analyze_line("1 3 3 5"));
    assert!(analyze_line("5 3 3 1"));
}

fn analyze_line(line: &str) -> bool {
    let mut levels: [Option<i8>; 8] = [None; 8];

    for (i, row) in line.split(' ').enumerate() {
        levels[i] = match row.parse::<i8>() {
            Ok(val) => Some(val),
            Err(_) => return false,
        };
    }

    for i in 0..levels.len() {
        let mut levels: [Option<i8>; 8] = levels;
        levels[i] = None;

        if try_analyze_line(levels) {
            return true;
        }
    }

    false
}

fn try_analyze_line(levels: [Option<i8>; 8]) -> bool {
    let mut last_level: Option<i8> = None;
    let mut increasing: Option<bool> = None;

    for level in levels {
        let level = match level {
            Some(val) => val,
            None => continue,
        };

        if let Some(last_level) = last_level {
            if level < last_level && level + 3 >= last_level {
                if let Some(increasing) = increasing {
                    if increasing {
                        return false;
                    }
                } else {
                    increasing = Some(false);
                }
            } else if level > last_level && level - 3 <= last_level {
                if let Some(increasing) = increasing {
                    if !increasing {
                        return false;
                    }
                } else {
                    increasing = Some(true);
                }
            } else {
                assert!(level < last_level + 3 || level + 3 > last_level);
                return false;
            }
        }

        last_level = Some(level);
    }

    true
}
