use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::iter::Extend;

#[derive(Clone, Copy, Debug)]
struct Rng {
    low: u64,
    high: u64,
}

impl Rng {
    fn new(low: u64, high: u64) -> Self {
        Self { low, high }
    }

    fn check(&self, id: u64) -> bool {
        id >= self.low && id <= self.high
    }
}

impl From<(&str, &str)> for Rng {
    fn from(value: (&str, &str)) -> Self {
        Self {
            low: value.0.parse().unwrap(),
            high: value.1.parse().unwrap(),
        }
    }
}

impl PartialEq for Rng {
    fn eq(&self, other: &Self) -> bool {
        self.low == other.low && self.high == other.high
    }
}

impl PartialOrd for Rng {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.low.partial_cmp(&other.low) {
            Some(core::cmp::Ordering::Equal) => self.high.partial_cmp(&other.high),

            Some(ord) => Some(ord),
            None => None,
        }
    }
}

impl Eq for Rng {}

impl Ord for Rng {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
struct Ranges {
    ranges: Vec<Rng>,
}

impl Ranges {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn add(&mut self, rng: Rng) {
        self.ranges.push(rng);
    }
    fn merge_rng(&mut self, rng: Rng) {
        if self.ranges.len() == 0 {
            self.ranges.push(rng);
            return;
        }
        for i in 0..self.ranges.len() {
            if self.ranges[i].high < rng.low {
                continue;
            }
            // at this point, the rng to be inserted starts before the end of ranges[i].
            // we either need to add rng here, if no overlap, or adjust ranges[i] to
            // include rng.
            if rng.high < self.ranges[i].low {
                // insert rng
                self.ranges.insert(i, rng);
                return;
            } else {
                self.ranges[i].low = std::cmp::min(self.ranges[i].low, rng.low);
                self.ranges[i].high = std::cmp::max(self.ranges[i].high, rng.high);
                return;
            }
        }
        // if we get here we made it through all ranges, so we must insert
        // rng at the end of the list
        self.ranges.push(rng);
    }

    fn merge_all(self) -> Self {
        let bt: BTreeSet<Rng> = BTreeSet::from_iter(self.ranges.into_iter());
        let mut rng_iter = bt.into_iter();
        let mut new_rngs: Vec<Rng> = vec![rng_iter.next().unwrap()];
        while let Some(mut r) = rng_iter.next() {
            let this_rng = &mut new_rngs.last_mut().unwrap();
            if r.low > this_rng.high {
                new_rngs.push(r);
            } else {
                this_rng.high = std::cmp::max(this_rng.high, r.high);
            }
        }
        Self { ranges: new_rngs }
    }
}

fn part1(input: &str) {
    let mut ranges: Vec<Rng> = Default::default();
    let mut in_ranges = true;
    let mut ids: Vec<u64> = Default::default();

    for line in input.lines().map(str::trim) {
        if line.len() == 0 {
            in_ranges = false;
            continue;
        }
        if in_ranges {
            let i = line.find("-").unwrap();
            ranges.push((&line[0..i], &line[i + 1..]).into());
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    println!("read {} ranges and {} ids", ranges.len(), ids.len());
    println!(
        "{} fresh ingredients",
        ids.iter()
            .filter(|id| ranges.iter().any(|r| r.check(**id)))
            .count()
    );
}

fn part2(input: &str) {
    let mut ranges = Ranges::new();
    let mut in_ranges = true;

    for line in input.lines().map(str::trim) {
        if line.len() == 0 {
            in_ranges = false;
            continue;
        }
        if in_ranges {
            let i = line.find("-").unwrap();
            ranges.add((&line[0..i], &line[i + 1..]).into());
        }
    }
    let ranges = ranges.merge_all();
    let mut count: u64 = 0;
    for r in ranges.ranges {
        println!("{:?}", r);
        count += (r.high - r.low) + 1;
    }
    println!("total possible IDs are {}", count);
}
fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d05.txt").unwrap();
    part2(&input);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_ranges() {
        let mut rs = Ranges::new();
        rs.merge_rng(Rng::new(2, 6));
        rs.merge_rng(Rng::new(12, 20));
        assert_eq!(2, rs.ranges.len());
        rs.merge_rng(Rng::new(13, 15));
        assert_eq!(2, rs.ranges.len());

        rs.merge_rng(Rng::new(18, 25));
        assert_eq!(2, rs.ranges.len());
        rs.merge_rng(Rng::new(0, 1));
        assert_eq!(3, rs.ranges.len());

        rs.merge_rng(Rng::new(100, 110));
        assert_eq!(4, rs.ranges.len());
        for r in rs.ranges {
            println!("{:?}", r);
        }
    }
}

static TEST: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
