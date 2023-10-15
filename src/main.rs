use std::collections::HashMap;
use std::io::Write;
mod arith_dsl;
mod string_dsl;
use std::hash::Hash;

use itertools::{Itertools, MultiProduct};

use ::next_gen::prelude::*;
// use string_dsl::{S, N, Expr, Transition};
// use arith_dsl::{S as AS, eval as aeval};

fn main() {
    // _string_dsl_tests();

    // let program1 = AS::If(
    //     Box::new(AS::Lt(Box::new(AS::Input(0)), Box::new(AS::Input(1)))),
    //     Box::new(AS::Input(1)),
    //     Box::new(AS::Input(0)));

    // let input1 = vec![1, 2];
    // let output1 = aeval(program1.clone(), input1.clone());
    // println!("output1 is {:?}", output1);

    // let input2 = vec![2, 1];
    // let output2 = aeval(program1.clone(), input2.clone());
    // println!("output2 is {:?}", output2);

}

trait DSL {
    type Input;
    type Output;
    type S where Self::S: Clone;
    type Expr where Self::Expr: Clone;
    type NonTerminal where Self::NonTerminal: Clone;
    type Transition where Self::Transition: Clone;
    
    
    // this is a hack to mimic something like a field for each DSL
    fn production_list(a:u32) -> &'static [(Self::NonTerminal, u32, Self::Transition, &'static[Self::NonTerminal])]
    where
        Self::NonTerminal: Clone,
        Self::Transition: Clone;
    
    fn isStart(s: Self::NonTerminal) -> bool
    where
        Self::NonTerminal: Clone;
    
    fn eval(s: Self::S, input: Self::Input) -> Self::Output
    where
        Self::S: Clone;

    fn expr_of_nt(nt: Self::NonTerminal) -> Self::Expr
    where
        Self::NonTerminal: Clone,
        Self::Expr: Clone;

}

// bottom up synthesis algorithm
// the membership oracle with the input is represented by e, the input output pair
// might make the member ship oracle some sort of a trait?
// the output is the program, with start node S
// I did not abstract away the grammar for now
fn bottom_up_synthesis<T>(e: Vec<(T::Input, T::Output)>) -> <T as DSL>::S
where
    T: DSL,
    <T as DSL>::NonTerminal: Clone,
    <T as DSL>::S: Clone,
    <T as DSL>::Expr: Clone,
    (u32, <T as DSL>::NonTerminal): std::cmp::Eq,
    (u32, <T as DSL>::NonTerminal): Hash,
    <T as DSL>::Output: PartialEq
{
    let mut b: HashMap<(u32, T::NonTerminal), Vec<T::Expr>> = HashMap::new();

    for n in 0..10 {
        // assume the max height is 10
        mk_gen!(let new_terms_res = new_terms(n, b.clone()));
        for (a, t) in new_terms_res {
            // if T::isStart(a.clone()){
                // todo!()
                // match t.clone() {
                //     T::Expr::S(s) => {
                //         println!("{}", s.clone());
                //         let mut pass = true;
                //         for (input, output) in e {
                //             if T::eval(s.clone(), input) != output {
                //                 pass = false;
                //             }
                //         }
                //         std::io::stdout().flush().unwrap();
                //         if pass == true {
                //             println!("Found the program: {}", s);
                //             return s;
                //         }
                //     }
                //     _ => {
                //         panic! {"NonTerminal and Expr is not aligned"}
                //     }
                // }
            // }
            b.entry((n, a)).or_insert(Vec::new()).push(t);
            // eprintln!("b is {:?}", b.clone());
        }
    }
    panic! {"No program found"}
}

// fn bottom_up_synthesis(e: Vec<(String, String)>) -> S {
//     let mut b: HashMap<(u32, NonTerminal), Vec<Expr>> = HashMap::new();

//     for n in 0..10 {
//         // assume the max height is 10
//         mk_gen!(let new_terms_res = new_terms(n, b.clone()));
//         for (a, t) in new_terms_res {
//             if a == NonTerminal::S {
//                 match t.clone() {
//                     Expr::S(s) => {
//                         println!("{}", s.clone());
//                         let mut pass = true;
//                         for (input, output) in e.clone() {
//                             if eval(s.clone(), input.clone()) != output {
//                                 pass = false;
//                             }
//                         }
//                         std::io::stdout().flush().unwrap();
//                         if pass == true {
//                             println!("Found the program: {}", s);
//                             return s;
//                         }
//                     }
//                     _ => {
//                         panic! {"NonTerminal and Expr is not aligned"}
//                     }
//                 }
//             }
//             b.entry((n, a)).or_insert(Vec::new()).push(t);
//             // eprintln!("b is {:?}", b.clone());
//         }
//     }
//     panic! {"No program found"}
// }

// TODO: It's unsatisfying that the HashMap is borrowed everytime.
#[generator(yield((<T as DSL>::NonTerminal, <T as DSL>::Expr)))]
fn new_terms<T>(n: u32, b: HashMap<(u32, T::NonTerminal), Vec<<T as DSL>::Expr>>) 
where 
    T: DSL,
    <T as DSL>::NonTerminal: Clone,
    <T as DSL>::Expr: Clone,
    <T as DSL>::Transition: Clone,
    <T as DSL>::Transition: 'static,
    <T as DSL>::NonTerminal: 'static,
    (u32, <T as DSL>::NonTerminal): std::cmp::Eq,
    (u32, <T as DSL>::NonTerminal): Hash,
    <T as DSL>::NonTerminal: Copy

{
    // for all grammar productions

    // base case, arity and height are 0
    // if n == 0 && k == 0 {
    //   return (NonTerminal::S, S::Input);
    // cannot implement Copy trait for recursive enum (with Box)
    for (_nt, k, transition, subnt) in T::production_list(1) {
        if *k == 0 && n == 0 {
            yield_!((_nt.clone(), T::expr_of_nt(subnt[0].clone())));
        } else {
            // build subterms from the bank
            // eprintln!("sub-nonterminals are {:?} n is {}, k is {}, transition: {:?}", subnt, n, k, transition);
            let heights = (0..n).product_repeat(*k as usize).collect_vec();
            for ns in heights.clone() {
                if !ns.contains(&(n - 1)) {
                    continue;
                }
                let mut subterms: Vec<Vec<T::Expr>> = Vec::new();
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
                    // subject to change, but this is a close enough abstraction
                    yield_!((_nt.clone(), T::expr_of_nt(subnt[0].clone())));
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

// fn _string_dsl_tests() {
//     let input1 = "Nadia Polikarpova".to_string();
//     let output1 = "Nadia".to_string();

//     let input2 = "Loris D\'Antoni".to_string();
//     let output2 = "Loris".to_string();

//     let input3 = "Nadia Polikarpova".to_string();
//     let output3 = "Nadia Nadia".to_string();

//     let input4 = "Loris D\'Antoni".to_string();
//     let output4 = "Loris Loris".to_string();

//     let input5 = "hello".to_string();
//     let output5 = "h".to_string();

//     let input6 = "world".to_string();
//     let output6 = "w".to_string();

//     let input7 = "hello".to_string();
//     let output7 = "o".to_string();

//     let input8 = "world".to_string();
//     let output8 = "d".to_string();

//     // x[0..find(x," ")]
//     // this program synthesizes really fast
//     bottom_up_synthesis(vec![(input1, output1), (input2, output2)]);

//     // x[0..find(x," ")]+" "+x[0..find(x," ")]
//     // height = 4, there are 6*10^15 according to nadia's book
//     // bottom_up_synthesis(vec![(input3, output3), (input4, output4)]);

//     // x[0..1]
//     bottom_up_synthesis(vec![(input5, output5), (input6, output6)]);

//     // ("_" ++ x)[len(x)..len(x ++ "_")]
//     // bottom_up_synthesis(vec![(input7, output7), (input8, output8)]);
// }
