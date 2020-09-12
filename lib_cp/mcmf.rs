#![allow(unused)]
// ///////
// MCMF //
// ///////

use std::collections::VecDeque;

type Cost = i64;
type Flow = i64;

const MAX_COST: Cost = 1e12 as Cost;
const MAX_CAP: Flow = 1e12 as Flow;

#[derive(Debug)]
struct Edge {
    to: usize,
    rev: usize,
    cap: Flow,
    cost: Cost,
}

struct MinCostFlow {
    n: usize,
    graph: Vec<Vec<usize>>,
    edges: Vec<Edge>,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![vec![]; n],
            edges: vec![],
        }
    }
    fn add_edge(&mut self, s: usize, t: usize, cap: Flow, cost: Cost) {
        let edge_id = self.edges.len();
        self.edges.push(Edge {
            to: t,
            rev: edge_id + 1,
            cap,
            cost,
        });
        self.edges.push(Edge {
            to: s,
            rev: edge_id,
            cap: 0,
            cost: -cost,
        });
        self.graph[s].push(edge_id);
        self.graph[t].push(edge_id + 1);
    }
    fn mcmf(&mut self, s: usize, t: usize, mut flow: Flow) -> Cost {
        let mut ans = 0;
        let mut from_eid = vec![0; self.graph.len()];

        while flow > 0 {
            let mut dis = vec![MAX_COST; self.n];
            let mut vis = vec![false; self.n];

            dis[s] = 0;
            vis[s] = true;

            let mut q = VecDeque::new();
            q.push_back(s);
            while let Some(s) = q.pop_front() {
                vis[s] = false;
                for &eid in &self.graph[s] {
                    let e = &self.edges[eid];
                    if e.cap > 0 {
                        let cost = dis[s] + e.cost;
                        if cost < dis[e.to] {
                            dis[e.to] = cost;
                            from_eid[e.to] = eid;
                            if !vis[e.to] {
                                vis[e.to] = true;
                                q.push_back(e.to);
                            }
                        }
                    }
                }
            }

            if dis[t] == MAX_COST {
                return dis[t];
            }

            let mut cap = flow;
            let mut to = t;
            while to != s {
                cap = cap.min(self.edges[from_eid[to]].cap);
                to = self.edges[from_eid[to] ^ 1].to;
            }
            flow -= cap;
            ans += dis[t] * (cap as Cost);
            let mut to = t;
            while to != s {
                self.edges[from_eid[to]].cap -= cap;
                self.edges[from_eid[to] ^ 1].cap += cap;
                to = self.edges[from_eid[to] ^ 1].to;
            }
        }
        ans
    }
}

fn main() {
    use proconio::input;
    input! {
    n: usize,
    k: i64,
    table: [[i64;n];n]
    }
    let mut mcmf = MinCostFlow::new(n * 2 + 2);
    let s = n * 2;
    let t = s + 1;
    for i in 0..n {
        mcmf.add_edge(s, i, k, 0);
        mcmf.add_edge(i + n, t, k, 0);
    }

    const MAX_SCORE: i64 = 1e9 as i64;

    for i in 0..n {
        for j in 0..n {
            mcmf.add_edge(i, j + n, 1, MAX_SCORE - table[i][j]);
        }
    }
    // remove ?
    mcmf.add_edge(s, t, MAX_CAP, MAX_SCORE);

    mcmf.mcmf(s, t, k * n as Flow);

    let mut ans = 0;
    let mut board = vec![vec!['.'; n]; n];
    for i in 0..n {
        for &eid in &mcmf.graph[i] {
            let e = &mcmf.edges[eid];
            if e.cap == 0 && e.to < 2 * n {
                ans += table[i][e.to - n];

                let j = e.to - n;
                board[i][j] = 'X';
            }
        }
    }

    println!("{:?}", ans);
    for i in 0..n {
        for j in 0..n {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
