#![allow(unused)]
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { buffer: vec![] }
    }
    fn next<F: FromStr>(&mut self) -> F {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("parsing fails");
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("reading fails");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    fn next_string(&mut self) -> String {
        assert_eq!(self.buffer.len(), 0);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("reading fails");
        input
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.next();
    let k: usize = scanner.next();
    let d: usize = scanner.next();

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scanner.next();
    }

    a.sort();

    let mut b = vec![0usize];
    b.append(&mut a);

    let mut q = VecDeque::new();
    let mut valid = vec![false; n + 1];
    valid[0] = true;

    for i in k..=n {
        if valid[i - k] {
            q.push_back(i - k);
        }
        while !q.is_empty() && b[q.front().unwrap() + 1] + d < b[i] {
            q.pop_front();
        }
        if !q.is_empty() {
            valid[i] = true
        }
    }
    if valid[n] {
        println!("YES")
    } else {
        println!("NO")
    }
}
