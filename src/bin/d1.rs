use std::io::Read;


fn part1(input: &str) {
    let mut p: i32 = 50;
    let mut zeros: u32 = 0;
    const DIAL_SIZE: i32 = 100;

    for line in input.lines().map(str::trim) {
        let (dir, mut cnt) = match line.as_bytes() {
            &[d, ref rest @ ..] => {
                (d, str::from_utf8(rest).unwrap().parse::<i32>().unwrap())
            },
            _ => panic!(),
        };
        if dir == b'L' {
            cnt *= -1;
        }

        p = (p + cnt).rem_euclid(DIAL_SIZE);
        if p == 0 {
            zeros += 1;
        }
//        println!("after '{}' pointer is at {}", line, p);
    }
    println!("zero {} times", zeros);
}

fn part2(input: &str) {
    const DIAL_SIZE: i32 = 100;
    let mut zeros: u32 = 0;
    let mut p: i32 = 50;
    for line in input.lines().map(str::trim) {
        let (dir, mut cnt) = match line.as_bytes() {
            &[d, ref rest @ ..] => {
                (d, str::from_utf8(rest).unwrap().parse::<i32>().unwrap())
            },
            _ => panic!(),
        };
    
        zeros += (cnt / DIAL_SIZE) as u32;  
        cnt = cnt % DIAL_SIZE;
        if dir == b'L' {
            if p > 0 && p <= cnt {
                zeros += 1;
            }
            cnt *= -1;
        } else if dir == b'R' {
            if p + cnt >= DIAL_SIZE {
                zeros += 1;
            }
        } else {
            panic!();
        }
        p = (p + cnt).rem_euclid(DIAL_SIZE);
    }
    println!("zero {} times", zeros);
}
fn main() {
    //part2(TEST);
    part2(&std::fs::read_to_string("input/d01.txt").unwrap());
}

const TEST: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;