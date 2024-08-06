use rand::Rng;

mod numbers;
use numbers::*;

use num_modular::ModularCoreOps;

#[allow(arithmetic_overflow)]
fn main() {
    const EPSILON: u32 = (1 << 24) - (1 << 16) + (1 << 8) - (1 << 0);
    const P: u32 = ((1 << 32) - EPSILON as u64) as u32;
    // let a = (1 << 31) + 425;
    // // let b = (1 << 20) + 333;
    // // let c = H::mult(a, b);
    // let d = H::mult_5(a);
    // println!("{}, {}", d, (a as u64 * (2 as u64).pow(5)) % (P as u64));

    let t = 15;
    let wrb = ((1u64 << 32) as u32 - (1 << t));
    let wrt = ((1u64 << 32) - 1) as u32;
    let prb = (P - (1 << t));
    let prt = P;

    let mut rng = rand::thread_rng();
    for i in prb..prt {
        for j in wrb..wrt {
            let a = H(rng.gen::<u32>());
            // let b = H(rng.gen::<u32>());
            // let a = H(3891629982);
            // let b = H(0);
            let result = a.prod_30();
            let correct = a.0.mulm(1 << 30, &P);
            // println!("result: {result:?}");
            if result.0 >= P && result.0 - (P as u32) != (correct as u32) {
                println!("first: {a:?},{result:?},{correct}");
                panic!();
            }
            if result.0 < P && result.0 != (correct as u32) {
                println!("second: {a:?},{result:?},{correct}");
                panic!();
            }
        }
    }
}

// use rand::Rng;

// mod numbers;
// use numbers::*;

// #[allow(arithmetic_overflow)]
// fn main() {
//     let pG: u128 = (1 << 64) - (1 << 32) + 1;
//     // let pH: u32 = ((1 << 32) as u64 - (1 << 24) + (1 << 16) - (1 << 8) + 1) as u32;
//     // ((2 as u64).pow(32) - (2 as u64).pow(24) + (2 as u64).pow(16) - (2 as u64).pow(8) + 1)
//     //     as u32;
//     // for a in (2 as u32).pow(16)..(2 as u32).pow(31) {
//     //     for b in (2 as u32).pow(16)..(2 as u32).pow(31) {
//     let mut rng = rand::thread_rng();

//     for i in (pG - (1 << 14))..pG {
//         for j in (pG - (1 << 14))..pG {
//             let a = rng.gen::<u64>();
//             let b = rng.gen::<u64>();
//             // let a = 628008511146584023;
//             // let b = 14368702434318396250;
//             let result = G::mult(a, b);
//             let correct = (a as u128) * (b as u128) % pG;
//             // println!("{}", correct);
//             if result != (correct as u64) && result - (pG as u64) != (correct as u64) {
//                 println!("{a},{b},{result},{correct}");
//                 panic!();
//             }
//         }
//         // let correct = (a as u64) * (b as u64) % (p as u64);
//         // if result != correct as u32 && result - p != correct as u32 {
//         //     println!("{a},{b},{result},{correct}");
//         //     panic!();
//     }
// }

// use primal::*;
// fn main() {
//     let mut p = 0;
//     // let lower_bound = 1 << 31;
//     // let upper_bound = 1 << 32;
//     // let mut b3: Vec<u32> = vec![0; 10];
//     // let mut b5: Vec<u32> = vec![0; 10];
//     // let mut b5: Vec<u32> = vec![0; 10];

//     // let mut b5:

//     // let top: u64 = 0xffffffffffffffff;
//     for k in ((1 << 56) - (1 << 8))..(1 << 56) {
//         if k % 2 == 0 {
//             continue;
//         }
//         p = k * (1 << 8) + 1;
//         // let diff = 64 - 10;
//         if is_prime(p) {

//     let mut d3 = 0;
//     let mut d5 = 0;
//     let mut d7 = 0;
//             // let r = (p << diffß
//             for i in 0..10 {
//                 if k % (3 as u64).pow((i as u32) + 1) == 0 {
//                     d3 += 1;
//                 }
//                 if k % (5 as u64).pow((i as u32) + 1) == 0 {
//                     d5 += 1;
//                 }
//                 if k % (7 as u64).pow((i as u32) + 1) == 0 {
//                     d7 += 1;
//                 }
//             }
//             let dt = 1.5849 * (d3 as f32) + 2.3219 * (d5 as f32) + 2.8073 * (d7 as f32);
//             // if dt > 8.0 {
//             println!("p: {}, k: {}, dt: {}", p, k, dt);
//             // }
//             // println!("p: {}, k: {}", p, k);
//         }
//     }
// }
