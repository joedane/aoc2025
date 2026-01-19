use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};
use utils::{BasicGrid, Coord};

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

fn part1(points: Vec<P>) {
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


#[derive(Clone, Copy, Debug)]
enum State {
    Outside,
    Boundary,
    Inside,
}

fn draw_line(grid: &mut BasicGrid<State>, a: P, b: P) {
    if a.x == b.x {
        let dir: i32 = if a.y < b.y { 1 } else { -1 };
        for j in 0..a.y.abs_diff(b.y) {
            let j = j as i32 * dir;
            grid[Coord::new(a.x as usize, ((a.y as i32) + j) as usize)] = State::Boundary;
        }
    } else if a.y == b.y {
        let dir: i32 = if a.x < b.x { 1 } else { -1 };
        for j in 0..a.x.abs_diff(b.x) {
            let j = j as i32 * dir;
            grid[Coord::new(((a.x as i32) + j) as usize, a.y as usize)] = State::Boundary;
        }
    } else {
        panic!()
    }
}
fn part2(points: &[P]) {
    let (mut max_x, mut max_y, mut min_x, mut min_y) = (0usize, 0usize, usize::MAX, usize::MAX);
    for p in points {
        max_x = usize::max(max_x, p.x as usize);
        max_y = usize::max(max_y, p.y as usize);
        min_x = usize::min(min_x, p.x as usize);
        min_y = usize::min(min_y, p.y as usize);
    }
    let mut grid: BasicGrid<State> =
        BasicGrid::new_from(max_x - min_x + 1, max_y - min_y + 1, |x, y| State::Outside);
    let start = points[0];
    for i in 0..points.len() - 1 {
        draw_line(&mut grid, points[i], points[i + 1]);
    }
    draw_line(&mut grid, points[points.len() - 1], start);
    for y in 0..grid.height {
        let mut was_inside = false;
        for x in 0..grid.width {
            match grid[Coord::new(y, x)] {
                State::Outside => {},
                State::Boundary => ,
                State::Inside => todo!(),
            }
        }
    }
}

fn main() {
    let input = TEST;
    //let input = std::fs::read_to_string("input/d09.txt").unwrap();
    let points: Vec<P> = input
        .lines()
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<P>, _>>()
        .unwrap();
    part2(&points);
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
