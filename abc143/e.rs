use std::io;
use std::cmp;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    let (n, m, l): usize = {
        let mut i = r.read_line().split_whitespace().map(|s| s.parse().unwrap());
        (i.unwrap(), i.unwrap(), i.unwrap())
    };


    let mut paths: Vec<Vec<usize>> = vec![vec![usize::max_value()]; m];
    for _ in 0..m {
        let mut i = r.read_line().split_whitespace().map(|s| s.parse().unwrap());
        let a = i.unwrap() - 1;
        let b = i.unwrap() - 1;
        let c = i.unwrap();
        paths[a][b] = c;
        paths[b][a] = c;
    }

    let q = r.read_line().parse().unwrap();
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let mut i = r.read_line().split_whitespace().map(|s| s.parse().unwrap());
        let s = i.unwrap() - 1;
        let t = i.unwrap() - 1;
        queries.push(cmp::min(s, t), cmp::max(s, t));
    }

    let ds = 
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
