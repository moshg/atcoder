use std::io;
use std::f64;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    let (a, b, x): (usize, usize, usize) = {
        let mut i = r.read_line().split_whitespace().map(|s| s.parse().unwrap());
        (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
    };
    // let (a, b, x) = (2, 2, 4);
    let (a, b, x) = (a as f64, b as f64, x as f64);

    let tant: f64;
    if x < a * a * b * 0.5 {
        tant = a * b * b * 0.5 / x;
    } else {
        tant = 2.0 * b / a - 2.0 * x / (a * a * a);
    }
    let t = tant.atan();
    let t = t / f64::consts::PI * 180.0;
    println!("{}", t);
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
