use std::io;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(stdin.lock());
    let s: Vec<u8> = r.read_line().as_bytes().iter().map(|&c| c - b'a').collect();
    let k: u64 = r.read_line().parse().unwrap();
    let ans = answer(&s, k);
    println!("{}", ans);
}

fn test() {
    assert_eq!(answer(&[0, 1, 1, 0, 0], 2), 4);
    assert_eq!(answer(&[0, 0], 81), 81);
    assert_eq!(answer(&[0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 6], 999993333), 8999939997);
}

fn answer(s: &[u8], k: u64) -> u64 {
    if s.len() == 1 {
        return k / 2;
    }

    let (first_contiguous, last_contiguous, mut count) = {
        let mut count = 0;
        let mut contiguous = 1;
        let mut prev = s[0];
        let mut first_contiguous = 0;
        for &c in &s[1..] {
            if c == prev {
                contiguous += 1;
            } else {
                prev = c;
                if first_contiguous == 0 {
                    first_contiguous = contiguous;
                } else {
                    count += contiguous / 2;
                }
                contiguous = 1;
            }
        }
        (first_contiguous, contiguous, count)
    };

    count += first_contiguous / 2 + last_contiguous / 2;
    if first_contiguous % 2 == 0 || last_contiguous % 2 == 0 {
        count * k
    } else {
        if s.first().unwrap() == s.last().unwrap() {
            (count * 2 + 2) * (k / 2) + count * (k % 2)
        } else {
            count * k
        }
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
