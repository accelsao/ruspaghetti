#![allow(unused)]
use proconio::*;
use std::cell::RefCell;
use std::mem::swap;

// Disjoint Set Union
struct DSU {
    len: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(len: usize) -> Self {
        let mut parent = Vec::with_capacity(len);
        for i in 0..len {
            parent.push(i);
        }
        Self {
            len,
            parent,
            rank: vec![1; len],
        }
    }
    fn find_parent(&mut self, u: usize) -> usize {
        assert!(u < self.len);

        let x = self.parent[u];

        if x == u {
            x
        } else {
            let parent = self.find_parent(x);
            self.parent[u] = parent;
            parent
        }
    }
    fn merge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.len);
        assert!(v < self.len);
        let x = self.find_parent(u);
        let y = self.find_parent(v);

        if x == y {
            x
        } else {
            if self.rank[x] > self.rank[y] {
                self.rank[x] += self.rank[y];
                self.parent[y] = x;
                self.rank[y] = 0;
                x
            } else {
                self.rank[y] += self.rank[x];
                self.parent[x] = y;
                self.rank[x] = 0;
                y
            }
        }
    }
}

fn main() {
    input! {
    n: usize,
    }

    let mut dsu = DSU::new(n);
    dsu.merge(0, 1);
    dsu.merge(0, 2);
    for i in 0..n {
        println!("parent: {}, rank: {}", dsu.parent[i], dsu.rank[i]);
    }
}
