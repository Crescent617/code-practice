use my_collection::dsu::DSU;

#[allow(unused_macros)]
macro_rules! input {
    () => {{
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim().to_owned()
    }};
}

#[allow(unused_macros)]
macro_rules! timeit {
    ($block: tt) => {{
        use std::time;
        let end = time::Duration::from_secs(1);
        let t = time::Instant::now();
        let mut loop_num = 0;
        loop {
            loop_num += 1;
            $block;
            if t.elapsed() > end {
                break;
            }
        }
        let dt = t.elapsed();
        println!(
            "run {} loops, mean time usage: {:?}",
            loop_num,
            dt / loop_num
        );
    }};

    ($name: expr, $block: tt) => {{
        println!("Test Name: {}", $name);
        timeit!($block);
    }};
}

#[allow(unused_macros)]
macro_rules! input_num {
    () => {{
        input!().parse().unwrap()
    }};
}

#[allow(unused_macros)]
macro_rules! input_nums {
    () => {{
        input!()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }};
}

#[allow(unused_macros)]
macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut dict = std::collections::HashMap::new();
        $( dict.insert($key, $val); )*
        dict
    }};
}

struct Solution;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut dsu = DSU::new(n);

        for i in 0..n {
            for j in i+1..n {
                if Self::is_similar(&strs[i], &strs[j]) {
                    dsu.unite(i, j);
                }
            }
        }

        dsu.component_num() as i32

    }

    fn is_similar(s1: &str, s2: &str) -> bool {
        let r: i32 = s1
            .chars()
            .zip(s2.chars())
            .map(|(c1, c2)| (c1 == c2) as i32)
            .sum();
        r == 0 || r == 2
    }
}

struct GridSolver {
    data: Vec<i32>,
    dim: (usize, usize),
}

impl GridSolver {
    fn neighbors(&self, i: usize) -> Vec<usize> {
        let (m, n) = self.dim;
        let (r, c) = ((i / n) as i32, (i % n) as i32);

        let mut ans = vec![];
        for &(dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (nr, nc) = (r + dr, c + dc);
            if nr >= 0 && nr < (m as i32) && nc >= 0 && nc < (n as i32) {
                ans.push(self.idx(nr as usize, nc as usize));
            }
        }
        ans
    }

    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.dim.1 + c
    }
}

fn main() {
    println!("Hello World");
    let v = vec![1, 2];
}
