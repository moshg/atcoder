use std::io;
use std::cmp;
use std::collections::VecDeque;
use io_ext::Reader;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(io::BufReader::new(stdin.lock()));
    r.read_line();
    let s: Vec<u8> = r.read_line().as_bytes().iter().map(|&c| c - b'a').collect();

    let mut slices = VecDeque::with_capacity(26);
    for _ in 0..26 {
        slices.push_back((1usize, Vec::new()));
    }
    for (i, &c) in s.iter().enumerate() {
        slices[c as usize].1.push(i);
    }

    let mut max_len = 1;
    while let Some((len, mut indices)) = slices.pop_back() {
        let c = 0;
        while c < 26 {
            let mut longer_indices = Vec::new();
            let mut i = indices.len() - 1;
            loop {
                if indices[i] + len >= s.len() {
                    indices.remove(i);
                    if i == 0 { break; }
                    i -= 1;
                    continue;
                }
                if s[indices[i] + 1] == c {
                    longer_indices.push(indices[i]);
                    indices.remove(i);
                }
                if i == 0 { break; }
                i -= 1;
            }
            if longer_indices.len() >= 2 && (longer_indices.last().unwrap() - longer_indices.first().unwrap() >= len) {
                slices.push_front((len + 1, longer_indices));
                max_len = cmp::max(max_len, len + 1);
            }
        }
    }

    println!("{}", max_len);
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
