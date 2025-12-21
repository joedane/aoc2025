fn part1() {
    let mut data: Vec<Vec<&str>> = vec![];

    for line in input.lines().map(str::trim) {
        data.push(line.split_whitespace().collect());
    }
    let col_count = data[0].len();
    if !data.iter().all(|v| v.len() == col_count) {
        panic!();
    }
    let mut total: u64 = 0;
    let operand_count = data.len() - 1;
    for i in 0..col_count {
        let op = data.last().unwrap()[i];
        let operands = &data[0..operand_count];
        if op == "+" {
            total += operands
                .iter()
                .map(|l| l[i].parse::<u64>().unwrap())
                .inspect(|i| println!("adding {}", i))
                .fold(0, |acc, x| acc + x);
        } else if op == "*" {
            total += operands
                .iter()
                .map(|l| l[i].parse::<u64>().unwrap())
                .inspect(|i| println!("mul {}", i))
                .fold(1, |acc, x| acc * x);
        } else {
            panic!("invalid operation '{}'", op);
        }
    }
    println!("{}", total);
}

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d06.txt").unwrap();
}

const TEST: &str = r#"123 328  51 64 
45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
