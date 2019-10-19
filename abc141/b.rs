use std::io;
use std::io::Write;
use io_ext::Reader;
use parse::ParseNext;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    let (n, k, q): (usize, isize, isize) = {
        let s = r.read_line();
        let mut split = s.split_whitespace();
        (split.parse_next(), split.parse_next(), split.parse_next())
    };

    let mut points = vec![0; n];
    for _ in 0..q {
        let ans = r.read_line().parse::<usize>().unwrap() - 1;
        points[ans] += 1;
    }

    let stdout = io::stdout();
    let mut w = io::BufWriter::new(stdout.lock());
    for pt in points {
        if k + pt - q > 0 {
            writeln!(w, "Yes");
        } else {
            writeln!(w, "No");
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
