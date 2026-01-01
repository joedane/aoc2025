use utils::{BasicGrid, Coord};

enum State {
    EMPTY,
    ROLL,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        if value == b'.' {
            Self::EMPTY
        } else if value == b'@' {
            Self::ROLL
        } else {
            panic!("{}", value)
        }
    }
}

fn can_lift(grid: &BasicGrid<State>, c: Coord) -> bool {
    matches!(grid.at(c), State::ROLL)
        && grid
            .neighbors(c)
            .filter(|p| matches!(grid.at(*p), State::ROLL))
            .count()
            < 4
}

#[allow(dead_code)]
fn part1(input: &[&str]) {
    let grid: BasicGrid<State> = BasicGrid::new(&input);
    println!(
        "{}",
        grid.row_major_iter()
            .filter(|c| can_lift(&grid, *c))
            .count()
    );
}

fn part2(input: &[&str]) {
    let mut grid: BasicGrid<State> = BasicGrid::new(&input);
    let mut to_remove: Vec<Coord> = grid
        .row_major_iter()
        .filter_map(|c| can_lift(&grid, c).then_some(c))
        .collect();
    let mut total_removed = 0;
    while to_remove.len() > 0 {
        for c in &to_remove {
            grid[*c] = State::EMPTY;
        }
        println!("removed {}", to_remove.len());
        total_removed += to_remove.len();
        to_remove.clear();
        to_remove.extend(
            grid.row_major_iter()
                .filter_map(|c| can_lift(&grid, c).then_some(c)),
        );
    }
    println!("{} total removed", total_removed);
}
fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d04.txt").unwrap();
    let input: Vec<&str> = input.lines().map(str::trim).collect();
    part2(&input);
}

#[allow(dead_code)]
static TEST: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
