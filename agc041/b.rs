use std::io;
use io_ext::Reader;
use parse::ParseAll;

fn main() {
    let stdin = io::stdin();
    let mut r = Reader::new(stdin.lock());
    let (problems, judges, votes, accepts) = r.read_line().split_whitespace().parse_all();
    let mut scores: Vec<usize> = r.read_line().split_whitespace().parse_all();
    scores.sort();
    let mut score_count: Vec<(usize, usize)> = Vec::new();
    let mut prev_score = usize::max_value();
    for score in scores {
        if score == prev_score {
            score_count.last_mut().unwrap().1 += 1;
        } else {
            score_count.push((score, 1));
            prev_score = score;
        }
    }
    let ans = answer(&score_count, problems, judges, votes, accepts);
    println!("{}", ans);
}

fn answer(scores: &[(usize, usize)], problems: usize, judges: usize, votes: usize, accepts: usize) -> usize {
    if problems <= accepts {
        return problems;
    }
    let mut bound_idx = 0;
    let mut bound = 0;
    let mut highers = 0;
    for (i, &(score, n)) in scores.iter().enumerate() {
        highers += n;
        if n >= accepts {
            bound_idx = i;
            bound = score;
            break;
        }
    }

    if bound_idx == scores.len() - 1 {
        return problems;
    }

    let mut lower = 0;
    let mut lowers = vec![0; scores.len()];
    for (i, &(_, n)) in scores.iter().enumerate().rev() {
        lower += n;
        lowers[i] = lower;
    }

    let mut count = highers;
    let mut should_vote = votes - (accepts - 1);
    for (&(score, n), &lower) in scores[(bound_idx + 1)..].iter().zip(lowers.iter()) {
        if should_vote > lower {
            break;
        }

        if bound <= score + judges {
            count += n;
        } else {
            break;
        }
    }
    count
}

/// A module for easy use of io.
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

/// Parsing Iterator.
pub mod parse {
    use std::borrow::Borrow;
    use std::str::FromStr;

    pub trait FromStrIterator {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(i: I) -> Self;
    }

    pub trait ParseAll {
        fn parse_all<F: FromStrIterator>(self) -> F;
    }

    impl<S: Borrow<str>, I: Iterator<Item=S>> ParseAll for I {
        #[inline]
        fn parse_all<F: FromStrIterator>(self) -> F {
            F::from_str_iter(self)
        }
    }

    fn parse<S: Borrow<str>, I: Iterator<Item=S>, F: FromStr>(i: &mut I) -> F {
        i.next().unwrap_or_else(|| panic!("too few strings error")).borrow().parse().unwrap_or_else(|_| panic!("parse error"))
    }

    // To avoid conflict, this is not implemented for `A` but `(A,)`.
    impl<A: FromStr> FromStrIterator for (A, ) {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
            let a = parse(&mut i);
            if i.next().is_some() {
                panic!("too many strings error");
            }
            (a, )
        }
    }

    impl<A: FromStr, B: FromStr> FromStrIterator for (A, B) {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
            let a = parse(&mut i);
            let b = parse(&mut i);
            if i.next().is_some() {
                panic!("too many strings error");
            }
            (a, b)
        }
    }

    impl<A: FromStr, B: FromStr, C: FromStr> FromStrIterator for (A, B, C) {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
            let a = parse(&mut i);
            let b = parse(&mut i);
            let c = parse(&mut i);
            if i.next().is_some() {
                panic!("too many strings error");
            }
            (a, b, c)
        }
    }

    impl<A: FromStr, B: FromStr, C: FromStr, D: FromStr> FromStrIterator for (A, B, C, D) {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
            let a = parse(&mut i);
            let b = parse(&mut i);
            let c = parse(&mut i);
            let d = parse(&mut i);
            if i.next().is_some() {
                panic!("too many strings error");
            }
            (a, b, c, d)
        }
    }

    impl<T: FromStr> FromStrIterator for Vec<T> {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(i: I) -> Self {
            i.map(|s| s.borrow().parse().unwrap_or_else(|_| panic!("parse error"))).collect()
        }
    }
}

pub mod modulo {
    /// Returns `x` + `y` mod `modulo`.
    ///
    /// `x < modulo` and `y < modulo` must hold.
    #[inline]
    pub fn add(x: u32, y: u32, modulo: u32) -> u32 {
        debug_assert!(modulo > 0);
        debug_assert!(x < modulo && y < modulo);
        let sum = x as u64 + y as u64;
        if sum <= modulo as u64 {
            sum as u32
        } else {
            (sum as u32).wrapping_sub(modulo)
        }
    }

    /// Returns `x` - `y` mod `modulo`.
    ///
    /// `x < modulo` and `y < modulo` must hold.
    #[inline]
    pub fn sub(x: u32, y: u32, modulo: u32) -> u32 {
        debug_assert!(0 < modulo);
        debug_assert!(x < modulo && y < modulo);
        if x >= y {
            x - y
        } else {
            modulo + x - y
        }
    }

    /// Returns `x` * `y` mod `modulo`.
    #[inline]
    pub fn mul(x: u32, y: u32, modulo: u32) -> u32 {
        ((x as u64 * y as u64) % modulo as u64) as u32
    }

    /// Returns `x`^ `y` mod `modulo`.
    pub fn pow(x: u32, mut y: u32, modulo: u32) -> u32 {
        debug_assert!(0 < modulo);
        let mut p = x;
        let mut ret = 1;

        while y != 0 {
            if y & 1 == 1 {
                ret = mul(ret, p, modulo);
            }
            p = mul(p, p, modulo);
            y >>= 1;
        }
        ret
    }

    fn is_prime(x: u32) -> bool {
        let sqrt = (x as f32).sqrt() as u32;
        for factor in 2..sqrt {
            if x % factor == 0 {
                return false;
            }
        }
        true
    }

    /// Returns 1 / `x`.
    ///
    /// `modulo` must be a prime number.
    #[inline]
    fn reciprocal(x: u32, modulo: u32) -> u32 {
        debug_assert!(0 < modulo && is_prime(modulo));
        pow(x, modulo - 2, modulo)
    }

    /// Returns `x` / `y` mod `modulo`.
    ///
    /// `modulo` must be a prime number.
    #[inline]
    pub fn div(x: u32, y: u32, modulo: u32) -> u32 {
        debug_assert!(0 < modulo && is_prime(modulo));
        mul(x, reciprocal(y, modulo), modulo)
    }

    /// Returns `n`P`k` mod `modulo`.
    pub fn perm(n: u32, k: u32, modulo: u32) -> u32 {
        let mut p = 1;
        for i in 0..k {
            p = mul(p, n - i, modulo);
        }
        p
    }

    /// Returns `n`C`k` mod `modulo`.
    pub fn comb(n: u32, k: u32, modulo: u32) -> u32 {
        let num = perm(n, k, modulo);
        let mut den = 1;
        for i in 0..k {
            den = mul(den, i + 1, modulo);
        }
        div(num, den, modulo)
    }
}