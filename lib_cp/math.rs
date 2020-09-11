#![allow(unused)]

// ///////
// Math //
// ///////

// for i in 0 to n-1
//   sum( floor( (a * i + b) / m ) )
fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }
    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == 0 {
        return ans;
    }
    ans += (n - (x_max + a - 1) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - (x_max % a)) % a);
    return ans;
}

fn main() {
    use proconio::*;
    input! {
    t: usize
    }
    for i in 0..t {
        input! {
        n:i64,
        m:i64,
        a:i64,
        b:i64,
        }
        println!("{}", floor_sum(n, m, a, b));
    }
}
