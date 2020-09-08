#![allow(unused)]
use proconio::*;

fn main() {
    input! {
    n: usize,
    f: [f64; n],
    }

    // log2(10^4 * 10^9)
    const P_MAX: usize = 45;

    let mut dp = vec![vec![0u64; P_MAX]; P_MAX];
    let mut ans = 0;
    for i in 0..n {
        let mut num = (f[i] * 1_000_000_000f64).round() as u64;
        let mut two = 0;
        let mut five = 0;
        while num % 2 == 0 {
            num /= 2;
            two += 1;
        }
        while num % 5 == 0 {
            num /= 5;
            five += 1;
        }
        // println!("{:?}", (two, five));
        for i in 0..P_MAX {
            for j in 0..P_MAX {
                if i + two >= 18 && j + five >= 18 {
                    ans += dp[i][j];
                }
            }
        }
        dp[two][five] += 1;
    }
    println!("{}", ans);
}
