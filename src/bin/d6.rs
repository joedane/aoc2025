fn part1(input: &str) {
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

struct Data<'a> {
    data: &'a [u8],
    column_sizes: Vec<usize>,
    num_columns: usize,
    num_rows: usize,
    line_len: usize,
}

impl<'a> Data<'a> {
    fn new(data: &'a str) -> Self {
        let mut line_count = 0;
        let mut line_len = 0;
        let column_sizes: Vec<usize> = data
            .lines()
            .map(|line| {
                line_count += 1;
                line_len = line.len() + 1; //assume this is constant.  add one for the newline stripped by lines()
                line.split_whitespace()
                    .map(|s| s.len())
                    .collect::<Vec<usize>>()
            })
            .reduce(|acc, e| {
                acc.iter()
                    .zip(e.iter())
                    .map(|(a, b)| std::cmp::max(*a, *b))
                    .collect()
            })
            .unwrap();
        let num_columns = column_sizes.len();
        Self {
            data: data.as_bytes(),
            column_sizes,
            num_columns,
            num_rows: line_count,
            line_len,
        }
    }

    fn process_column(&self, col: usize) -> u64 {
        let column_start_index = self.column_start_index(col);
        let mut operands: Vec<u64> = vec![];
        let mut adding = true;
        for c in 0..self.column_sizes[col] {
            let i = column_start_index + self.column_sizes[col] - 1 - c;
            let mut char_value: u64 = 0;
            for row in 0..self.num_rows {
                match self.data[i + row * self.line_len] {
                    digit @ 48..=57 => {
                        char_value = (char_value * 10) + digit as u64 - 48u64;
                    }
                    42 => {
                        adding = false;
                    }
                    43 | 32 => {} // plus sign or space
                    bad @ _ => panic!("invalid character '{}'", bad),
                }
            }
            operands.push(char_value)
        }
        operands
            .into_iter()
            .reduce(|acc, x| if adding { acc + x } else { acc * x })
            .unwrap()
    }

    fn column_start_index(&self, col: usize) -> usize {
        self.column_sizes[0..col].iter().map(|s| s + 1).sum()
    }
}

fn part2(input: &str) {
    let data = Data::new(input);
    let mut total = 0;
    for c in 0..data.num_columns {
        total += data.process_column(c);
    }

    println!("{}", total);
}
fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d06.txt").unwrap();
    part2(&input);
}

#[rustfmt::skip]
const TEST: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;
