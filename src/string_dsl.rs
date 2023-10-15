// AST and interpreter of a string based DSL
// The parser might also be defined here

// TODO: probably need to extend this to a trait to abstract away the grammar
// where it shares the following operations:
// eval
use std::fmt;

pub static PRODUCTION: &[(NonTerminal, u32, Transition, &[NonTerminal])] = &[
    (NonTerminal::S, 0, Transition::Input, &[]),
    (NonTerminal::S, 0, Transition::Space, &[]),
    (
        NonTerminal::S,
        2,
        Transition::Append,
        &[NonTerminal::S, NonTerminal::S],
    ),
    (
        NonTerminal::S,
        3,
        Transition::SubString,
        &[NonTerminal::S, NonTerminal::N, NonTerminal::N],
    ),
    (NonTerminal::N, 0, Transition::Zero, &[]),
    (
        NonTerminal::N,
        2,
        Transition::Find,
        &[NonTerminal::S, NonTerminal::S],
    ),
    (NonTerminal::N, 1, Transition::Len, &[NonTerminal::S]),
];

// p.17 nadia STRINGY DSL
#[derive(Clone, Debug)]
pub enum S {
    Input,
    Space,
    Append(Box<S>, Box<S>),
    SubString(Box<S>, Box<N>, Box<N>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            S::Input => write!(f, "x"),
            S::Space => write!(f, "\"_\""),
            S::Append(s1, s2) => write!(f, "{} ++ {}", s1, s2),
            S::SubString(s, n1, n2) => write!(f, "({})[{}..{}]", s, n1, n2),
        }
    }
}

#[derive(Clone, Debug)]
pub enum N {
    Zero,
    Find(S, S),
    Len(S),
}

impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            N::Zero => write!(f, "0"),
            N::Find(s1, s2) => write!(f, "find({}, {})", s1, s2),
            N::Len(s) => write!(f, "len({})", s),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum NonTerminal {
    S,
    N,
}

#[derive(Debug, Clone)]
pub enum Expr {
    S(S),
    N(N),
}

#[derive(Clone, Debug)]
pub enum Transition {
    Input,
    Space,
    Append,
    SubString,
    Zero,
    Find,
    Len,
}

// interpreter
pub fn eval(s: S, input: String) -> Option<String> {
    match s {
        S::Input => Some(input),
        S::Space => Some(" ".to_string()),
        S::Append(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input)) {
            (Some(v1), Some(v2)) => Some(v1 + &v2),
            _ => None,
        },
        S::SubString(s, n1, n2) => {
            let s = eval(*s, input.clone());
            let n1 = eval_n(*n1, &input);
            let n2 = eval_n(*n2, &input);
            // is n1 .. n2 does not make sense return empty string
            match (s, n1, n2) {
                (Some(s), Some(n1), Some(n2)) => {
                    if n1 > n2 || n2 > s.len() {
                        None
                    } else {
                        Some(s[n1..n2].to_string())
                    }
                }
                _ => None,
            }
        }
    }
}

pub fn eval_n(n: N, input: &str) -> Option<usize> {
    match n {
        N::Zero => Some(0),
        N::Find(s1, s2) => {
            let s1 = eval(s1, input.to_owned());
            let s2 = eval(s2, input.to_owned());
            match (s1, s2) {
                (Some(s1), Some(s2)) => s1.find(&s2),
                _ => None,
            }
        }
        N::Len(s) => eval(s, input.to_owned()).map(|s| s.len()),
    }
}

pub fn size(s: &S) -> usize {
    match s {
        S::Input => 1,
        S::Space => 1,
        S::Append(s1, s2) => 1 + size(s1) + size(s2),
        S::SubString(s, n1, n2) => 1 + size(s) + size_n(n1) + size_n(n2),
    }
}

fn size_n(n: &N) -> usize {
    match n {
        N::Zero => 1,
        N::Find(s1, s2) => 1 + size(s1) + size(s2),
        N::Len(s) => 1 + size(s),
    }
}
