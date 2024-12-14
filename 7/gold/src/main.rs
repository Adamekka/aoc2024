#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Combine,
}

trait Next {
    fn next(&mut self) -> Result<(), ()>;
}

impl Next for Operation {
    fn next(&mut self) -> Result<(), ()> {
        *self = match self {
            Operation::Add => Operation::Multiply,
            Operation::Multiply => Operation::Combine,
            Operation::Combine => return Err(()),
        };

        Ok(())
    }
}

impl Next for [Operation] {
    fn next(&mut self) -> Result<(), ()> {
        let mut carry = true;

        for op in self.iter_mut() {
            if carry {
                match op.next() {
                    Ok(_) => carry = false,
                    Err(_) => *op = Operation::Add,
                }
            }
        }

        if carry {
            Err(())
        } else {
            Ok(())
        }
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", solve(&text));

    assert_eq!(solve("156: 15 6"), 156);
    assert_eq!(solve("7290: 6 8 6 15"), 7290);
}

fn solve(str: &str) -> u64 {
    let mut result = 0;

    for line in str.lines() {
        if line.is_empty() {
            break;
        }

        let mut line = line.split(':');
        let answer = line.next().unwrap().parse::<u64>().unwrap();

        let equation = line
            .next()
            .unwrap()
            .split(' ')
            .filter(|str| !str.is_empty())
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        assert_eq!(line.next(), None);

        if let Ok(v) = try_calculate(answer, &equation) {
            result += v
        }
    }

    result
}

fn try_calculate(real_answer: u64, equation: &[u64]) -> Result<u64, ()> {
    assert_ne!(equation.len(), 0);
    assert_ne!(equation.len(), 1);

    let mut operations = vec![Operation::Add; equation.len() - 1];

    loop {
        let mut equation = equation.iter();
        let mut my_answer = *equation.next().unwrap();

        for (idx, n) in equation.enumerate() {
            match operations[idx] {
                Operation::Add => my_answer += n,
                Operation::Multiply => my_answer *= n,
                Operation::Combine => {
                    let left = my_answer.to_string();
                    let right = n.to_string();
                    my_answer = (left + &right).parse::<u64>().unwrap();
                }
            }
        }

        if my_answer == real_answer {
            return Ok(real_answer);
        }

        match operations.next() {
            Ok(()) => (),
            Err(_) => return Err(()),
        }
    }
}
