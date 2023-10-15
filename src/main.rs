use ::next_gen::prelude::*;
use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;

mod arith_dsl;
mod string_dsl;
use arith_dsl::{eval as aeval, Transition as ATransition, S as AS};
use string_dsl::{Expr as SExpr, NonTerminal as SNonTerminal, Transition as STransition, N, S};

fn main() {
    _string_dsl_tests();

    _arith_dsl_tests();
}

// bottom up synthesis algorithm
// the membership oracle with the input is represented by e, the input output pair
// output is the program displayed as a string
fn bottom_up_synthesis<T>(e: Vec<(T::Input, T::Output)>) -> String
where
    T: Dsl,
    T::Input: Clone,
    T::Output: Clone,
    <T as Dsl>::Output: PartialEq,
{
    if T::is_stringdsl() {
        let mut counter = 0;
        let mut b: HashMap<(u32, SNonTerminal), Vec<SExpr>> = HashMap::new();

        // assume the max height is 10
        for n in 0..10 {
            mk_gen!(let new_terms_res = stringdsl_new_terms(n, b.clone()));
            for (a, t) in new_terms_res {
                if a == SNonTerminal::S {
                    match t.clone() {
                        SExpr::S(s) => {
                            // println!("{}", s.clone());
                            let mut pass = true;
                            for (input, output) in e.clone() {
                                if string_dsl::eval(s.clone(), T::to_stringdsl_input(input))
                                    != T::to_stringdsl_output(output)
                                {
                                    pass = false;
                                }
                            }
                            if pass {
                                println!("Found the program: {}", s);
                                println!("counter is {}, height is {}", counter, n);
                                return s.to_string();
                            }
                        }
                        _ => {
                            panic! {"SNonTerminal and SExpr is not aligned"}
                        }
                    }
                }
                counter += 1;
                b.entry((n, a)).or_insert(Vec::new()).push(t);
            }
        }
    }

    if T::is_arithdsl() {
        let mut b: HashMap<u32, Vec<AS>> = HashMap::new();
        let mut counter = 0;

        let mut max_input_len = T::to_arithdsl_input(e[0].0.clone()).len();
        for (input, _) in e.clone() {
            if T::to_arithdsl_input(input.clone()).len() >= max_input_len {
                max_input_len = T::to_arithdsl_input(input).len();
            }
        }

        // assume the max height is 10
        for n in 0..10 {
            mk_gen!(let new_terms_res = arithdsl_new_terms(max_input_len, n, b.clone()));
            for s in new_terms_res {
                // println!("{}", s.clone());
                let mut pass = true;
                for (input, output) in e.clone() {
                    if aeval(s.clone(), T::to_arithdsl_input(input))
                        != T::to_arithdsl_output(output)
                    {
                        pass = false;
                    }
                }
                if pass {
                    println!("Found the program: {}", s);
                    println!("counter is {}, height is {}", counter, n);
                    return format!("{}", s);
                }
                b.entry(n).or_insert(Vec::new()).push(s);
                counter += 1;
            }
        }
    }

    panic! {"No program found"}
}

// we do not need to specify nonterminal becuase there's only one
#[generator(yield(AS))]
fn arithdsl_new_terms(len: usize, n: u32, b: HashMap<u32, Vec<AS>>) {
    for (k, transition) in arith_dsl::PRODUCTION {
        if *k == 0 && n == 0 {
            match transition {
                ATransition::Input => {
                    for i in 0..len {
                        yield_!(AS::Input(i));
                    }
                }
                _ => {
                    panic! {"production list encoded with errorneous information"}
                }
            }
        } else {
            let heights = (0..n).product_repeat(*k as usize).collect_vec();
            for ns in heights.clone() {
                if !ns.contains(&(n - 1)) {
                    continue;
                }
                let mut subterms: Vec<Vec<AS>> = Vec::new();
                for i in 0..*k {
                    let subterm = b.get(&(ns[i as usize])).unwrap();
                    subterms.push(subterm.clone());
                }
                for subterm in subterms.iter().multi_cartesian_product() {
                    match transition {
                        ATransition::Add => {
                            yield_!(AS::Add(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone())
                            ));
                        }
                        ATransition::Sub => {
                            yield_!(AS::Sub(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone())
                            ));
                        }
                        ATransition::Mul => {
                            yield_!(AS::Mul(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone())
                            ));
                        }
                        ATransition::Div => {
                            yield_!(AS::Div(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone())
                            ));
                        }
                        ATransition::If => {
                            yield_!(AS::If(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone()),
                                Box::new(subterm[2].clone())
                            ));
                        }
                        ATransition::Eq => {
                            yield_!(AS::Eq(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone())
                            ));
                        }
                        ATransition::Lt => {
                            yield_!(AS::Lt(
                                Box::new(subterm[0].clone()),
                                Box::new(subterm[1].clone())
                            ));
                        }
                        ATransition::Not => {
                            yield_!(AS::Not(Box::new(subterm[0].clone())));
                        }
                        _ => {
                            panic! {"production list encoded with errorneous information"}
                        }
                    }
                }
            }
        }
    }
}

// TODO: It's unsatisfying that the HashMap is borrowed everytime.
#[generator(yield((SNonTerminal, SExpr)))]
fn stringdsl_new_terms(n: u32, b: HashMap<(u32, SNonTerminal), Vec<SExpr>>) {
    // for all grammar productions

    // base case, arity and height are 0
    // if n == 0 && k == 0 {
    //   return (SNonTerminal::S, S::Input);

    // cannot implement Copy trait for recursive enum (with Box)
    for (_nt, k, transition, subnt) in string_dsl::PRODUCTION {
        if *k == 0 && n == 0 {
            match transition {
                STransition::Input => {
                    yield_!((SNonTerminal::S, SExpr::S(S::Input)));
                }
                STransition::Space => {
                    yield_!((SNonTerminal::S, SExpr::S(S::Space)));
                }
                STransition::Zero => {
                    yield_!((SNonTerminal::N, SExpr::N(N::Zero)));
                }
                _ => {
                    panic! {"production list encoded with errorneous information"}
                }
            }
        } else {
            // build subterms from the bank
            // eprintln!("sub-nonterminals are {:?} n is {}, k is {}, transition: {:?}", subnt, n, k, transition);
            let heights = (0..n).product_repeat(*k as usize).collect_vec();
            for ns in heights.clone() {
                if !ns.contains(&(n - 1)) {
                    continue;
                }
                let mut subterms: Vec<Vec<SExpr>> = Vec::new();
                for i in 0..*k {
                    // println!("i is {}", i);
                    // eprintln!("ns[i] is {}, subnt[i] is {:?}", ns[i as usize], subnt[i as usize]);
                    // println!("ns, subnt are {:?}, {:?}", ns, subnt);
                    // println!("b is {:?}", b);
                    let subterm = b.get(&(ns[i as usize], subnt[i as usize])).unwrap();
                    subterms.push(subterm.clone());
                }
                for subterm in subterms.iter().multi_cartesian_product() {
                    // println!("subterm is {:?}", subterm);
                    match transition {
                        STransition::Append => match (subterm[0].clone(), subterm[1].clone()) {
                            (SExpr::S(s1), SExpr::S(s2)) => {
                                yield_!((
                                    SNonTerminal::S,
                                    SExpr::S(S::Append(Box::new(s1), Box::new(s2)))
                                ));
                            }
                            _ => {
                                panic! {"production list encoded with errorneous information"}
                            }
                        },
                        STransition::SubString => {
                            match (subterm[0].clone(), subterm[1].clone(), subterm[2].clone()) {
                                (SExpr::S(s1), SExpr::N(n1), SExpr::N(n2)) => {
                                    yield_!((
                                        SNonTerminal::S,
                                        SExpr::S(S::SubString(
                                            Box::new(s1),
                                            Box::new(n1),
                                            Box::new(n2)
                                        ))
                                    ));
                                }
                                _ => {
                                    panic! {"production list encoded with errorneous information"}
                                }
                            }
                        }
                        STransition::Find => match (subterm[0].clone(), subterm[1].clone()) {
                            (SExpr::S(s1), SExpr::S(s2)) => {
                                yield_!((SNonTerminal::N, SExpr::N(N::Find(s1, s2))));
                            }
                            _ => {
                                panic! {"production list encoded with errorneous information"}
                            }
                        },
                        STransition::Len => match subterm[0].clone() {
                            SExpr::S(s) => {
                                yield_!((SNonTerminal::N, SExpr::N(N::Len(s))));
                            }
                            _ => {
                                panic! {"production list encoded with errorneous information"}
                            }
                        },
                        _ => {
                            panic! {"production list encoded with errorneous information"}
                        }
                    }
                }
            }
        }
    }
}

/// https://stackoverflow.com/questions/44139493/in-rust-what-is-the-proper-way-to-replicate-pythons-repeat-parameter-in-iter
/// Rust version of Python's itertools.product().
/// It returns the cartesian product of the input iterables, and it is
/// semantically equivalent to `repeat` nested for loops.
///
/// # Arguments
///
/// * `it` - An iterator over a cloneable data structure
/// * `repeat` - Number of repetitions of the given iterator
pub trait ProductRepeat: Iterator + Clone
where
    Self::Item: Clone,
{
    fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
        std::iter::repeat(self)
            .take(repeat)
            .multi_cartesian_product()
    }
}

impl<T: Iterator + Clone> ProductRepeat for T where T::Item: Clone {}

// an attempt to at least put the two synthesizer into one function
// general generic support from traits is kinda impossible because the generators
// does not support generics
// see https://github.com/danielhenrymantilla/next-gen-rs/issues/14
trait Dsl {
    type Input;
    type Output;
    type Program;

    fn is_stringdsl() -> bool;
    fn is_arithdsl() -> bool;
    // fn eval(prog: Self::Program, input: Self::Input) -> Self::Output;

    fn to_stringdsl_input(_: Self::Input) -> String;
    fn to_arithdsl_input(_: Self::Input) -> Vec<u32>;

    fn to_arithdsl_output(_: Self::Output) -> Option<u32>;
    fn to_stringdsl_output(_: Self::Output) -> String;
}

struct StringDsl {}

impl Dsl for StringDsl {
    type Input = String;
    type Output = String;
    type Program = S;

    fn is_stringdsl() -> bool {
        true
    }

    fn is_arithdsl() -> bool {
        false
    }

    fn to_stringdsl_input(input: Self::Input) -> String {
        input
    }

    fn to_arithdsl_input(_: Self::Input) -> Vec<u32> {
        panic! {"StringDsl does not have an arith input"}
    }

    fn to_arithdsl_output(_: Self::Output) -> Option<u32> {
        panic! {"StringDsl does not have an arith output"}
    }

    fn to_stringdsl_output(output: Self::Output) -> String {
        output
    }
    // fn eval(prog: S, input: Self::Input) -> Self::Output {
    //     string_dsl::eval(prog, input)
    // }
}

struct ArithDsl {}

impl Dsl for ArithDsl {
    type Input = Vec<u32>;
    type Output = Option<u32>;
    type Program = AS;

    fn is_stringdsl() -> bool {
        false
    }

    fn is_arithdsl() -> bool {
        true
    }

    fn to_stringdsl_input(_: Self::Input) -> String {
        panic! {"ArithDsl does not have a string input"}
    }

    fn to_arithdsl_input(input: Self::Input) -> Vec<u32> {
        input
    }

    fn to_arithdsl_output(output: Self::Output) -> Option<u32> {
        output
    }

    fn to_stringdsl_output(_: Self::Output) -> String {
        panic! {"ArithDsl does not have a string output"}
    }
    // fn eval(prog: AS, input: Self::Input) -> Self::Output {
    //     arith_dsl::eval(prog, input)
    // }
}

fn _string_dsl_tests() {
    println!("Testing String Dsl");
    let input1 = "Nadia Polikarpova".to_string();
    let output1 = "Nadia".to_string();

    let input2 = "Loris D\'Antoni".to_string();
    let output2 = "Loris".to_string();

    let input3 = "Nadia Polikarpova".to_string();
    let output3 = "Nadia Nadia".to_string();

    let input4 = "Loris D\'Antoni".to_string();
    let output4 = "Loris Loris".to_string();

    let input5 = "hello".to_string();
    let output5 = "h".to_string();

    let input6 = "world".to_string();
    let output6 = "w".to_string();

    let input7 = "hello".to_string();
    let output7 = "o".to_string();

    let input8 = "world".to_string();
    let output8 = "d".to_string();

    // x[0..find(x," ")]
    // this program synthesizes really fast
    bottom_up_synthesis::<StringDsl>(vec![(input1, output1), (input2, output2)]);

    // x[0..find(x," ")]+" "+x[0..find(x," ")]
    // height = 4, there are 6*10^15 according to nadia's book
    // bottom_up_synthesis(vec![(input3, output3), (input4, output4)]);

    // x[0..1]
    bottom_up_synthesis::<StringDsl>(vec![(input5, output5), (input6, output6)]);

    // ("_" ++ x)[len(x)..len(x ++ "_")]
    bottom_up_synthesis::<StringDsl>(vec![(input7, output7), (input8, output8)]);
}

fn _arith_dsl_tests() {
    println!("Testing Arith Dsl");
    let input1 = vec![1, 2];
    // let output1 = aeval(program1.clone(), input1.clone());
    // println!("output1 is {:?}", output1);
    let output1 = Some(2);

    let input2 = vec![2, 1];
    // let output2 = aeval(program1.clone(), input2.clone());
    // println!("output2 is {:?}", output2);
    let output2 = Some(2);

    let input3 = vec![2, 4];
    let output3 = Some(4);

    let input4 = vec![4, 2];
    let output4 = Some(4);

    let input5 = vec![20, 10];
    let output5 = Some(20);

    bottom_up_synthesis::<ArithDsl>(vec![
        (input1, output1),
        (input2, output2),
        (input3, output3),
        (input4, output4),
        (input5, output5),
    ]);
}
