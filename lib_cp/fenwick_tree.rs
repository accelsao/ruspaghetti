#![allow(unused)]

// ///////////////
// Fenwick Tree //
// ///////////////

struct FenwickTree {
    n: isize,
    value: Vec<usize>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n: n as isize,
            value: vec![0; n],
        }
    }
    fn add(&mut self, pos: usize, value: usize) {
        let mut x = pos as isize;
        while x <= self.n {
            self.value[x as usize - 1] += value;
            x += x & -x;
        }
    }

    fn get(&self, pos: usize) -> usize {
        let mut sum = 0;
        let mut x = pos as isize;
        while x > 0 {
            sum += self.value[x as usize - 1];
            x -= x & -x;
        }
        sum
    }

    // 1-indexed, [l,r]
    fn sum(&self, l: usize, r: usize) -> usize {
        self.get(r) - self.get(l - 1)
    }
}

fn main() {
    use proconio::*;
    input! {
    n: usize,
    q: usize,
    a: [usize;n],
    b: [(usize,usize,usize);q],
    }

    let mut fwt = FenwickTree::new(n);
    for i in 0..n {
        fwt.add(i + 1, a[i]);
    }
    for (t, l, r) in b {
        let l = l + 1;
        match t {
            0 => fwt.add(l, r),
            1 => {
                println!("{}", fwt.sum(l, r));
            }
            _ => {}
        }
    }
}
