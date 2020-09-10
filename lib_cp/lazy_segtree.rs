#![allow(unused)]

// ///////////////
// Lazy SegTree //
// ///////////////

use std::ops::Add;

#[derive(Copy, Clone)]
struct Delta {
    value: i64,
}
impl Default for Delta {
    fn default() -> Self {
        Self { value: 0 }
    }
}
impl Delta {
    fn new(value: i64) -> Self {
        Self { value }
    }
    fn update_delta(&mut self, delta: Delta) {
        self.value += delta.value;
    }
}

#[derive(Copy, Clone)]
struct Node {
    value: i64,
}
impl Node {
    fn new(value: i64) -> Self {
        Self { value }
    }
    fn new_from_child(lhs: Self, rhs: Self) -> Self {
        Self {
            value: std::cmp::max(lhs.value, rhs.value),
        }
    }
    fn apply_delta(&mut self, delta: Delta) {
        self.value += delta.value;
    }
}

impl Default for Node {
    fn default() -> Self {
        Self { value: 0 }
    }
}

struct LazySegTree {
    n: usize,
    tree_height: usize,
    tree: Vec<Node>,
    lazy_delta: Vec<Delta>,
    dirty: Vec<bool>,
}

impl LazySegTree {
    fn new(n: usize) -> Self {
        let nodes = vec![Node::default(); n];
        LazySegTree::new_from_base(n, nodes)
    }

    fn new_from_base(n: usize, nodes: Vec<Node>) -> Self {
        assert_eq!(nodes.len(), n);
        let mut tree_height = 0;
        while (1 << tree_height) < n {
            tree_height += 1;
        }
        let mut tree = vec![Node::default(); n * 2];
        for i in 0..n {
            tree[i] = nodes[i];
        }
        for i in (1..n).rev() {
            tree[i] = Node::new_from_child(tree[i * 2], tree[i * 2 + 1]);
        }

        Self {
            n,
            tree_height,
            tree,
            lazy_delta: vec![Delta::default(); n],
            dirty: vec![false; n],
        }
    }

    // 1. update Delta for Node
    // 2. Delta accumulate
    fn apply(&mut self, i: usize, delta: Delta) {
        self.tree[i].apply_delta(delta);

        if i < self.n {
            self.lazy_delta[i].update_delta(delta);
            self.dirty[i] = true;
        }
    }

    fn query(&mut self, l: usize, r: usize) -> Node {
        if l >= r {
            return Node::default();
        }
        let mut l = l + self.n;
        let mut r = r + self.n;
        self.propagate(l);
        self.propagate(r - 1);
        let mut lp = Node::default();
        let mut rp = Node::default();
        while l < r {
            if l & 1 == 1 {
                lp = Node::new_from_child(lp, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                rp = Node::new_from_child(self.tree[r], rp);
            }
            l /= 2;
            r /= 2;
        }
        Node::new_from_child(lp, rp)
    }

    fn update(&mut self, l: usize, r: usize, delta: Delta) {
        if l >= r {
            return;
        }

        let mut l = l + self.n;
        let mut r = r + self.n;

        self.propagate(l);
        self.propagate(r - 1);
        let origin_l = l.clone();
        let origin_r = r.clone();

        while l < r {
            if l & 1 == 1 {
                self.apply(l, delta);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;

                self.apply(r, delta);
            }
            l /= 2;
            r /= 2;
        }
        self.build(origin_l);
        self.build(origin_r - 1);
    }

    fn build(&mut self, node: usize) {
        let mut i = node / 2;
        while i > 0 {
            self.tree[i] = Node::new_from_child(self.tree[i * 2], self.tree[i * 2 + 1]);
            if self.dirty[i] {
                self.tree[i].apply_delta(self.lazy_delta[i]);
            }
            i /= 2;
        }
    }

    fn propagate(&mut self, node: usize) {
        for h in (1..=self.tree_height).rev() {
            let i = node >> h;
            if self.dirty[i] {
                self.apply(i * 2, self.lazy_delta[i]);
                self.apply(i * 2 + 1, self.lazy_delta[i]);
                self.lazy_delta[i] = Delta::default();
                self.dirty[i] = false;
            }
        }
    }
}

fn main() {
    // Atcoder: Intervals
    use proconio::marker::Usize1;
    use proconio::*;

    input! {
    n: usize,
    m: usize,
    a: [(Usize1, Usize1, i64); m]
    }
    let mut q = vec![vec![]; n];
    for (l, r, a) in a {
        q[r].push((l, a));
    }
    let mut tree = LazySegTree::new(n);

    for r in 0..n {
        let node = tree.query(0, r);
        tree.update(r, r + 1, Delta::new(node.value));

        for &(l, a) in &q[r] {
            tree.update(l, r + 1, Delta::new(a));
        }
    }
    let ans = tree.query(0, n);
    println!("{:?}", ans.value);
}
