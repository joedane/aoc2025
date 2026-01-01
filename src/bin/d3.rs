use std::collections::HashMap;

trait AsciiDigit {
    fn digit_value(&self) -> u64;
}

impl AsciiDigit for u8 {
    fn digit_value(&self) -> u64 {
        *self as u64 - 48
    }
}

#[allow(dead_code)]
fn process_part1(input: &[u8]) -> u64 {
    let (mut i_1, mut i_2) = (0usize, 1usize);
    'top: while i_2 < input.len() {
        for i in i_2..input.len() {
            if input[i] > input[i_1] && i < input.len() - 1 {
                i_1 = i;
                i_2 = i + 1;
                continue 'top;
            } else if input[i] > input[i_2] {
                i_2 = i;
                continue 'top;
            }
        }
        break 'top;
    }
    return (input[i_1].digit_value() * 10 + input[i_2].digit_value()) as u64;
}

#[allow(dead_code)]
fn part1(input: &str) {
    println!(
        "{}",
        input.lines().map(str::trim).map(|s| process_part1(s.as_bytes())). /* inspect(|v| println!("v: {}", v)). */ sum::<u64>()
    );
}

fn process_part2<'a>(digits: &'a [u8], n: u64, cache: &mut HashMap<(&'a [u8], u64), u64>) -> u64 {
    let v = cache.get(&(digits, n));
    match v {
        Some(v) => *v,
        None => {
            let v = if n == 0 {
                0
            } else if digits.len() == n as usize {
                std::str::from_utf8(digits).unwrap().parse::<u64>().unwrap()
            } else {
                std::cmp::max(
                    digits[0].digit_value() * 10u64.pow(n as u32 - 1)
                        + process_part2(&digits[1..], n - 1, cache),
                    process_part2(&digits[1..], n, cache),
                )
            };
            cache.insert((digits, n), v);
            v
        }
    }
}

fn part2(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(str::trim)
            .map(|s| process_part2(s.as_bytes(), 12, &mut Default::default()))
            .sum::<u64>()
    );
}

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d03.txt").unwrap();
    part2(&input);
}

#[allow(dead_code)]
static TEST: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;
