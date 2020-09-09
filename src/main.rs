#![allow(unused)]

// //////
// I/O //
// //////
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { buffer: vec![] }
    }
    fn next<F: std::str::FromStr>(&mut self) -> F {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("parsing fails");
            }
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("reading fails");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    fn next_string(&mut self) -> String {
        assert_eq!(self.buffer.len(), 0);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("reading fails");
        input
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let mut n: usize = scanner.next();
    println!("{:?}", n);
}
