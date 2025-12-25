use std::collections::{BTreeSet, HashSet};

use utils::{AsciiByte, BasicGrid, Coord, Dir};

#[derive(Debug, PartialEq, Eq)]
struct OrdCoord(Coord);

impl PartialOrd for OrdCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.row.cmp(&other.0.row) {
            std::cmp::Ordering::Equal => Some(self.0.col.cmp(&other.0.col)),
            o @ _ => Some(o),
        }
    }
}

impl Ord for OrdCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(grid: &mut BasicGrid<AsciiByte>) {
    let start = grid.find(b'S'.into()).unwrap();
    let mut positions: BTreeSet<OrdCoord> = BTreeSet::new();
    let mut split_count: usize = 0;

    assert!(start.row == 0);
    positions.insert(OrdCoord(start));

    while let Some(OrdCoord(c)) = positions.pop_first() {
        if c.row < grid.height - 1 {
            if *grid.at(c.in_dir(Dir::Down).unwrap()) == AsciiByte(b'^') {
                split_count += 1;
                if c.col > 0 {
                    let nc = Coord::new(c.row + 1, c.col - 1);
                    grid[nc] = AsciiByte(b'|');
                    positions.insert(OrdCoord(nc));
                }
                if c.col < grid.width - 1 {
                    let nc = Coord::new(c.row + 1, c.col + 1);
                    grid[nc] = AsciiByte(b'|');
                    positions.insert(OrdCoord(nc));
                }
            } else {
                let nc = Coord::new(c.row + 1, c.col);
                grid[nc] = AsciiByte(b'|');
                positions.insert(OrdCoord(nc));
            }
        }
    }
    println!("{}", split_count);
}
fn part2(grid: &mut BasicGrid<AsciiByte>) {
    let start = grid.find(b'S'.into()).unwrap();
    assert!(start.row == 0);
    let mut this_row_counts: Vec<usize> = vec![0; grid.width];
    this_row_counts[start.col] = 1;
    let mut next_row_counts: Vec<usize> = vec![0; grid.width];

    for this_row in 0..grid.height - 1 {
        for c in 0..grid.width {
            if this_row_counts[c] > 0 {
                if grid.at(Coord::new(this_row, c)) == &AsciiByte(b'^') {
                    if c > 0 {
                        next_row_counts[c - 1] += this_row_counts[c];
                    }
                    if c < grid.width - 1 {
                        next_row_counts[c + 1] += this_row_counts[c];
                    }
                } else {
                    next_row_counts[c] += this_row_counts[c];
                }
            }
        }
        if this_row < grid.height - 2 {
            std::mem::swap(&mut this_row_counts, &mut next_row_counts);
            next_row_counts.iter_mut().for_each(|v| *v = 0);
        }
    }
    println!("{}", next_row_counts.into_iter().sum::<usize>());
}

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d07.txt").unwrap();
    let data: Vec<&str> = input.lines().map(str::trim).collect();
    let mut grid: BasicGrid<AsciiByte> = BasicGrid::new(&data);
    part2(&mut grid);
}

const TEST: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;
