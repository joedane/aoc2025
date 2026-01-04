use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug)]
struct P {
    x: u32,
    y: u32,
}

impl P {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl FromStr for P {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.find(',').ok_or(format!("failed to parse point: {}", s))?;
        Ok(P::new(
            s[..i].parse().map_err(|e: ParseIntError| e.to_string())?,
            s[i + 1..]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        ))
    }
}

fn rect_size((p1, p2): (P, P)) -> u64 {
    (p1.x.abs_diff(p2.x) as u64 + 1) * (p1.y.abs_diff(p2.y) as u64 + 1)
}

fn main() {
    //    let input = TEST;
    let input = std::fs::read_to_string("input/d09.txt").unwrap();
    let points: Vec<P> = input
        .lines()
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<P>, _>>()
        .unwrap();
    let mut biggest_pair = (P::new(0, 0), P::new(0, 0));
    let mut biggest_size = rect_size(biggest_pair);
    for p in points.into_iter().tuple_combinations() {
        let size = rect_size(p);
        if size > biggest_size {
            biggest_size = size;
            biggest_pair = p;
        }
    }
    println!("{}", biggest_size);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let p: P = "23,100".parse().unwrap();
        assert_eq!(p.x, 23);
        assert_eq!(p.y, 100);
    }

    #[test]
    fn test_rect_size() {
        assert_eq!(rect_size((P::new(2, 5), P::new(9, 7))), 24);
    }
}

static TEST: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
