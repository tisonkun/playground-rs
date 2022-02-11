use std::cmp::Ordering;

use rand::Rng;

fn reverse(ns: &mut Vec<u8>, lo: usize, hi: usize) {
    let mut i = lo;
    let mut j = hi - 1;
    while i < j {
        ns.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn merge_sort_lookup(ns: &mut Vec<u8>, lo: usize, hi: usize) -> Option<u8> {
    if lo + 1 < hi {
        let mi = lo + ((hi - lo) >> 1);
        let x = merge_sort_lookup(ns, lo, mi);
        let y = merge_sort_lookup(ns, mi, hi);
        let z = merge_lookup(ns, lo, mi, hi);
        return x.or(y).or(z);
    } else {
        None
    }
}

fn merge_lookup(ns: &mut Vec<u8>, lo: usize, mi: usize, hi: usize) -> Option<u8> {
    let mut i = lo;
    let mut j = mi;
    while i < j && j < hi {
        let mut p = 0;
        while i < j {
            match ns[i].cmp(&ns[j]) {
                Ordering::Less => i += 1,
                Ordering::Equal => return Some(ns[i]),
                Ordering::Greater => break,
            }
        }

        if i == j {
            return None;
        }

        while j < hi {
            match ns[i].cmp(&ns[j]) {
                Ordering::Less => break,
                Ordering::Equal => return Some(ns[i]),
                Ordering::Greater => {
                    j += 1;
                    p += 1;
                }
            }
        }

        reverse(ns, i, j - p);
        reverse(ns, j - p, j);
        reverse(ns, i, j);
    }
    None
}

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let len = 10;
        let mut ns: Vec<u8> = (0..len).map(|_| rng.gen_range(0..=50)).collect();

        let mut cloned = ns.clone();

        let found = merge_sort_lookup(&mut ns, 0, len);
        match found {
            None => {
                let expected = cloned.len();
                cloned.sort();
                cloned.dedup();
                let got = cloned.len();
                assert_eq!(expected, got)
            }
            Some(u) => {
                let count = cloned.iter().filter(|n| **n == u).count();
                assert!(count > 1);
            }
        }
    }
}
