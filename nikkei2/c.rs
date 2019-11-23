use std::io;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    let mut a: Vec<i32> = r.read_line().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut b: Vec<i32> = r.read_line().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut c: Vec<(i32, i32)> = a.iter().map(|x| *x).zip(b.iter().map(|x| *x)).collect();
    a.sort();
    b.sort();
    let possible = true;
    for (&x, &y) in a.iter().zip(b.iter()) {
        if x > y {
            possible = false;
            break;
        }
    }

    if !possible {
        println!("No");
        return;
    }

    c.sort_by_key(|x| x.1);
    let mut a: Vec<i32> = c.iter().enumerate().map(|(i, x)| (i, x.0)).collect();
    a.sort_by_key(|x| x.1);
    
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
