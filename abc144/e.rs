use std::io;
use std::collections::BTreeMap;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    let (n, k): (usize, usize) = {
        let mut i = r.read_line().split_whitespace().map(|s| s.parse().unwrap());
        (i.next().unwrap(), i.next().unwrap())
    };
    let mut a: Vec<usize> = r.read_line().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut f: Vec<usize> = r.read_line().split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    a.sort();
    f.sort_by(|a, b| b.cmp(a));

    let mut map = BTreeMap::new();
    a.iter().(f.iter()).map(|(&x, &y)| x * y).collect();
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
