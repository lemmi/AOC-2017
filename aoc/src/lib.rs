pub mod input {
    use std::io;
    use std::io::Stdin;
    use std::io::BufRead;
    use std::io::Lines;

    pub struct LineFilter<'a> {
        lines: Lines<io::StdinLock<'a>>,
    }

    pub fn lines<'a>(stdin: &'a Stdin) -> LineFilter<'a> {
        LineFilter{lines: stdin.lock().lines()}
    }

    impl<'a> Iterator for LineFilter<'a> {
        type Item = String;
        fn next(&mut self) -> Option<String> {
            self.lines.by_ref()
                .map(|l| l.unwrap())
                .skip_while(
                    |l| l.trim()
                    .is_empty())
                .next()
        }
    }
}

pub mod hash {
    use std::fmt;
    use std::slice::Iter;
    use std::str::FromStr;

    fn reverse_wrapped(s: &mut[u8], pos: usize, length: usize) {
        let mut l = pos;
        let mut r = pos + length - 1;

        while l < r {
            s.swap(l%256, r%256);

            l += 1;
            r -= 1;
        }
    }

    fn knot_hash(input: &[u8]) -> [u8; 256] {
        let mut hash = [0u8; 256];
        for (i, h) in hash.iter_mut().enumerate() {
            *h = i as u8;
        }

        let mut pos = 0;
        let mut skip = 0;
        let postfix = [17, 31, 73, 47, 23];

        for _ in 0..64 {
            for l in input.iter().chain(&postfix) {
                let l = *l as usize;
                reverse_wrapped(&mut hash, pos, l);

                pos = (pos +l + skip) % 256;
                skip = (skip + 1) % 256;
            }
        }

        hash
    }

    pub struct Knot([u8; 16]);
    impl fmt::Display for Knot {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for c in self.0.iter() {
                write!(f, "{:02x}", c)?;
            }
            Ok(())
        }
    }

    impl Knot {
        pub fn iter(&self) -> Iter<u8> {
            self.0.iter()
        }
    }

    impl FromStr for Knot {
        type Err = ();
        fn from_str(input: &str) -> Result<Self,Self::Err> {
            let sparse = knot_hash(input.as_bytes());
            let mut ret = [0u8; 16];
            for (i, chunk) in sparse.chunks(16).enumerate() {
                ret[i] = chunk.iter().fold(0, |sum, x| sum ^ x);
            }
            Ok(Knot(ret))
        }
    }
}

pub mod graph {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use std::hash::Hash;
    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Implicit<T>
    where T: Eq + Hash + Copy + Debug {
        pub edges: HashMap<T, HashSet<T>>,
    }

    impl<T> Implicit<T> 
    where T: Eq + Hash + Copy + Debug {
        pub fn new() -> Implicit<T> {
            Implicit{edges: HashMap::default()}
        }
        pub fn len(&self) -> usize {
            self.edges.len()
        }
        pub fn insert(&mut self, k: T, v: Option<T>) {
            let links = self.edges.entry(k).or_insert(HashSet::new());
            if let Some(v) = v {
                links.insert(v);
            }
        }
        pub fn get_any(&self) -> Option<T> {
            self.edges.keys().next().cloned()
        }
        pub fn group(&self, seed: T) -> HashSet<T> {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(seed);

            while let Some(node) = queue.pop_front() {
                if !visited.insert(node) {
                    continue;
                }
                if let Some(candidates) = self.edges.get(&node) {
                    queue.extend(candidates);
                }
            }

            visited
        }
        pub fn num_groups(&self) -> u32 {
            let mut t = Implicit::<T>{edges: self.edges.clone()};
            let mut n = 0u32;
            while let Some(seed) = t.get_any() {
                let g = t.group(seed);
                for node in g {
                    t.edges.remove(&node);
                }
                n += 1;
            }
            n
        }

        pub fn get(&self, k: T) -> Option<HashSet<T>> {
            self.edges.get(&k).cloned()
        }
    }
}
