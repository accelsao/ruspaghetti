// /////////
// Primes //
// /////////
#[derive(Debug)]
struct Primes {
    primes: Vec<usize>,
    len: usize,
}

impl Primes {
    fn new(n: usize) -> Self {
        let mut is_prime = vec![true; n];
        let mut primes = vec![];
        for i in 2..n {
            if is_prime[i] {
                primes.push(i);
            }
            for j in &primes {
                let k = *j * i;
                if k < n {
                    is_prime[k] = false;
                } else {
                    break;
                }
            }
        }
        let len = primes.len();
        Self { primes, len }
    }
}

fn main() {
    let p = Primes::new(10);
    assert_eq!(p.primes, vec![2, 3, 5, 7]);
}

// TODO
// Iterator
// Index
