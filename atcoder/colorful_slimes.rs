#![allow(unused)]

use proconio::*;
use std::cmp;

fn main() {
    input! {
        n: usize,
        x: u64,
        mut a: [u64; n],
    }

    let mut ans: u64 = a.iter().sum();
    let mut tran_cost: u64 = 0;
    for i in 0..n {
        tran_cost += x;
        let mut b = a.clone();
        for j in 0..n {
            b[j] = cmp::min(a[(j + n - 1) % n], b[j])
        }
        a = b;

        ans = cmp::min(ans, a.iter().sum::<u64>() + tran_cost);
    }
    println!("{}", ans);
}
