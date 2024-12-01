fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    let lines = text.split("\n");

    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();

    let mut result: i32 = 0;

    for line in lines {
        if line.is_empty() {
            break;
        }

        let (first, second) = line.split_at(5);
        let (_, second) = second.split_at(3);

        first_vec.push(first.parse::<i32>().unwrap());
        second_vec.push(second.parse::<i32>().unwrap());
    }

    first_vec.sort();
    second_vec.sort();

    for first in first_vec {
        for second in second_vec.iter() {
            if first == *second {
                result += first;
            }
        }
    }

    println!("{}", result);
}
