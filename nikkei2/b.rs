use std::io;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    let n = r.read_line().parse().unwrap();
    // let n = 7;
    let dists: Vec<usize> = r.read_line().split_whitespace().map(|s| s.parse().unwrap()).collect();
    // let dists = vec![0, 3, 2, 1, 2, 2, 1];

    let mut counts: Vec<usize> = vec![0; n];
    for &d in &dists {
        counts[d] += 1;
    }

    if dists[0] != 0 || counts[0] != 1 {
        println!("{}", 0);
        return;
    }

    let mut count: usize = 1;
    let mut ans: usize = 1; 
    for i in 0..(counts.len() - 1) {
        let prev = counts[i];
        let curr = counts[i + 1];
        
        if curr == 0 {
            if count != dists.len() {
                ans = 0;
            }
            break;
        }

        count += curr;
        let mul = prev % 998244353;
        for _ in 0..curr {
            ans *= mul;
            ans %= 998244353;
        }
    }

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
