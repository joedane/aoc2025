use env_logger::Env;

mod part1 {
    use std::{
        collections::{BTreeSet, HashMap, HashSet},
        fmt::Display,
        sync::{LazyLock, Mutex},
    };

    use itertools::{Itertools, merge, merge_join_by};
    use log::{debug, warn};
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
                u64::isqrt(
                    Dist::FACTOR * dx * dx + Dist::FACTOR * dy * dy + Dist::FACTOR * dz * dz,
                )
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

        fn extend(&mut self, other: &mut Self) {
            self.points.extend(other.points.iter());
            other.points.clear();
        }
    }
    pub(crate) fn run(input: &str, count: usize) {
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
                pairs.push((objs[0], objs[1]));
            }
        }
        pairs.sort_by(|a, b| a.0.dist(&a.1).cmp(&b.0.dist(&b.1)));
        let mut union_map: HashMap<C3, Circuit> =
            objs.iter().map(|p| (*p, Circuit::new(*p))).collect();

        for (i, p) in pairs.iter().enumerate() {
            if std::ptr::eq(union_map.get(&p.0).unwrap(), union_map.get(&p.1).unwrap()) {
                continue;
            }
            union_map
                .get_mut(&p.0)
                .unwrap()
                .extend(union_map.get_mut(&p.1).unwrap());
            }
            union_map.insert(p.1, v)
        'pairs: for sd in dists.iter() {
            if connection_count == count {
                break;
            }
            let ia = u16::min(sd.obja, sd.objb);
            let ib = u16::max(sd.obja, sd.objb);
            let (s1, s2) = objs.split_at_mut(ia as usize + 1);
            let obja = &mut s1[ia as usize];
            let objb = &mut s2[(ib - ia - 1) as usize];
            debug!(
                "Distance between [{}] and [{}] is {}",
                obja.display_full(),
                objb.display_full(),
                sd.dist
            );
            if obja.circuit == 0 && objb.circuit == 0 {
                debug!(
                    "adding {} and {} to new circuit |{}|",
                    obja.display_id(),
                    objb.display_id(),
                    next_circuit_id
                );
                obja.circuit = next_circuit_id;
                objb.circuit = next_circuit_id;
                next_circuit_id += 1;
                connection_count += 1;
            } else if obja.circuit == 0 || objb.circuit == 0 {
                if obja.circuit == 0 {
                    debug!("adding {} to circuit |{}|", obja.display_id(), objb.circuit);
                    obja.circuit = objb.circuit;
                } else {
                    debug!("adding {} to circuit |{}|", objb.display_id(), obja.circuit);
                    objb.circuit = obja.circuit;
                }
                connection_count += 1;
            } else {
                connection_count += 1;
                debug!(
                    "objs {} and {} have already been assigned (to circuit |{}| and circuit |{}|, respectively)",
                    obja.display_id(),
                    objb.display_id(),
                    obja.circuit,
                    objb.circuit
                );
                if obja.circuit != objb.circuit {
                    let obja_circuit = obja.circuit;
                    let objb_circuit = objb.circuit;
                    let merge_key = u16::min(obja.circuit, objb.circuit);
                    for o in &mut objs {
                        if o.circuit == obja_circuit || o.circuit == objb_circuit {
                            debug!("moving {} to circuit |{}|", o.display_id(), merge_key);
                            o.circuit = merge_key;
                        }
                    }
                    debug!(
                        "adding new merge set for |{}| and |{}|",
                        obja_circuit, objb_circuit
                    );
                }
            }
        }

        let mut counts: HashMap<u16, usize> = Default::default();
        for obj in objs {
            *counts.entry(obj.circuit).or_insert(0) += 1;
        }

        for c in counts.iter().sorted_by(|a, b| b.1.cmp(a.1)) {
            println!("circuit {}: {:?}", c.0, c.1);
        }
        println!(
            "{}",
            counts
                .into_iter()
                .filter(|p| p.0 > 0)
                .sorted_by(|a, b| b.1.cmp(&a.1))
                .take(3)
                .map(|e| e.1)
                .reduce(|acc, e| acc * e)
                .unwrap()
        );
    }
}

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    //let input = TEST;
    let input = std::fs::read_to_string("input/d08.txt").unwrap();
    part1::run(&input, 1000);
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
