use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, num::ParseIntError, ops::Deref, str::FromStr};
use utils::{BasicGrid, Coord, Dir};

#[derive(Clone, Copy, Debug)]
struct P(Coord);

impl P {
    fn new(row: usize, col: usize) -> Self {
        Self(Coord::new(row, col))
    }
}
impl FromStr for P {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.find(',').ok_or(format!("failed to parse point: {}", s))?;
        Ok(P(Coord::new(
            s[i + 1..]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
            s[..i].parse().map_err(|e: ParseIntError| e.to_string())?,
        )))
    }
}

impl Deref for P {
    type Target = Coord;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn rect_size((p1, p2): (P, P)) -> u64 {
    (p1.0.col.abs_diff(p2.0.col) as u64 + 1) * (p1.0.row.abs_diff(p2.0.row) as u64 + 1)
}

fn part1(points: Vec<P>) {
    let mut biggest_pair = (P(Coord::new(0, 0)), P(Coord::new(0, 0)));
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

impl Default for State {
    fn default() -> Self {
        Self::Inside
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Outside => "O",
                State::Boundary => "B",
                State::Inside => "I",
            }
        )
    }
}
fn draw_line(grid: &mut BasicGrid<State>, a: Coord, b: Coord) {
    if a.col == b.col {
        let dir: i32 = if a.row < b.row { 1 } else { -1 };
        for j in 0..a.row.abs_diff(b.row) {
            let j = j as i32 * dir;
            grid[Coord::new(((a.row as i32) + j) as usize, a.col as usize)] = State::Boundary;
        }
    } else if a.row == b.row {
        let dir: i32 = if a.col < b.col { 1 } else { -1 };
        for j in 0..a.col.abs_diff(b.col) {
            let j = j as i32 * dir;
            grid[Coord::new(a.row, ((a.col as i32) + j) as usize)] = State::Boundary;
        }
    } else {
        panic!()
    }
}

fn fill_from(grid: &mut BasicGrid<State>, seen: &mut HashSet<Coord>, seed: Coord) {
    seen.insert(seed);
    let mut stack: Vec<Coord> = vec![seed];
    while let Some(c) = stack.pop() {
        grid[c] = State::Outside;
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .filter_map(|d| grid.next_pos(c, d))
            .for_each(|c| {
                if !seen.contains(&c) && matches!(grid[c], State::Inside) {
                    stack.push(c)
                }
            });
    }
}
fn fill(grid: &mut BasicGrid<State>) {
    let mut seen: HashSet<Coord> = Default::default();
    for row in 0..grid.height {
        let c = Coord::new(row, 0);
        if !seen.contains(&c) {
            match grid[c] {
                State::Boundary => {}
                _ => {
                    fill_from(grid, &mut seen, c);
                }
            }
        }
        let c = Coord::new(row, grid.width - 1);
        if !seen.contains(&c) {
            match grid[c] {
                State::Boundary => {}
                _ => {
                    fill_from(grid, &mut seen, c);
                }
            }
        }
    }

    for col in 0..grid.width {
        let c = Coord::new(0, col);
        if !seen.contains(&c) {
            match grid[c] {
                State::Boundary => {}
                _ => {
                    fill_from(grid, &mut seen, c);
                }
            }
        }
        let c = Coord::new(grid.height - 1, col);
        if !seen.contains(&c) {
            match grid[c] {
                State::Boundary => {}
                _ => {
                    fill_from(grid, &mut seen, c);
                }
            }
        }
    }
}
fn area(p1: P, p2: P) -> usize {
    p1.col.abs_diff(p2.col) * p1.row.abs_diff(p2.row)
}

fn valid(grid: &BasicGrid<State>, p1: P, p2: P) -> bool {
    let ul = P::new(usize::min(p1.row, p2.row), usize::min(p1.col, p2.col));
    let lr = P::new(usize::max(p1.row, p2.row), usize::max(p1.col, p2.col));
    for r in ul.row..=lr.row {
        for c in ul.col..=lr.col {
            if matches!(grid.at(Coord::new(r, c)), State::Outside) {
                return false;
            }
        }
    }
    return true;
}

fn part2(points: &[P]) {
    let (mut max_x, mut max_y, mut min_x, mut min_y) = (0usize, 0usize, usize::MAX, usize::MAX);
    for p in points {
        max_x = usize::max(max_x, p.col as usize);
        max_y = usize::max(max_y, p.row as usize);
    }
    let mut grid: BasicGrid<State> = BasicGrid::new_default(max_x + 1, max_y + 1);
    let start = points[0];
    for i in 0..points.len() - 1 {
        draw_line(&mut grid, *points[i], *points[i + 1]);
    }
    draw_line(&mut grid, *points[points.len() - 1], *start);
    fill(&mut grid);

    let mut biggest: usize = 0;

    for i1 in 0..points.len() - 1 {
        for i2 in (i1 + 1)..points.len() {
            let a = area(points[i1], points[i2]);
            if a > biggest && valid(&grid, points[i1], points[i2]) {
                biggest = a
            }
        }
    }
    println!("{}", biggest);


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
        assert_eq!(p.col, 23);
        assert_eq!(p.row, 100);
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
