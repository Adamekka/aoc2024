#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

trait Next {
    fn next(&mut self) -> Result<(), ()>;
}

impl Next for [Operation] {
    fn next(&mut self) -> Result<(), ()> {
        for (idx, op) in self.iter_mut().enumerate() {
            if *op == Operation::Add {
                *op = Operation::Multiply;

                for op in &mut self[..idx] {
                    *op = Operation::Add;
                }

                return Ok(());
            }
        }

        Err(())
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", solve(&text));

    assert_eq!(solve("190: 10 19"), 190);
    assert_eq!(solve("3267: 81 40 27"), 3267);
    assert_eq!(solve("292: 11 6 16 20"), 292);
    assert_eq!(solve("190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20"), 3749);
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
