// AST and interpreter of the string based language
// The parser might also be defined here

// TODO: probably need to extend this to a trait to abstract away the grammar
// where it shares the following operations:
// eval

pub static PRODUCTION: &'static [(NonTerminal, u32, Transition, &[NonTerminal])] = &[
    (NonTerminal::S, 0, Transition::Input, &[]),
    (NonTerminal::S, 0, Transition::Space, &[]),
    (NonTerminal::S, 2, Transition::Append, &[NonTerminal::S, NonTerminal::S]),
    (NonTerminal::S, 3, Transition::SubString, &[NonTerminal::S, NonTerminal::N, NonTerminal::N]),
    (NonTerminal::N, 0, Transition::Zero, &[]),
    (NonTerminal::N, 2, Transition::Find, &[NonTerminal::S, NonTerminal::S])
    ];

// p.17 nadia STRINGY DSL
#[derive(Clone, Debug)]
pub enum S {
    Input,
    Space,
    Append(Box<S>, Box<S>),
    SubString(Box<S>, Box<N>, Box<N>)
}

#[derive(Clone, Debug)]
pub enum N {
    Zero,
    Find(S, S)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum NonTerminal {
    S,
    N
}

#[derive(Debug, Clone)]
pub enum Expr {
    S(S),
    N(N)
}

#[derive(Clone, Debug)]
pub enum Transition {
    Input,
    Space,
    Append,
    SubString,
    Zero,
    Find
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
            s1.find(&s2).unwrap()
        }
    }
}