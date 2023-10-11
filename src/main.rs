use std::collections::HashMap;
// use generator::*;

mod string_dsl;

use itertools::Itertools;
use string_dsl::*;
use ::next_gen::prelude::*;


fn main() {
    // x[0..find(x," ")]
    let program1 = S::SubString(
        Box::new(S::Input),
        Box::new(N::Zero),
        Box::new(N::Find(
            S::Input,
            S::Space
        ))
    );

    // x[0..find(x," ")]+" "+x[0..find(x," ")]
    let _program2 = S::Append(
        Box::new(program1.clone()),
        Box::new(S::Append(
            Box::new(S::Space),
            Box::new(program1.clone())
        ))
    );

    let input1 = "Nadia Polikarpova".to_string();
    let output1 = eval(program1.clone(), input1.clone());
    println!("Output1 is: {}", output1);

    let input2 = "Loris D\'Antoni".to_string();
    let output2 = eval(program1, input2.clone());
    println!("Output2 is: {}", output2);

    // let input3 = "Nadia Polikarpova".to_string();
    // let output3 = eval(program2.clone(), input3);
    // println!("Output3 is: {}", output3);

    // let input4 = "Loris D\'Antoni".to_string();
    // let output4 = eval(program2, input4);
    // println!("Output4 is: {}", output4);
    
    bottom_up_synthesis(vec![(input1, output1), (input2, output2)]);
}


// bottom up synthesis algorithm
// the member ship oracle with the input is represented by e, the input output pair
// the output is the program, with start node S
// I do not abstract away the grammar for now
fn bottom_up_synthesis(_e: Vec<(String, String)>) -> S {

    // temporary fix here..., it is hard to make a global vector in rust since allocation cannot happen in const/static
    // let production:Vec<(NonTerminal, u32, Transition, Vec<NonTerminal>) > = vec![
    // (NonTerminal::S, 0, Transition::Input, vec![]),
    // (NonTerminal::S, 0, Transition::Space, vec![]),
    // (NonTerminal::S, 1, Transition::Append, vec![NonTerminal::S, NonTerminal::S]),
    // (NonTerminal::S, 2, Transition::SubString, vec![NonTerminal::S, NonTerminal::N, NonTerminal::N]),
    // (NonTerminal::N, 0, Transition::Zero, vec![]),
    // (NonTerminal::N, 1, Transition::Find, vec![NonTerminal::S, NonTerminal::S])
    // ];


    let mut b:HashMap<(u32, NonTerminal), Vec<Expr>> = HashMap::new();

    for n in 0..10 { // assume the max height is 10
        mk_gen!(let new_terms_res = new_terms(n, b.clone()));
        for (a, t) in new_terms_res {
            eprintln!("a is {:?}, t is {:?}", a.clone(), t.clone());
            eprintln!("b is {:?}", b.clone());
            b.entry((n, a)).or_insert(Vec::new()).push(t);
            eprintln!("hihihi");
        }
    }

    return S::Input;
}


// fn new_terms<'a>(n: u32, b: HashMap<(u32, NonTerminal), Vec<Expr>>) -> Generator<'static, (), (NonTerminal, Expr)> {
//     Gn::new_scoped(move |mut s| {
//         // for all grammar productions

//         // base case, arity and height are 0
//         // if n == 0 && k == 0 {
//         //   return (NonTerminal::S, S::Input);

//         // cannot implement Copy trait for recursive enum (with Box)
//         for (_nt, k, transition, subnt ) in string_dsl::PRODUCTION {
//             if *k == 0 && n == 0 {
//                 match transition {
//                     Transition::Input => {
//                         s.yield_with((NonTerminal::S, Expr::S(S::Input)));
//                     },
//                     Transition::Space => {
//                         s.yield_with((NonTerminal::S, Expr::S(S::Space)));
//                     },
//                     Transition::Zero => {
//                         s.yield_with((NonTerminal::N, Expr::N(N::Zero)));
//                     },
//                     _ => { 
//                         panic!{"production list encoded with errorneous information"}
//                     }
//                 }
//             } else {
//                 // build subterms from the bank
//                 eprintln!("sub-nonterminals are {:?} n is {}, k is {}, transition: {:?}", subnt, n, k, transition);
//                 let heights = (0..n).permutations(*k as usize).collect_vec();
//                 eprintln!("heights are {:?}", heights);
//                 if heights.is_empty() {
//                     eprintln!("continue");
//                     continue;
//                 }
//                 for ns in heights.clone() {
//                     if !ns.contains(&(n-1)) {
//                         continue;
//                     }
//                     let mut subterms:Vec<Vec<Expr>> = Vec::new();
//                     for i in 0..*k {
//                         println!("i is {}", i);
//                         eprintln!("ns[i] is {}, subnt[i] is {:?}", ns[i as usize], subnt[i as usize]);
//                         let subterm = b.get(&(ns[i as usize], subnt[i as usize])).unwrap();
//                         subterms.push(subterm.clone());
//                     }
//                     // for subterm in subterms.iter().multi_cartesian_product() {
//                     //     match transition {
//                     //         Transition::Append => {
//                     //             match (subterm[0].clone(), subterm[1].clone()) {
//                     //                 (Expr::S(s1), Expr::S(s2)) => {
//                     //                     s.yield_with((NonTerminal::S, Expr::S(S::Append(Box::new(s1), Box::new(s2)))));
//                     //                 },
//                     //                 _ => {
//                     //                     panic!{"production list encoded with errorneous information"}
//                     //                 }
//                     //             }
//                     //         },
//                     //         Transition::SubString => {
//                     //             match (subterm[0].clone(), subterm[1].clone(), subterm[2].clone()) {
//                     //                 (Expr::S(s1), Expr::N(n1), Expr::N(n2)) => {
//                     //                     s.yield_with((NonTerminal::S, Expr::S(S::SubString(Box::new(s1), Box::new(n1), Box::new(n2))))); 
//                     //                 },
//                     //                 _ => {
//                     //                     panic!{"production list encoded with errorneous information"}
//                     //                 }
//                     //             }
//                     //         },
//                     //         Transition::Find => {
//                     //             match (subterm[0].clone(), subterm[1].clone()) {
//                     //                 (Expr::S(s1), Expr::S(s2)) => {
//                     //                     s.yield_with((NonTerminal::N, Expr::N(N::Find(s1, s2))));
//                     //                 },
//                     //                 _ => {
//                     //                     panic!{"production list encoded with errorneous information"}
//                     //                 }
//                     //             }
//                     //         },
//                     //         _ => { 
//                     //             panic!{"production list encoded with errorneous information"}
//                     //         }
//                     //     }
//                     // }

//                 }
//             }

//         }
//         done!();
//     })
// }

#[generator(yield((NonTerminal, Expr)))]
fn new_terms (n: u32, b: HashMap<(u32, NonTerminal), Vec<Expr>>)
{
        // for all grammar productions

        // base case, arity and height are 0
        // if n == 0 && k == 0 {
        //   return (NonTerminal::S, S::Input);

        // cannot implement Copy trait for recursive enum (with Box)
        for (_nt, k, transition, subnt ) in string_dsl::PRODUCTION {
            if *k == 0 && n == 0 {
                match transition {
                    Transition::Input => {
                        yield_!((NonTerminal::S, Expr::S(S::Input)));
                    },
                    Transition::Space => {
                        yield_!((NonTerminal::S, Expr::S(S::Space)));
                    },
                    Transition::Zero => {
                        yield_!((NonTerminal::N, Expr::N(N::Zero)));
                    },
                    _ => { 
                        panic!{"production list encoded with errorneous information"}
                    }
                }
            } else {
                // build subterms from the bank
                eprintln!("sub-nonterminals are {:?} n is {}, k is {}, transition: {:?}", subnt, n, k, transition);
                let heights = (0..n).permutations(*k as usize).collect_vec();
                eprintln!("heights are {:?}", heights);
                if heights.is_empty() {
                    eprintln!("continue");
                    continue;
                }
                for ns in heights.clone() {
                    if !ns.contains(&(n-1)) {
                        continue;
                    }
                    let mut subterms:Vec<Vec<Expr>> = Vec::new();
                    for i in 0..*k {
                        println!("i is {}", i);
                        eprintln!("ns[i] is {}, subnt[i] is {:?}", ns[i as usize], subnt[i as usize]);
                        let subterm = b.get(&(ns[i as usize], subnt[i as usize])).unwrap();
                        subterms.push(subterm.clone());
                    }
                    // for subterm in subterms.iter().multi_cartesian_product() {
                    //     match transition {
                    //         Transition::Append => {
                    //             match (subterm[0].clone(), subterm[1].clone()) {
                    //                 (Expr::S(s1), Expr::S(s2)) => {
                    //                     s.yield_with((NonTerminal::S, Expr::S(S::Append(Box::new(s1), Box::new(s2)))));
                    //                 },
                    //                 _ => {
                    //                     panic!{"production list encoded with errorneous information"}
                    //                 }
                    //             }
                    //         },
                    //         Transition::SubString => {
                    //             match (subterm[0].clone(), subterm[1].clone(), subterm[2].clone()) {
                    //                 (Expr::S(s1), Expr::N(n1), Expr::N(n2)) => {
                    //                     s.yield_with((NonTerminal::S, Expr::S(S::SubString(Box::new(s1), Box::new(n1), Box::new(n2))))); 
                    //                 },
                    //                 _ => {
                    //                     panic!{"production list encoded with errorneous information"}
                    //                 }
                    //             }
                    //         },
                    //         Transition::Find => {
                    //             match (subterm[0].clone(), subterm[1].clone()) {
                    //                 (Expr::S(s1), Expr::S(s2)) => {
                    //                     s.yield_with((NonTerminal::N, Expr::N(N::Find(s1, s2))));
                    //                 },
                    //                 _ => {
                    //                     panic!{"production list encoded with errorneous information"}
                    //                 }
                    //             }
                    //         },
                    //         _ => { 
                    //             panic!{"production list encoded with errorneous information"}
                    //         }
                    //     }
                    // }

                }
            }

        }
}


// fn allcombinations(n:u32, k:u32) -> Vec<Vec<u32>> {
//     // create a vector of vectors, each with length k, and each element is a number from 0 to n-1,
//     // they also need to have one index equal to n-1
//     // there are n * n * ... * n = n^k such vectors
//     let mut result:Vec<Vec<u32>> = Vec::new();
//     for i in 0..k {
//         let mut v:Vec<u32> = Vec::new();
//         for j in 0..k {
//             if j == i {
//                 v.push(n-1);
//             } else {
//             }
//         }
//         result.push(v);
//     }
//     return result;
// }

// // note that new terms is implemented as an interator
// fn new_terms(n:u32, B:HashMap<(u32, NonTerminal), Vec<Expr>>) -> (NonTerminal, Expr) {
//     // for all grammar productions
//     for 

//     // base case, arity and height are 0
//     // if n == 0 && k == 0 {
//     //   return (NonTerminal::S, S::Input);


//     // build subterms from the bank

//     return (NonTerminal::S, S::Input);
// }