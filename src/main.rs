use std::collections::HashMap;
use std::io::Write;
mod string_dsl;

use itertools::{Itertools, MultiProduct};

use ::next_gen::prelude::*;
use string_dsl::*;

fn main() {
    // x[0..find(x," ")]
    let program1 = S::SubString(
        Box::new(S::Input),
        Box::new(N::Zero),
        Box::new(N::Find(S::Input, S::Space)),
    );

    // x[0..find(x," ")]+" "+x[0..find(x," ")]
    let program2 = S::Append(
        Box::new(program1.clone()),
        Box::new(S::Append(Box::new(S::Space), Box::new(program1.clone()))),
    );

    let input1 = "Nadia Polikarpova".to_string();
    let output1 = eval(program1.clone(), input1.clone());
    println!("Output1 is: {}", output1);

    let input2 = "Loris D\'Antoni".to_string();
    let output2 = eval(program1, input2.clone());
    println!("Output2 is: {}", output2);

    let input3 = "Nadia Polikarpova".to_string();
    let output3 = eval(program2.clone(), input3.clone());
    println!("Output3 is: {}", output3);

    let input4 = "Loris D\'Antoni".to_string();
    let output4 = eval(program2, input4.clone());
    println!("Output4 is: {}", output4);

    // this program synthesizes really fast
    bottom_up_synthesis(vec![(input1, output1), (input2, output2)]);

    bottom_up_synthesis(vec![(input3, output3), (input4, output4)]);
}

// bottom up synthesis algorithm
// the member ship oracle with the input is represented by e, the input output pair
// the output is the program, with start node S
// I do not abstract away the grammar for now
fn bottom_up_synthesis(e: Vec<(String, String)>) -> S {
    let mut b: HashMap<(u32, NonTerminal), Vec<Expr>> = HashMap::new();

    for n in 0..10 {
        // assume the max height is 10
        mk_gen!(let new_terms_res = new_terms(n, b.clone()));
        for (a, t) in new_terms_res {
            // eprintln!("a is {:?}, t is {:?}", a.clone(), t.clone());
            if a == NonTerminal::S {
                match t.clone() {
                    Expr::S(s) => {
                        println!("{}", s.clone());
                        for (input, output) in e.clone() {
                            std::io::stdout().flush().unwrap();
                            if eval(s.clone(), input.clone()) == output {
                                println!("Found the program: {:?}", s);
                                return s;
                            }
                        }
                    }
                    _ => {
                        panic! {"NonTerminal and Expr is not aligned"}
                    }
                }
            }
            b.entry((n, a)).or_insert(Vec::new()).push(t);
            // eprintln!("b is {:?}", b.clone());
        }
    }
    panic! {"No program found"}
}

#[generator(yield((NonTerminal, Expr)))]
fn new_terms(n: u32, b: HashMap<(u32, NonTerminal), Vec<Expr>>) {
    // for all grammar productions

    // base case, arity and height are 0
    // if n == 0 && k == 0 {
    //   return (NonTerminal::S, S::Input);

    // cannot implement Copy trait for recursive enum (with Box)
    for (_nt, k, transition, subnt) in string_dsl::PRODUCTION {
        if *k == 0 && n == 0 {
            match transition {
                Transition::Input => {
                    yield_!((NonTerminal::S, Expr::S(S::Input)));
                }
                Transition::Space => {
                    yield_!((NonTerminal::S, Expr::S(S::Space)));
                }
                Transition::Zero => {
                    yield_!((NonTerminal::N, Expr::N(N::Zero)));
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
                let mut subterms: Vec<Vec<Expr>> = Vec::new();
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
                        Transition::Append => match (subterm[0].clone(), subterm[1].clone()) {
                            (Expr::S(s1), Expr::S(s2)) => {
                                yield_!((
                                    NonTerminal::S,
                                    Expr::S(S::Append(Box::new(s1), Box::new(s2)))
                                ));
                            }
                            _ => {
                                panic! {"production list encoded with errorneous information"}
                            }
                        },
                        Transition::SubString => {
                            match (subterm[0].clone(), subterm[1].clone(), subterm[2].clone()) {
                                (Expr::S(s1), Expr::N(n1), Expr::N(n2)) => {
                                    yield_!((
                                        NonTerminal::S,
                                        Expr::S(S::SubString(
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
                        Transition::Find => match (subterm[0].clone(), subterm[1].clone()) {
                            (Expr::S(s1), Expr::S(s2)) => {
                                yield_!((NonTerminal::N, Expr::N(N::Find(s1, s2))));
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
