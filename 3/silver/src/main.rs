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
        mul_it("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        161
    );
}

fn mul_it(str: &str) -> u32 {
    let muls = str.split("mul(");

    let mut result: u32 = 0;

    for mul in muls {
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
    }

    result
}
