use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{LazyLock, Mutex},
};

use env_logger::Env;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Dist(u32);

impl Dist {
    const FACTOR: u64 = 100;
    fn dist(o1: &C3, o2: &C3) -> Self {
        let (dx, dy, dz) = (
            (o1.x as u64).abs_diff(o2.x as u64),
            (o1.y as u64).abs_diff(o2.y as u64),
            (o1.z as u64).abs_diff(o2.z as u64),
        );
        Self(
            u64::isqrt(Dist::FACTOR * dx * dx + Dist::FACTOR * dy * dy + Dist::FACTOR * dz * dz)
                .try_into()
                .unwrap(),
        )
    }
}

impl std::fmt::Display for Dist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0 as f32 / (Dist::FACTOR as f32).sqrt())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct C3 {
    x: u32,
    y: u32,
    z: u32,
}

impl C3 {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: &Self) -> Dist {
        Dist::dist(self, other)
    }
}

static NEXT_CIRCUIT_ID: LazyLock<Mutex<u16>> = LazyLock::new(|| Mutex::new(0));
#[derive(Debug)]
struct Circuit {
    id: u16,
    points: HashSet<C3>,
}

impl Circuit {
    fn new(point: C3) -> Self {
        let mut lock = NEXT_CIRCUIT_ID.lock().unwrap();
        let this_id = *lock;
        *lock = *lock + 1;
        Self {
            id: this_id,
            points: [point].into(),
        }
    }

    fn join(c1: &Circuit, c2: &Circuit) -> Self {
        let mut lock = NEXT_CIRCUIT_ID.lock().unwrap();
        let this_id = *lock;
        *lock = *lock + 1;
        let mut points = c1.points.clone();
        points.extend(c2.points.iter());
        Self {
            id: this_id,
            points,
        }
    }
}

fn parse(input: &str) -> (Vec<(C3, C3)>, HashMap<C3, Rc<Circuit>>) {
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let mut objs: Vec<C3> = Vec::with_capacity(1000);
    for (_, [x, y, z]) in re.captures_iter(&input).map(|c| c.extract()) {
        objs.push(C3::new(
            x.parse().unwrap(),
            y.parse().unwrap(),
            z.parse().unwrap(),
        ));
    }
    let mut pairs: Vec<(C3, C3)> = Vec::with_capacity(objs.len() * objs.len() / 2);
    //        let mut circuits: Vec<Circuit> = objs.iter().map(|o| Circuit::new(*o)).collect();
    for i1 in 0..objs.len() {
        for i2 in i1 + 1..objs.len() {
            pairs.push((objs[i1], objs[i2]));
        }
    }
    pairs.sort_by(|a, b| a.0.dist(&a.1).cmp(&b.0.dist(&b.1)));
    let mut union_map: HashMap<C3, Rc<Circuit>> = objs
        .iter()
        .map(|p| (*p, Rc::new(Circuit::new(*p))))
        .collect();

    (pairs, union_map)
}

mod part1 {
    use itertools::Itertools;
    use std::{collections::HashMap, rc::Rc};

    pub(crate) fn run(input: &str, count: usize) {
        let (pairs, mut union_map) = super::parse(input);
        for (i, p) in pairs.iter().enumerate() {
            if i == count {
                let counts: HashMap<u16, usize> = union_map
                    .into_iter()
                    .map(|(p, u)| (u.id, u.points.len()))
                    .collect();
                println!(
                    "{}",
                    counts
                        .into_iter()
                        .sorted_by(|a, b| b.1.cmp(&a.1))
                        .take(3)
                        .fold(1, |acc, e| acc * e.1)
                );
                return;
            }
            if union_map.get(&p.0).unwrap().id == union_map.get(&p.1).unwrap().id {
                continue;
            }
            let (c1, c2) = (union_map.get(&p.0).unwrap(), union_map.get(&p.1).unwrap());
            let new_circuit = Rc::new(super::Circuit::join(c1, c2));
            for p in &new_circuit.points {
                union_map.insert(*p, new_circuit.clone());
            }
        }
    }
}

mod part2 {
    use itertools::Itertools;
    use std::{collections::HashMap, rc::Rc};
    pub(crate) fn run(input: &str) {
        let (pairs, mut union_map) = super::parse(input);
        for (i, p) in pairs.iter().enumerate() {
            if union_map.get(&p.0).unwrap().id == union_map.get(&p.1).unwrap().id {
                continue;
            }
            let (c1, c2) = (union_map.get(&p.0).unwrap(), union_map.get(&p.1).unwrap());
            let new_circuit = Rc::new(super::Circuit::join(c1, c2));
            if new_circuit.points.len() == union_map.len() {
                println!("{}", p.0.x as u64 * p.1.x as u64);
                return;
            }
            for p in &new_circuit.points {
                union_map.insert(*p, new_circuit.clone());
            }
        }
    }
}

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    //let input = TEST;
    let input = std::fs::read_to_string("input/d08.txt").unwrap();
    part2::run(&input);
}

#[allow(unused)]
const TEST: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
