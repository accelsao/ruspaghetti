#![allow(unused)]
///////////
// Dinic //
///////////

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    cap: u64,
    rev: usize,
}

struct Dinic {
    n: usize,
    edges: Vec<Vec<Edge>>,
    dis: Vec<u64>,
    iter: Vec<usize>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![vec![]; n],
            dis: vec![0; n],
            iter: vec![0; n],
        }
    }
    fn add_edge(&mut self, s: usize, t: usize, cap: u64) {
        let edge_id = self.edges[t].len();
        self.edges[s].push(Edge {
            to: t,
            cap,
            rev: edge_id,
        });
        let edge_id = self.edges[s].len() - 1;
        self.edges[t].push(Edge {
            to: s,
            cap: 0,
            rev: edge_id,
        });
    }
    fn dfs(&mut self, v: usize, t: usize, flow: u64) -> u64 {
        if v == t {
            return flow;
        }
        for i in self.iter[v]..self.edges[v].len() {
            let e = self.edges[v][i];
            if e.cap > 0 && self.dis[v] < self.dis[e.to] {
                let f = self.dfs(e.to, t, std::cmp::min(e.cap, flow));
                if f > 0 {
                    self.edges[v][i].cap -= f;
                    self.edges[e.to][e.rev].cap += f;
                    return f;
                }
            }
            self.iter[v] += 1;
        }
        0
    }
    fn bfs(&mut self, s: usize) {
        self.dis = vec![0; self.n];
        self.dis[s] = 1;
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        while let Some(from) = q.pop_front() {
            for &e in &self.edges[from] {
                if e.cap > 0 && self.dis[e.to] == 0 {
                    self.dis[e.to] = self.dis[from] + 1;
                    q.push_back(e.to);
                }
            }
        }
    }
    fn calc(&mut self, s: usize, t: usize) -> u64 {
        let mut flow = 0;
        loop {
            self.bfs(s);

            if self.dis[t] == 0 {
                return flow;
            }
            self.iter = vec![0; self.n];
            let mut f = 0;
            loop {
                let res = self.dfs(s, t, std::u64::MAX);
                if res == 0 {
                    break;
                }
                f += res;
            }
            flow += f;
        }
    }
}

fn main() {
    // Problem on AtCoder
    // https://atcoder.jp/contests/practice2/tasks/practice2_d

    use proconio::input;
    use proconio::marker::Chars;
    input! {
    n:usize,
    m:usize,
    mut table: [Chars; n]
    }

    let mut dinic = Dinic::new(n * m + 2);
    for i in 0..n {
        for j in 0..m {
            if table[i][j] == '#' {
                continue;
            }
            if (i + j) % 2 == 1 {
                dinic.add_edge(i * m + j + 1, n * m + 1, 1);
                continue;
            }
            dinic.add_edge(0, i * m + j + 1, 1);
            let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for &(dx, dy) in &dir {
                let nx = i.wrapping_add(dx as usize);
                let ny = j.wrapping_add(dy as usize);
                if nx < n && ny < m && table[nx][ny] != '#' {
                    dinic.add_edge(i * m + j + 1, nx * m + ny + 1, 1);
                }
            }
        }
    }

    let ans = dinic.calc(0, n * m + 1);

    for &e in &dinic.edges[0] {
        if e.cap == 1 {
            continue;
        }
        for &e1 in &dinic.edges[e.to] {
            if e1.cap == 0 {
                let (px, py) = ((e.to - 1) / m, (e.to - 1) % m);
                let (nx, ny) = ((e1.to - 1) / m, (e1.to - 1) % m);

                if px == nx {
                    table[px][std::cmp::min(py, ny)] = '>';
                    table[px][std::cmp::max(py, ny)] = '<';
                } else {
                    table[std::cmp::min(px, nx)][py] = 'v';
                    table[std::cmp::max(px, nx)][py] = '^';
                }
            }
        }
    }

    println!("{}", ans);

    for i in 0..n {
        for j in 0..m {
            print!("{}", table[i][j]);
        }
        println!();
    }
}
