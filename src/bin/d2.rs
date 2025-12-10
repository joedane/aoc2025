use std::str::FromStr;


#[derive(Debug)]
struct Range(u64, u64);

impl Range {

    fn check_invalid(id: &u64) -> bool {
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
    fn sum_invalid_ids(&self) -> usize {
        (self.0..=self.1).filter(|n| Range::check_invalid(n)).map(|v| v as usize).sum()
    }
}
impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.find("-").unwrap();
        Ok(Range(s[0..i].parse().unwrap(), s[i+1..].parse().unwrap()))
    }
}

fn part1(input: &str) {

    let mut invalid_count: usize = 0;
    for r_str in input.split_terminator(",") {
        let range: Range = r_str.parse().unwrap();
        invalid_count += range.sum_invalid_ids();
    }
    println!("{}", invalid_count);
}
fn main() {
    
    //let input = TEST;
    let input = std::fs::read_to_string("input/d02.txt").unwrap();
    part1(&input.trim());
}

static TEST: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_range() {
        let r: Range = "998-1012".parse().unwrap();
        assert_eq!(r.sum_invalid_ids(), 1);
    }
}