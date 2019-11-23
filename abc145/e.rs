use std::io;
use std::cmp;
use io_ext::Reader;

fn main() {
    let (t, mut eats) = input();
    // let t = 60;
    // let mut eats = vec![(10, 10), (100, 100)];
    eats.sort_by(|a, b| b.0.cmp(&a.0));
    println!("{}", answer(t, &eats));
}

fn input() -> (u64, Vec<(u64, u64)>) {
    let stdin = io::stdin();
    let mut r = Reader::new(stdin.lock());
    let (n, t): (usize, u64) = {
        let mut s = r.read_line().split_whitespace();
        (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
    };

    let mut eats: Vec<(u64, u64)> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut s = r.read_line().split_whitespace();
        let (a, b): (u64, u64) = (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap());
        eats.push((a, b));
    }
    (t, eats)
}

fn answer(t: u64, eats: &[(u64, u64)]) -> u64 {
    if t == 0 {
        return eats.iter().map(|&(a, b)| b).max().unwrap();
    }

    if eats.is_empty() {
        return 0;
    }

    let &(a, b) = eats.last().unwrap();
    
    if a > t {
        cmp::max(answer(t, &eats[..eats.len() - 1]), b)
    } else {
        cmp::max(answer(t, &eats[..eats.len() - 1]), answer(t - a, &eats[..eats.len() - 1]) + b)
    }
}

/// Extension of io module.
pub mod io_ext {
    use std::io::BufRead;

    pub struct Reader<R> {
        buf: String,
        inner: R,
    }

    impl<R> Reader<R> {
        #[inline]
        pub fn new(inner: R) -> Self {
            Reader { buf: String::new(), inner: inner }
        }

        #[inline]
        pub fn into_inner(self) -> R {
            self.inner
        }
    }

    impl<R: BufRead> Reader<R> {
        #[allow(deprecated)]
        #[inline]
        pub fn read_line(&mut self) -> &str {
            self.buf.clear();
            self.inner.read_line(&mut self.buf).unwrap_or_else(|e| panic!("{}", e));
            self.buf.trim_right()
        }
    }
}

/// Module supporting to parse strings.
pub mod parse {
    use std::str::FromStr;
    use std::borrow::Borrow;

    pub trait ParseNext: Iterator {
        fn parse_next<F: FromStr>(&mut self) -> F;
    }

    impl<'a, S: Borrow<str>, I: Iterator<Item=S>> ParseNext for I {
        fn parse_next<F: FromStr>(&mut self) -> F {
            if let Some(s) = self.next() {
                match s.borrow().parse() {
                    Ok(x) => x,
                    Err(_) => panic!("provided string cannot be parsed")
                }
            } else {
                panic!("iterator has no next element")
            }
        }
    }
}
