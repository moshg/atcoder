use std::io;
use parse::ParseNext;
use std::cmp::max;

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Job {
    delay: u64,
    reward: u64,
}

impl Job {
    pub fn new(delay: u64, reward: u64) -> Job {
        Job { delay: delay, reward: reward }
    }
}

fn input() -> (Vec<Job>, u64) {
    let stdin = io::stdin();
    let mut r = read::Reader::new(io::BufReader::new(stdin.lock()));
    let (n, m): (u64, u64);
    {
        let mut s = r.read_line().unwrap().split_whitespace();
        n = s.parse_next().unwrap();
        m = s.parse_next().unwrap();
    };
    let mut jobs = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut s = r.read_line().unwrap().split_whitespace();
        let a = s.parse_next().unwrap();
        let b = s.parse_next().unwrap();
        let job = Job::new(a, b);
        jobs.push(job);
    }
    (jobs, m)
}

fn knapsack(jobs: &[Job], rest: u64) -> u64 {
    if let Some(&job) = jobs.last() {
        let cut_jobs = &jobs[..(jobs.len() - 1)];
        if job.delay > rest {
            knapsack(cut_jobs, rest)
        } else {
            max(knapsack(cut_jobs, rest), knapsack(cut_jobs, rest - job.delay) + job.reward)
        }
    } else {
        0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Calc<'a> {
    Yet { jobs: &'a [Job], rest: u64 },
    Max(u64, u64),
}

impl<'a> Calc<'a> {
    fn yet(jobs: &'a [Job], rest: u64) -> Calc<'a> {
        Calc::Yet { jobs: jobs, rest: rest }
    }
}

fn knapsack_stack(jobs: &[Job], rest: u64) -> u64 {
    unsafe {
        let mut result = 0;
        let mut calcs: Vec<(*mut u64, Calc)> = Vec::new();
        calcs.push((&mut result, Calc::yet(jobs, rest)));

        loop {
            if let Some((result, calc)) = calcs.pop() {
                match calc {
                    Calc::Max(x, y) => { *result += max(x, y) }
                    Calc::Yet { jobs, rest } => {
                        if let Some((job, jobs)) = jobs.split_last() {
                            if job.delay > rest {
                                calcs.push((result, Calc::yet(jobs, rest)));
                            } else {
                                calcs.push((result, Calc::Max(0, job.reward)));
                                let (res0, res1): (*mut u64, *mut u64) = {
                                    if let &mut (_, Calc::Max(ref mut res0, ref mut res1)) = calcs.last_mut().unwrap() {
                                        (res0, res1)
                                    } else {
                                        panic!("unreachable");
                                    }
                                };
                                calcs.push((res0, Calc::yet(jobs, rest)));
                                calcs.push((res1, Calc::yet(jobs, rest - job.delay)));
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        result
    }
}

fn main() {
    let (jobs, rest) = input();
    println!("{}", knapsack_stack(&jobs, rest));
}

mod tests {
    use super::{Job, knapsack_stack};

    fn test_knapsack_stack() {
        let jobs = [Job::new(4, 3), Job::new(4, 1), Job::new(2, 2)];
        let rest = 4;
        assert_eq!(5, knapsack_stack(&jobs, rest));
    }
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
        #[allow(deprecated)]
        #[inline]
        pub fn read_line(&mut self) -> Result<&str> {
            self.buf.clear();
            self.inner.read_line(&mut self.buf)?;
            Ok(self.buf.trim_right())
        }
    }
}
