struct Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut word_map: HashMap<_, _> = word_list
            .iter()
            .map(|x| (x.as_bytes().to_vec(), 0))
            .collect();

        word_map.insert(begin_word.as_bytes().to_vec(), 1);
        word_map.insert(end_word.as_bytes().to_vec(), 2);

        let mut f = VecDeque::from(vec![begin_word.as_bytes().to_vec()]);
        let mut b = VecDeque::from(vec![end_word.as_bytes().to_vec()]);

        let mut cnt = 1;
        let mut dir = 1;

        while !f.is_empty() {
            cnt += 1;
            if f.len() > b.len() {
                std::mem::swap(&mut f, &mut b);
                dir ^= 0b11;
            }

            for _ in 0..f.len() {
                let mut w = f.pop_front().unwrap();
                for i in 0..w.len() {
                    let old = w[i];

                    for c in b'a'..=b'z' {
                        w[i] = c;
                        if let Some(p) = word_map.get_mut(&w) {
                            if *p & dir != 0 {
                                continue;
                            }
                            *p |= dir;
                            if *p == 0b11 {
                                return cnt;
                            }
                            f.push_back(w.clone());
                        }
                    }

                    w[i] = old;
                }
            }
            // println!("{:?}", seen);
        }
        0
    }
}


fn main() {}
