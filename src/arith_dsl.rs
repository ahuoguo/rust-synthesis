// AST and interpreter of a toy scheme-like language

// For simplicity, 0 is interpreted as false, otherwise true
use std::fmt;

#[derive(Clone, Debug)]
pub enum S {
    Input(usize),
    Add(Box<S>, Box<S>),
    Sub(Box<S>, Box<S>),
    Mul(Box<S>, Box<S>),
    Div(Box<S>, Box<S>),
    If(Box<S>, Box<S>, Box<S>),
    Eq(Box<S>, Box<S>),
    Lt(Box<S>, Box<S>),
    Not(Box<S>),
}

pub fn eval(s: S, input: Vec<u32>) -> Option<u32> {
    match s {
        S::Input(v) => {
            if v < input.len() {
                Some(input[v])
            } else {
                None
            }
        }
        S::Add(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input.clone())) {
            (Some(v1), Some(v2)) => Some(v1 + v2),
            _ => None,
        },
        S::Sub(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input.clone())) {
            (Some(v1), Some(v2)) => Some(v1 - v2),
            _ => None,
        },
        S::Mul(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input.clone())) {
            (Some(v1), Some(v2)) => Some(v1 * v2),
            _ => None,
        },
        S::Div(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input.clone())) {
            (Some(v1), Some(v2)) => {
                if v2 == 0 {
                    None
                } else {
                    Some(v1 / v2)
                }
            }
            _ => None,
        },
        S::If(s1, s2, s3) => match eval(*s1, input.clone()) {
            Some(v1) => {
                if v1 == 0 {
                    eval(*s3, input.clone())
                } else {
                    eval(*s2, input.clone())
                }
            }
            _ => None,
        },
        S::Eq(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input.clone())) {
            (Some(v1), Some(v2)) => {
                if v1 == v2 {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            _ => None,
        },
        S::Lt(s1, s2) => match (eval(*s1, input.clone()), eval(*s2, input.clone())) {
            (Some(v1), Some(v2)) => {
                if v1 < v2 {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            _ => None,
        },
        S::Not(s) => match eval(*s, input.clone()) {
            Some(v) => {
                if v == 0 {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            _ => None,
        },
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            S::Input(v) => write!(f, "args.{}", v),
            S::Add(s1, s2) => write!(f, "({} + {})", s1, s2),
            S::Sub(s1, s2) => write!(f, "({} - {})", s1, s2),
            S::Mul(s1, s2) => write!(f, "({} * {})", s1, s2),
            S::Div(s1, s2) => write!(f, "({} / {})", s1, s2),
            S::If(s1, s2, s3) => write!(f, "(if {} then {} else {})", s1, s2, s3),
            S::Eq(s1, s2) => write!(f, "({} == {})", s1, s2),
            S::Lt(s1, s2) => write!(f, "({} < {})", s1, s2),
            S::Not(s) => write!(f, "!{}", s),
        }
    }
}