use std::io::{self, Write};
use parse::ParseNext;
use std::collections::HashMap;

fn main() {
    let str_count: Vec<u64> = {
        let stdin = io::stdin();
        let mut r = read::Reader::new(io::BufReader::new(stdin));
        let n = r.read_line().unwrap().parse().unwrap();
        let mut str_count = HashMap::with_capacity(n);
        for _ in 0..n {
            let mut s = r.read_line().unwrap().as_bytes().to_vec();
            s.sort();

            let mut count = str_count.entry(s).or_insert(0u64);
            *count += 1;
        }
        str_count.values().map(|x| *x).collect()
    };

    let mut sum = 0;
    for count in str_count {
        sum += count * (count - 1) / 2;
    }

    println!("{}", sum);
}

/// 文字列のパースの補助モジュール.
pub mod parse {
    use std::error;
    use std::fmt;
    use std::str::FromStr;

    #[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
    pub enum Error<E> {
        GetNextError,
        ParseErr(E),
    }

    impl<E: fmt::Display> fmt::Display for Error<E> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            match self {
                &Error::GetNextError => f.write_str("Iterator has no next element"),
                &Error::ParseErr(ref e) => write!(f, "{}", e)
            }
        }
    }

    impl<E: error::Error> error::Error for Error<E> {
        #[inline]
        fn description(&self) -> &str {
            match self {
                &Error::GetNextError => "Iterator error",
                &Error::ParseErr(ref e) => e.description()
            }
        }
    }

    pub trait ParseNext: Iterator {
        fn parse_next<F: FromStr>(&mut self) -> Result<F, Error<F::Err>>;
    }

    impl<'a, S: AsRef<str>, I: Iterator<Item=S>> ParseNext for I {
        fn parse_next<F: FromStr>(&mut self) -> Result<F, Error<F::Err>> {
            if let Some(s) = self.next() {
                match s.as_ref().parse() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(Error::ParseErr(e))
                }
            } else {
                Err(Error::GetNextError)
            }
        }
    }
}

/// 標準入力読み取りモジュール.
pub mod read {
    use std::io::{BufRead, Result};

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
        #[inline]
        pub fn read_line(&mut self) -> Result<&str> {
            self.buf.clear();
            self.inner.read_line(&mut self.buf)?;
            Ok(self.buf.trim_right())
        }
    }
}
