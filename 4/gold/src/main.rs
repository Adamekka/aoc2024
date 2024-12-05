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

    assert_eq!(find_xmas("M.S\n.A.\nM.S"), 1);
    assert_eq!(
        find_xmas(".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n.........."),
        9
    )
}

fn find_xmas(str: &str) -> u16 {
    let mut result: u16 = 0;

    let x_pos = find_a(str);

    for x in x_pos {
        result += find_mas(str, x);
    }

    result
}

fn find_a(str: &str) -> Vec<Pos> {
    let mut x_pos: Vec<Pos> = Vec::new();

    for (line_idx, line) in str.lines().enumerate() {
        for (row_idx, c) in line.chars().enumerate() {
            if c == 'A' {
                x_pos.push(Pos::new(line_idx as i16, row_idx as i16));
            }
        }
    }

    x_pos
}

fn find_mas(s: &str, pos: Pos) -> u16 {
    let mut result: u16 = 0;

    macro_rules! gen_find_m {
        ($s:expr, $pos:expr, $first:expr, $second:expr, $third:expr, $fourth: expr) => {
            if find_char($first, $s, Pos::new($pos.line - 1, $pos.row - 1))
                && find_char($second, $s, Pos::new($pos.line - 1, $pos.row + 1))
                && find_char($third, $s, Pos::new($pos.line + 1, $pos.row - 1))
                && find_char($fourth, $s, Pos::new($pos.line + 1, $pos.row + 1))
            {
                result += 1;
            }
        };
    }

    gen_find_m!(s, pos, 'M', 'M', 'S', 'S'); // M M on top
    gen_find_m!(s, pos, 'S', 'M', 'S', 'M'); // S M on top
    gen_find_m!(s, pos, 'S', 'S', 'M', 'M'); // S S on top
    gen_find_m!(s, pos, 'M', 'S', 'M', 'S'); // M S on top

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
