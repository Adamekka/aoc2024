trait IsInt {
    fn is_int(&self) -> bool;
}

impl IsInt for String {
    fn is_int(&self) -> bool {
        for c in self.chars() {
            if !c.is_ascii_digit() {
                return false;
            }
        }

        true
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", mul_it(&text));

    assert_eq!(mul_it("mul(2,4)"), 8);
    assert_eq!(
        mul_it("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
    assert_eq!(
        mul_it("xxxmul(1,1)xdon't()mul(1,2)xxxdo()xxxxmul(1,2)xxxdon't()mul(1,3)xxxmul(1,3)xxxx"),
        3
    );
}

fn mul_it(str: &str) -> u32 {
    let muls = str.split("mul(");

    let mut enabled = true;

    let mut result: u32 = 0;

    for mul in muls {
        if !enabled {
            if let Some(e) = find_does_and_does_not(mul) {
                enabled = e
            }
            continue;
        }

        let expression = mul.split_once(',');
        if let Some(expression) = expression {
            let first = match expression.0.parse::<u32>() {
                Ok(val) => val,
                Err(_) => continue,
            };

            let mut second = expression.1.to_owned();
            let mut c = '\0';

            while !second.is_int() {
                c = second.pop().unwrap();
            }

            if c != ')' {
                continue;
            }

            let second = match second.parse::<u32>() {
                Ok(val) => val,
                Err(_) => continue,
            };

            result += first * second;
        }

        if let Some(e) = find_does_and_does_not(mul) {
            enabled = e
        }
    }

    result
}

fn find_does_and_does_not(mul: &str) -> Option<bool> {
    let rev = mul.chars().rev().collect::<String>();
    let does = rev.find(")(od");
    let does_not = rev.find(")(t'nod");

    if let Some(does) = does {
        if let Some(does_not) = does_not {
            Some(does < does_not) // This is not needed
        } else {
            Some(true)
        }
    } else if does_not.is_some() {
        Some(false)
    } else {
        None
    }
}
