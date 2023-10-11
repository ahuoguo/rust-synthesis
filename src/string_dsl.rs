// AST and interpreter of the string based language
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
            S::SubString(s, n1, n2) => write!(f, "{}[{}..{}])", s, n1, n2),
        }
    }
}

#[derive(Clone, Debug)]
pub enum N {
    Zero,
    Find(S, S),
}

impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            N::Zero => write!(f, "0"),
            N::Find(s1, s2) => write!(f, "find({}, {})", s1, s2),
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
}

// interpreter
pub fn eval(s: S, input: String) -> String {
    match s {
        S::Input => input,
        S::Space => " ".to_string(),
        S::Append(s1, s2) => eval(*s1, input.clone()) + &eval(*s2, input),
        S::SubString(s, n1, n2) => {
            let s = eval(*s, input.clone());
            let n1 = eval_n(*n1, &input);
            let n2 = eval_n(*n2, &input);
            // is n1 .. n2 does not make sense return empty string
            if n1 > n2 {
                return "".to_string();
            }
            if n2 > s.len() {
                return "".to_string();
            }
            s[n1..n2].to_string()
        }
    }
}

pub fn eval_n(n: N, input: &str) -> usize {
    match n {
        N::Zero => 0,
        N::Find(s1, s2) => {
            let s1 = eval(s1, input.to_owned());
            let s2 = eval(s2, input.to_owned());
            s1.find(&s2).unwrap_or(0)
        }
    }
}
