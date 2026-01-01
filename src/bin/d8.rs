use env_logger::Env;

mod part1 {
    use std::collections::{BTreeSet, HashMap};

    use itertools::Itertools;
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
    struct SortData {
        dist: Dist,
        obja: u16,
        objb: u16,
    }

    impl SortData {
        fn new(dist: Dist, obja: u16, objb: u16) -> Self {
            Self { dist, obja, objb }
        }
    }

    impl Ord for SortData {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.dist.cmp(&other.dist)
        }
    }

    impl PartialOrd for SortData {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for SortData {
        fn eq(&self, other: &Self) -> bool {
            self.dist == other.dist && self.obja == other.obja && self.objb == other.objb
        }
    }

    impl Eq for SortData {}

    #[derive(Clone, Copy, Debug)]
    struct C3 {
        x: u32,
        y: u32,
        z: u32,
        circuit: u16,
    }

    impl C3 {
        fn new(x: u32, y: u32, z: u32) -> Self {
            Self {
                x,
                y,
                z,
                circuit: 0,
            }
        }

        fn dist(&self, other: &Self) -> Dist {
            Dist::dist(self, other)
        }
    }

    impl std::fmt::Display for C3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.circuit)
        }
    }

    pub(crate) fn run(input: &str, count: usize) {
        let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
        let mut objs: Vec<C3> = Vec::with_capacity(1000);
        let mut dists: BTreeSet<SortData> = BTreeSet::new();
        for (_, [x, y, z]) in re.captures_iter(&input).map(|c| c.extract()) {
            objs.push(C3::new(
                x.parse().unwrap(),
                y.parse().unwrap(),
                z.parse().unwrap(),
            ));
        }
        for i1 in 0..objs.len() {
            for i2 in 0..objs.len() {
                if i1 == i2 {
                    continue;
                }
                dists.insert(SortData::new(
                    objs[i1].dist(&objs[i2]),
                    i1.try_into().unwrap(),
                    i2.try_into().unwrap(),
                ));
            }
        }
        let mut next_circuit_id: u16 = 1;
        let mut connection_count = 0;

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
                "Distance between [{:6}, {:6}, {:6}] and [{:6}, {:6}, {:6}] is {}",
                obja.x, obja.y, obja.z, objb.x, objb.y, objb.z, sd.dist
            );
            if obja.circuit == 0 && objb.circuit == 0 {
                debug!(
                    "adding {} and {} to new circuit |{}|",
                    obja, objb, next_circuit_id
                );
                obja.circuit = next_circuit_id;
                objb.circuit = next_circuit_id;
                next_circuit_id += 1;
                connection_count += 1;
            } else if obja.circuit == 0 || objb.circuit == 0 {
                if obja.circuit == 0 {
                    debug!("adding {} to circuit |{}|", obja, objb.circuit);
                    obja.circuit = objb.circuit;
                } else {
                    debug!("adding {} to circuit |{}|", objb, obja.circuit);
                    objb.circuit = obja.circuit;
                }
                connection_count += 1;
            } else {
                connection_count += 1;
                debug!(
                    "objs [{:6}, {:6}, {:6}] and [{:6}, {:6}, {:6}] have already been assigned (to circuit |{}| and circuit |{}|, respectively)",
                    obja.x, obja.y, obja.z, objb.x, objb.y, objb.z, obja.circuit, objb.circuit
                );
                if obja.circuit != objb.circuit {
                    for c in &mut circuits_to_merge {
                        let merge_key = u16::min(obja.circuit, objb.circuit);

                        if c.contains(&obja.circuit) || c.contains(&objb.circuit) {
                            debug!(
                                "merging |{}| and |{}| into merge set {:?}",
                                obja.circuit, objb.circuit, c
                            );
                            c.insert(obja.circuit);
                            c.insert(objb.circuit);
                            continue 'pairs;
                        }
                    }
                    debug!(
                        "adding new merge set for |{}| and |{}|",
                        obja.circuit, objb.circuit
                    );
                    circuits_to_merge.push([obja.circuit, objb.circuit].into());
                }
            }
        }

        warn!("merging: {:?}", circuits_to_merge);
        for cs in &mut circuits_to_merge {
            assert!(cs.len() > 1);
            let to_circuit = cs.pop_first().unwrap();
            for obj in &mut objs {
                if cs.contains(&obj.circuit) {
                    obj.circuit = to_circuit;
                }
            }
        }
        let mut counts: HashMap<u16, usize> = Default::default();
        for obj in objs {
            *counts.entry(obj.circuit).or_insert(0) += 1;
        }
        /*         for c in &counts {
                   println!("circuit {} has {} elements", c.0, c.1);
               }
        */

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
