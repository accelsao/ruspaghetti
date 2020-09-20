use proconio::*;

// Compressed sparse row
struct CSR {
    start: Vec<usize>,
    elist: Vec<usize>,
}

impl CSR {
    fn new(n: usize, edge: &[(usize, usize)]) -> Self {
        let mut start = vec![0; n + 1];
        edge.iter().for_each(|&(s, _)| start[s + 1] += 1);
        for i in 1..=n {
            start[i] += start[i - 1];
        }
        let mut index = start.clone();
        let mut elist = vec![0; edge.len()];
        for &(s, t) in edge.iter() {
            elist[index[s]] = t;
            index[s] += 1;
        }
        CSR { start, elist }
    }
}

struct State {
    graph: CSR,
    ts: usize,
    scc_id: usize,
    low: Vec<usize>,
    vis: Vec<usize>,
    ids: Vec<usize>,
    stack: Vec<usize>,
}

impl State {
    fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        Self {
            graph: CSR::new(n, edges),
            ts: 1,
            scc_id: 0,
            low: vec![0; n],
            vis: vec![0; n],
            ids: vec![n; n],
            stack: Vec::with_capacity(n),
        }
    }
}

struct SccGraph {
    n: usize,
    edge: Vec<(usize, usize)>,
}

impl SccGraph {
    fn new(n: usize) -> Self {
        Self { n, edge: vec![] }
    }
    fn add_edge(&mut self, s: usize, t: usize) {
        assert!(s < self.n);
        assert!(t < self.n);
        self.edge.push((s, t));
    }

    fn scc_ids(&self) -> (usize, Vec<usize>) {
        let mut state = State::new(self.n, &self.edge);

        fn dfs(v: usize, n: usize, state: &mut State) {
            state.low[v] = state.ts;
            state.vis[v] = state.ts;
            state.ts += 1;
            state.stack.push(v);
            for i in state.graph.start[v]..state.graph.start[v + 1] {
                let u = state.graph.elist[i];
                if state.vis[u] == 0 {
                    dfs(u, n, state);
                    state.low[v] = std::cmp::min(state.low[v], state.low[u]);
                } else if state.ids[u] == n {
                    state.low[v] = std::cmp::min(state.low[v], state.vis[u]);
                }
            }
            if state.low[v] == state.vis[v] {
                while let Some(u) = state.stack.pop() {
                    state.ids[u] = state.scc_id;
                    if u == v {
                        break;
                    }
                }
                state.scc_id += 1;
            }
        }

        for i in 0..self.n {
            if state.vis[i] == 0 {
                dfs(i, self.n, &mut state);
            }
        }

        for x in state.ids.iter_mut() {
            *x = state.scc_id - 1 - *x;
        }
        (state.scc_id, state.ids)
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
        let (scc_num, ids) = self.scc_ids();
        let mut groups = vec![vec![]; scc_num];
        for (i, &id) in ids.iter().enumerate() {
            groups[id].push(i);
        }
        groups
    }
}

#[fastout]
fn main() {
    input! {
    n: usize,
    m: usize,
    e: [(usize, usize); m],
    }
    let mut scc = SccGraph::new(n);
    for (s, t) in e {
        scc.add_edge(s, t);
    }

    let groups = scc.scc();

    println!("{}", groups.len());
    for i in 0..groups.len() {
        print!("{}", groups[i].len());
        for j in &groups[i] {
            print!(" {}", j);
        }
        println!();
    }
}
