#![allow(unused)]
use proconio::*;
use std::collections::BinaryHeap;

const N: usize = 5e5 as usize + 10;

fn main() {
    input! {t:usize}
    for i in 0..t {
        input! {
        n:usize,
        mut a:[(usize, i64, i64); n],
        }

        let mut sum = 0;
        let mut pos = vec![];
        let mut neg = vec![];
        for (k, l, r) in a {
            sum += if l >= r {
                pos.push((k, r - l));
                r
            } else {
                if k < n {
                    neg.push((n - k, l - r))
                }
                l
            }
        }
        for mut a in vec![pos, neg] {
            a.sort();
            let mut heap = BinaryHeap::new();
            for (k, d) in a {
                heap.push(d);
                if heap.len() > k {
                    heap.pop();
                }
            }
            for v in heap {
                sum += -v;
            }
        }
        println!("{}", sum);
    }
}
