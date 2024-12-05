struct Pos {
    line: i16,
    row: i16,
}

impl Pos {
    fn new(line: i16, row: i16) -> Self {
        Pos { line, row }
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", find_xmas(&text));

    assert_eq!(find_xmas("S\nA\nM\nX"), 1);
    assert_eq!(find_xmas("S  S\n A A\n  MM\n   X"), 2);
    assert_eq!(
        find_xmas("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"),
        18
    );
}

fn find_xmas(str: &str) -> u16 {
    let mut result: u16 = 0;

    let x_pos = find_x(str);

    for x in x_pos {
        result += find_m(str, x);
    }

    result
}

fn find_x(str: &str) -> Vec<Pos> {
    let mut x_pos: Vec<Pos> = Vec::new();

    for (line_idx, line) in str.lines().enumerate() {
        for (row_idx, c) in line.chars().enumerate() {
            if c == 'X' {
                x_pos.push(Pos::new(line_idx as i16, row_idx as i16));
            }
        }
    }

    x_pos
}

#[allow(clippy::neg_multiply)]
fn find_m(s: &str, pos: Pos) -> u16 {
    let mut result: u16 = 0;

    macro_rules! gen_find_m {
        ($s:expr, $pos:expr, $offset_line:expr, $offset_row:expr) => {
            if find_char(
                'M',
                $s,
                Pos::new($pos.line + $offset_line, $pos.row + $offset_row),
            ) && find_char(
                'A',
                $s,
                Pos::new($pos.line + (2 * $offset_line), $pos.row + (2 * $offset_row)),
            ) && find_char(
                'S',
                $s,
                Pos::new($pos.line + (3 * $offset_line), $pos.row + (3 * $offset_row)),
            ) {
                result += 1;
            }
        };
    }

    gen_find_m!(s, pos, -1, 0); // Top
    gen_find_m!(s, pos, -1, 1); // Top-right
    gen_find_m!(s, pos, 0, 1); // Right
    gen_find_m!(s, pos, 1, 1); // Bottom-right
    gen_find_m!(s, pos, 1, 0); // Bottom
    gen_find_m!(s, pos, 1, -1); // Bottom-left
    gen_find_m!(s, pos, 0, -1); // Left
    gen_find_m!(s, pos, -1, -1); // Top-Left

    result
}

fn find_char(char: char, str: &str, pos: Pos) -> bool {
    let line = str.lines().nth(pos.line as usize);
    if let Some(line) = line {
        let c = line.chars().nth(pos.row as usize);
        if let Some(c) = c {
            if c == char {
                return true;
            }
        }
    }

    false
}
