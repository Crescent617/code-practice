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
            .map(|x| x.trim().parse().unwrap())
            .collect()
    }};
}

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
macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut dict = std::collections::HashMap::new();
        $(dict.insert($key, $val); )*
        dict
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


use rand::prelude::*;
struct Point(i32, i32);

fn dist(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

fn main() {
    const N: usize = 1000;
    let mut rng = rand::thread_rng();
    let mut arr = vec![];
    for _ in 0..N {
        arr.push(Point(rng.gen_range(0..1000), rng.gen_range(0..1000)));
    }
    let mut d = [[0; N]; N];
    timeit!("calc points dist", {
        for i in 0..N {
            for j in 0..N {
                d[i][j] = dist(&arr[i], &arr[j]);
            }
        }
    });
}
