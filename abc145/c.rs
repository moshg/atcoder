use std::io;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(stdin.lock());

    let n: usize = r.read_line().parse().unwrap();
    let mut poses = Vec::with_capacity(n);
    for _ in 0..n {
        let mut s = r.read_line().split_whitespace();
        let (x, y): (isize, isize) = (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap());
        poses.push((x, y));
    }
    // let n = 2;
    // let poses: Vec<(isize, isize)> = vec![(-879, 981), (-866, 890)];

    let mut sum: f64 = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (px, py) = poses[i];
            let (qx, qy) = poses[j];
            let dist = ((px - qx) * (px - qx) + (py - qy) * (py - qy)) as f64;
            let dist = dist.sqrt();
            sum += dist;
        }
    }

    let ans = sum * 2.0 / n as f64;

    println!("{}", ans);
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
