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
    assert!(!analyze_line("1 3 2 4 5"));
    assert!(!analyze_line("8 6 4 4 1"));
    assert!(analyze_line("1 3 6 7 9"));
}

fn analyze_line(line: &str) -> bool {
    let mut last_level: Option<i8> = None;
    let mut increasing: Option<bool> = None;

    for level in line.split(' ') {
        let level = match level.parse::<i8>() {
            Ok(val) => val,
            Err(_) => return false,
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
