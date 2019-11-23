use std::io;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(stdin.lock());
    
    let mut s = r.read_line().split_whitespace();
    let (x, y): (u64, u64) = (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap());
    // let (x, y): (u64, u64) = (3, 3);
    // let (x, y) = (2, 2);
    // let (x, y) = (999999, 999999);

    println!("{}", answer(x, y));
}

fn answer(x: u64, y: u64) -> u64 {
    if (x + y) % 3 != 0 {
        return 0;
    }

    let mut pair = None;
    let sum = (x + y) / 3;
    for k in 0..(sum + 1) {
        let l = sum - k;
        let p = (k + l * 2, k * 2 + l);
        if p == (x, y) {
            pair = Some((k, l));
            break;
        }
    }

    if pair.is_none() {
        return 0;
    }

    let (k, l) = pair.unwrap();
    comb(k + l, l)
}

const R: u64 = 1000000000 + 7;

fn comb(n: u64, k: u64) -> u64 {
    let n = n % R;
    let k = if k <= n / 2 {
        k % R
    } else {
        (n - k) % R
    };

    let mut s = 1;
    for i in 0..k {
        s *= (n - i);
        s %= R;
    }
    let mut d = 1;
    for i in 1..(k + 1) {
        d *= i;
        d %= R;
    }

    for i in 0..R {
        if (i * d) % R == s {
            return i
        }
    }
    panic!()
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
