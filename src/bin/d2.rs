use std::str::FromStr;

#[derive(Debug)]
struct Range(u64, u64);

impl Range {
    fn check_invalid_part2(id: u64) -> bool {
        let total_digits = id.ilog10() + 1;
        if total_digits == 1 {
            return false;
        }
        let mut part_digits = total_digits / 2;
        'digits: while part_digits > 0 {
            let num_parts = total_digits / part_digits;
            if num_parts * part_digits != total_digits {
                part_digits -= 1;
                continue 'digits;
            }
            let d = 10u64.pow(part_digits);
            let pat = id % d;
            let mut check = id;
            for _ in 0..num_parts {
                if check % d != pat {
                    part_digits -= 1;
                    continue 'digits;
                }
                check /= d;
            }
            println!("{} is invalid", id);
            return true;
        }
        return false;
    }

    fn check_invalid_part1(id: &u64) -> bool {
        let l = id.ilog10();
        if l % 2 == 1 {
            let p = 10u64.pow(l.div_ceil(2));
            if id / p == id % p {
                return true;
            } else {
                return false;
            }
        }
        false
    }
    fn sum_invalid_ids_part1(&self) -> usize {
        (self.0..=self.1)
            .filter(|n| Range::check_invalid_part1(n))
            .map(|v| v as usize)
            .sum()
    }
    fn sum_invalid_ids_part2(&self) -> usize {
        (self.0..=self.1)
            .filter(|n| Range::check_invalid_part2(*n))
            .map(|v| v as usize)
            .sum()
    }
}
impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.find("-").unwrap();
        Ok(Range(s[0..i].parse().unwrap(), s[i + 1..].parse().unwrap()))
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut invalid_count: usize = 0;
    for r_str in input.split_terminator(",") {
        let range: Range = r_str.parse().unwrap();
        invalid_count += range.sum_invalid_ids_part1();
    }
    println!("{}", invalid_count);
}

fn part2(input: &str) {
    let mut invalid_count: usize = 0;
    for r_str in input.split_terminator(",") {
        let range: Range = r_str.parse().unwrap();
        invalid_count += range.sum_invalid_ids_part2();
    }
    println!("{}", invalid_count);
}
fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d02.txt").unwrap();
    part2(&input.trim());
}

#[allow(unused)]
static TEST: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_range() {
        //let r: Range = "998-1012".parse().unwrap();
        //assert_eq!(r.sum_invalid_ids(), 1);

        //        assert!(Range::check_invalid_part2(22));
        //        assert!(Range::check_invalid_part2(1010));
        //        assert!(!Range::check_invalid_part2(1011));
        //        assert!(Range::check_invalid_part2(123123));
        //        assert!(Range::check_invalid_part2(1111111));
        assert!(!Range::check_invalid_part2(5552525252));
    }
}
