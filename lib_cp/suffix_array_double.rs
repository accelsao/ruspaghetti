use proconio::*;

fn construct_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let str = s.chars().collect::<Vec<_>>();

    let mut sa = vec![0; n];
    let mut rk = vec![0; n];

    for i in 0..n {
        sa[i] = i;
        rk[i] = str[i] as i32
    }

    let mut k = 1;
    while k < n {
        sa.sort_by_key(|&i| (rk[i], rk.get(i + k).unwrap_or(&-1)));

        let mut new_rk = vec![0; n];
        new_rk[sa[0]] = 0;
        for i in 1..n {
            new_rk[sa[i]] = new_rk[sa[i - 1]];
            let cmp_sa = |i, j| {
                if rk[i] != rk[j] {
                    return rk[i] < rk[j];
                }
                let ri = if i + k < n { rk[i + k] } else { -1 };
                let rj = if j + k < n { rk[j + k] } else { -1 };
                ri < rj
            };
            if cmp_sa(sa[i - 1], sa[i]) {
                new_rk[sa[i]] += 1;
            }
        }
        rk = new_rk;
        k *= 2;
    }
    sa
}

fn construct_lcp(s: &str) -> Vec<usize> {
    let n = s.len();
    let sa = construct_suffix_array(s);

    let str = s.chars().collect::<Vec<_>>();

    let mut lcp = vec![0; n];
    let mut rank = vec![0; n];

    for i in 0..n {
        rank[sa[i]] = i;
    }

    let mut k = 0;
    for i in 0..n {
        if rank[i] == n - 1 {
            k = 0;
            continue;
        }

        let j = sa[rank[i] + 1];

        while i + k < n && j + k < n && str[i + k] == str[j + k] {
            k += 1;
        }

        lcp[rank[i]] = k;

        if k > 0 {
            k -= 1;
        }
    }
    lcp
}

fn main() {
    input! {
    s: String,
    }
    let n = s.len();
    let lcp = construct_lcp(&s);

    let ans = n * (n + 1) / 2;
    let ans = ans - lcp.iter().sum::<usize>();

    println!("{:?}", ans);
}

/*
a
aba
ababa
banana
na
nana
*/
