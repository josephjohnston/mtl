use num_modular::ModularCoreOps;

use super::params::*;

pub fn go(major_input: &[Input], constants: &[Output]) -> Vec<Output> {
    let mut output = vec![0; E * F * G * D];
    for e in 0..E {
        for f in 0..F {
            for g in 0..G {
                let global_element_read_index = (e * F + f) * G + g;
                let input_range =
                    global_element_read_index * D..(global_element_read_index + 1) * D;
                let minor_components: &[Output] =
                    &constants[global_element_read_index * D..(global_element_read_index + 1) * D];
                // we want to process for each block e separately. but we need to know where to write it. we're writing all to output.
                let major_input_slice: &[Input] = &major_input[input_range];
                let major = Ring::init(major_input_slice);
                let reduced_coefs = major.reduce(NUMBER_OF_DECOMPS);
                let multiplied_coefs =
                    Ring::multiply(NUMBER_OF_DECOMPS, &reduced_coefs, minor_components);
                let output_coefs = multiplied_coefs;

                // let correct_coefs = r.check(k, print);
                // let global_element_write_index = (e * 1 + 0) * G + g;
                // let output_range =
                // global_element_write_index * D..(global_element_write_index + 1) * D;
                for d in 0..D {
                    let index = ((e * F + 0) * G + g) * D + d;
                    // output[index] = output[index].addm(output_coefs[d], &P);
                    output[index] = output[index].addm(output_coefs[d], &P);
                }
                // .clone_from_slice(&output_coefs);
            }
        }
    }

    println!("\nCPU COEFS");
    for (i, x) in output.iter().enumerate() {
        if i % (1 << 10) == 0 {
            println!("{i}: {x}");
        }
    }

    output

    // // 2, 6, 4, 8, 3, 7, 5, 9
    // // 2, 3, 4, 5, 6, 7, 8, 9
    // let mut left = [2, 6, 4, 8, 3, 7, 5, 9];
    // // [1723542064, 1305395542, 1002929091, 3613929278];
    // // [2, 3, 4, 5, 6, 7, 8, 9];
    // // [1723542064, 1305395542, 1002929091, 3613929278];
    // let mut right = [0, 0, 0, 0, 0, 0, 0, 1];
    // // [203, 101, 666, 220, 223, 402, 585, 189];
    // dual_karatsuba(&mut left, 0, &mut right, 0, 8, true);
    // println!("RESULTS");
    // println!("left: {left:?}");
    // println!("right: {right:?}");
    // vec![0, 0]
}

fn dual_karatsuba(
    left: &mut [u32],
    left_index: usize,
    right: &mut [u32],
    right_index: usize,
    Theta: usize,
    preserve: bool,
) {
    let mut middle_array = vec![0; Theta];
    let middle = &mut middle_array;
    let middle_index = 0;
    // 1: base case
    if Theta == 1 {
        left[left_index] = left[left_index].mulm(right[right_index], &P);
        return;
    }

    // 3: copy upper half of left to middle
    for theta in 0..Theta / 2 {
        middle[middle_index + theta] = left[left_index + Theta / 2 + theta];
    }

    // 4: recursively multiply tops
    dual_karatsuba(
        middle,
        middle_index,
        right,
        right_index + Theta / 2,
        Theta / 2,
        true,
    );

    // 5: add left and right lower to upper halves
    for theta in 0..Theta / 2 {
        left[left_index + Theta / 2 + theta] =
            left[left_index + Theta / 2 + theta].addm(left[left_index + theta], &P);
        right[right_index + Theta / 2 + theta] =
            right[right_index + Theta / 2 + theta].addm(right[right_index + theta], &P);
    }

    // 6: recursively multiply middles and bottoms
    dual_karatsuba(left, left_index, right, right_index, Theta / 2, preserve);
    dual_karatsuba(
        left,
        left_index + Theta / 2,
        right,
        right_index + Theta / 2,
        Theta / 2,
        preserve,
    );

    // 7: subtract bottom and top from middle
    for theta in 0..Theta / 2 {
        left[left_index + Theta / 2 + theta] =
            left[left_index + Theta / 2 + theta].subm(middle[middle_index + theta], &P);
        left[left_index + Theta / 2 + theta] =
            left[left_index + Theta / 2 + theta].subm(left[left_index + theta], &P);
    }
    // 8: finite field reduction
    let zeta = 1 << 20;
    left[left_index] =
        left[left_index].addm(middle[middle_index + Theta / 2 - 1].mulm(zeta, &P), &P);
    for theta in 1..Theta / 2 {
        let power_basis_theta = rho(theta, Theta / 2);
        let power_basis_middle_theta = rho(power_basis_theta - 1, Theta / 2);
        left[left_index + theta] =
            left[left_index + theta].addm(middle[middle_index + power_basis_middle_theta], &P);
    }

    // 9: preservation of right
    if preserve {
        for theta in 0..Theta / 2 {
            right[right_index + Theta / 2 + theta] =
                right[right_index + Theta / 2 + theta].subm(right[right_index + theta], &P);
        }
    }
}

#[derive(Debug)]
pub struct Ring {
    pub coefs: Vec<Output>,
}
impl Ring {
    pub fn init(input: &[Input]) -> Self {
        let mut r = Self { coefs: vec![0; D] };
        for i in 0..D {
            r.coefs[i] = input[i] as Output;
        }
        // Self::coefs_to_arrays(&mut r.arrays, &r.coefs);
        r
    }
    pub fn reduce(&self, k: usize) -> Vec<Output> {
        let d = D / (1 << k);
        let mut reduced_coefs = vec![0; D];
        for i in 0..(1 << k) {
            let zeta = Self::get_zeta(k + 1, i);
            let result = self.reduce_to_irreducible(d, zeta);
            for j in 0..d {
                reduced_coefs[i * d + j] = result[j];
            }
        }
        reduced_coefs
    }
    fn reduce_to_irreducible(&self, d: usize, zeta: Output) -> Vec<Output> {
        let mut result = vec![0; d];
        for j in 0..d {
            let mut zeta_acc = 1;
            for i in 0..(D / d) {
                let index = j + i * d;
                let coef = self.coefs[index];
                result[j] = result[j].addm(coef.mulm(zeta_acc, &P), &P);
                zeta_acc = zeta_acc.mulm(zeta, &P);
            }
        }
        result
    }
    pub fn multiply(
        k: usize,
        major_components: &[Output],
        minor_components: &[Output],
    ) -> Vec<Output> {
        let component_size = T_J * S / (1 << K);
        let mut output = vec![0; D];
        for component_index in 0..D / DEG {
            let range_bottom = component_index * component_size;
            let range_top = (component_index + 1) * component_size;
            let minor_component = &minor_components[range_bottom..range_top];
            let minor_component = vec![
                3614796953, 1208427060, 1889015752, 3198863462, 3614796953, 1208427060, 1889015752,
                3198863462,
            ];
            // ];
            let major_component = &major_components[range_bottom..range_top];
            Self::multiply_component(
                k,
                component_index,
                major_component,
                &minor_component,
                &mut output[range_bottom..range_top],
            );
        }
        output
    }
    pub fn multiply_component(
        k: usize,
        component_index: usize,
        major_component: &[Output],
        minor_component: &[Output],
        output: &mut [Output],
    ) {
        let zeta = Self::get_zeta(k + 1, component_index);
        let component_size = T_J * S / (1 << K);
        for i in 0..component_size {
            for j in 0..component_size {
                let mut value = minor_component[i].mulm(major_component[j], &P);
                if i + j >= component_size {
                    value = value.mulm(zeta, &P);
                }
                let index = (i + j) % component_size;
                output[index] = output[index].addm(value, &P);
            }
        }
    }
    pub fn add(&mut self, other: &Ring) {
        for i in 0..D {
            self.coefs[i] = self.coefs[i].addm(other.coefs[i], &P);
        }
    }
    fn get_zeta(k: usize, index: usize) -> Output {
        let prims_1 = [1 << 8];
        let prims_2 = [
            // (1 << 20), (1 << 30).mulm(1 << 30, &P)
            (1 << 20),
            4277206785,
        ];
        let prims_3 = [
            // 1 << 10,
            // (1 << 30).mulm(1 << 20, &P),
            // 1 << 30,
            // (1 << 30).mulm(1 << 30, &P).mulm(1 << 10, &P),
            1024, 4278254337, 1073741824, 3204513537,
        ];
        let prims_4 = [
            // // 5, 45 => 10
            // 1 << 5,
            // (1 << 30).mulm(1 << 15, &P),
            // // 25, 65 => 50
            // 1 << 25,
            // (1 << 30).mulm(1 << 30, &P).mulm(1 << 5, &P),
            // // 15, 55 => 30
            // 1 << 15,
            // (1 << 30).mulm(1 << 25, &P),
            // // 75, 35 => 79
            // (1 << 30).mulm(1 << 30, &P).mulm(1 << 15, &P),
            // (1 << 30).mulm(1 << 5, &P),
            32, 4278255329, 33554432, 4244700929, 32768, 4278222593, 4144559881, 133695480,
        ];
        let prims_5 = [
            16707839, 4261547522, 4261539330, 16716031, 4274061053, 4194308, 4274061061, 4194300,
            534650848, 3743604513, 534912992, 3743342369, 4144037761, 134217600, 4144037505,
            134217856,
        ];
        let prims_6 = [
            364914777, 3913340584, 2274230434, 2004024927, 1464515241, 2813740120, 1441048032,
            2837207329, 3120762142, 1157493219, 45032751, 4233222610, 196321259, 4081934102,
            947271947, 3330983414, 4184386525, 93868836, 3065742370, 1212512991, 919248094,
            3359007267, 2278185239, 2000070122, 171586511, 4106668850, 531848519, 3746406842,
            1274452609, 3003802752, 3982137898, 296117463,
        ];
        let prims_7 = [
            1297642494, 2980612867, 3327754660, 950500701, 2526751946, 1751503415, 2129504484,
            2148750877, 3020261559, 1257993802, 3810020456, 468234905, 3847465774, 430789587,
            3970313073, 307942288, 2689265513, 1588989848, 2766251085, 1512004276, 1388567172,
            2889688189, 3842193303, 436062058, 2954927500, 1323327861, 3786866165, 491389196,
            3159035588, 1119219773, 2626659467, 1651595894, 2504948723, 1773306638, 1488462141,
            2789793220, 3151931493, 1126323868, 2392531113, 1885724248, 3830654479, 447600882,
            1816320888, 2461934473, 1128492723, 3149762638, 3708275820, 569979541, 3081630698,
            1196624663, 3779079003, 499176358, 2234052728, 2044202633, 1762621666, 2515633695,
            3037601520, 1240653841, 786573619, 3491681742, 1139377988, 3138877373, 4065946328,
            212309033,
        ];
        let prims_8 = [
            493853244, 3784402117, 2630285104, 1647970257, 871589258, 3406666103, 2389324427,
            1888930934, 2968537725, 1309717636, 2882271469, 1395983892, 2221324090, 2056931271,
            3728040527, 550214834, 1792685315, 2485570046, 4069366704, 208888657, 338212691,
            3940042670, 10783282, 4267472079, 2529644974, 1748610387, 2406181663, 1872073698,
            2011959971, 2266295390, 3933190337, 345065024, 1122335902, 3155919459, 562582394,
            3715672967, 1578728461, 2699526900, 1480102279, 2798153082, 889615164, 3388640197,
            2589549385, 1688705976, 302463957, 3975791404, 819753580, 3458501781, 1816837538,
            2461417823, 2239014032, 2039241329, 599443123, 3678812238, 394504728, 3883750633,
            1081892113, 3196363248, 2521481523, 1756773838, 210614787, 4067640574, 2069158492,
            2209096869, 2122591668, 2155663693, 3775120428, 503134933, 1819528072, 2458727289,
            180144644, 4098110717, 1486373247, 2791882114, 1670676750, 2607578611, 529152400,
            3749102961, 3265551773, 1012703588, 206566095, 4071689266, 3541041349, 737214012,
            1934204752, 2344050609, 1889168591, 2389086770, 3720435503, 557819858, 1998977010,
            2279278351, 2331859679, 1946395682, 2078683782, 2199571579, 1026655235, 3251600126,
            2800282348, 1477973013, 1058032482, 3220222879, 1155858166, 3122397195, 3909251897,
            369003464, 2761418424, 1516836937, 4043927916, 234327445, 1373075368, 2905179993,
            2126899470, 2151355891, 276736331, 4001519030, 1013149118, 3265106243, 3965176830,
            313078531, 1805271112, 2472984249, 1462002270, 2816253091, 3979203491, 299051870,
            3886952625, 391302736,
        ];
        if k == 1 {
            prims_1[index]
        } else if k == 2 {
            prims_2[index]
        } else if k == 3 {
            prims_3[index]
        } else if k == 4 {
            prims_4[index]
        } else if k == 5 {
            prims_5[index]
        } else if k == 6 {
            prims_6[index]
        } else if k == 7 {
            prims_7[index]
        } else if k == 8 {
            prims_8[index]
        } else {
            panic!();
        }
    }
    // 1<<20, 1<<10, 1<<5, 16707839, 364914777, 1297642494, 493853244
    // 2,4,8,16,32,64,128
    // there's 2^7 possibilities for these. or given the first 3 there's 2^4.
    pub fn find_prims(k: u32) -> Vec<u32> {
        let zeta = 493853244;
        let mut prims = vec![0; 1 << k];
        for i in 0..(1 << k) {
            let mut acc = zeta;
            for j in 0..32 {
                let bit = (i << (32 - j - 1)) >> 32 - 1;
                acc = Self::exp(acc, 1 + bit * (1 << k - j));
            }
            prims[i as usize] = acc;
        }
        println!("{prims:?}");
        prims
    }
    pub fn get_prim_sets() {
        for k in 2..9 {
            println!("uint prims_{k}[] = {{");
            for i in 0..(1 << k - 2) {
                let zeta = Self::get_zeta(k, 2 * i);
                print!("{zeta},");
            }
            println!("\n}};");
        }
    }
    pub fn check_prims(k: usize, zeta: u32) {
        // check k'th prims.
        let mut ours = vec![0; 1 << k - 1];
        for i in 0..ours.len() {
            ours[i] = Self::get_zeta(k, i);
        }
        let mut correct = vec![0; 1 << k - 1];
        correct[0] = zeta;
        for i in 1..correct.len() {
            correct[i] = correct[i - 1].mulm(zeta, &P).mulm(zeta, &P);
        }
        ours.sort();
        correct.sort();
        for i in 0..ours.len() {
            assert_eq!(ours[i], correct[i]);
        }
    }
    pub fn find_prim(k: u32) {
        // let mut rng = rand::thread_rng();
        // let e = (P - 1) / k;
        // for _ in 0..100 {
        //     let x = rng.gen::<u32>();
        //     let rou = Self::exp(x, e);
        //     if Self::exp(rou, k / 2) == 1 {
        //         // println!("no: {rou}");
        //     } else {
        //         println!("yes: {}, {}", rou, rou.mulm(rou, &P));
        //     }
        // }
    }
    fn exp(x: u32, e: u32) -> u32 {
        let mut powers = [x; 32];
        for i in 1..32 {
            powers[i] = powers[i - 1].mulm(powers[i - 1], &P);
        }
        let mut acc = 1;
        for i in 0..32 {
            let bit = (e << (32 - i - 1)) >> 32 - 1;
            // assert!(bit == 0 || bit == 1);
            if bit == 1 {
                acc = acc.mulm(powers[i], &P);
            }
        }
        acc
    }
}

// fn coefs_to_arrays(arrays: &mut Vec<u32>, coefs: &Vec<u32>) {
//     for t in 0..T {
//         let array = &mut arrays[t * S * U..(t + 1) * S * U];
//         for s in 0..S {
//             for u in 0..U {
//                 array[s * U + u] = coefs[s * T * U + t * U + u];
//             }
//         }
//     }
// }
// pub fn decompose_within_threads(&mut self) {
//     for k in 0..LOG_S {
//         for i in 0..(1 << k) {
//             for t in 0..T {
//                 let range = t * S * U..(t + 1) * S * U;
//                 let array = &mut self.arrays[range];
//                 for s in 0..(S / (1 << (k + 1))) {
//                     for u in 0..U {
//                         let hi_index = (2 * i + 1) * (S / (1 << (k + 1))) * U + s * U + u;
//                         let zeta = ZETAS[(1 << k) - 1 + i];
//                         // Self::get_zeta(k + 2, 2 * i);
//                         let mult = array[hi_index].mulm(zeta, &P);
//                         let lo_index = (2 * i) * (S / (1 << (k + 1))) * U + s * U + u;
//                         array[hi_index] = array[lo_index].subm(mult, &P);
//                         array[lo_index] = array[lo_index].addm(mult, &P);
//                     }
//                 }
//             }
//         }
//     }
// }
// pub fn decompose_across_threads(&mut self) {
//     for l in 0..3 {
//         //LOG_T + LOG_U - LOG_ORD {
//         for t in 0..T / (1 << l + 1) {
//             for r in 0..(1 << l) {
//                 let lo_index = (2 * r) * T / (1 << l + 1) + t;
//                 let hi_index = (2 * r + 1) * T / (1 << l + 1) + t;
//                 for s in 0..S {
//                     let i = s * (1 << l) + r;
//                     let zeta = ZETAS[(1 << (LOG_S + l)) - 1 + i];
//                     // Self::get_zeta(Self::logS() + l + 2, 2 * i);
//                     for u in 0..U {
//                         let lo_coef = self.arrays[lo_index * S * U + s * U + u];
//                         let hi_coef = self.arrays[hi_index * S * U + s * U + u];
//                         let mult = hi_coef.mulm(zeta, &P);
//                         self.arrays[hi_index * S * U + s * U + u] = lo_coef.subm(mult, &P);
//                         self.arrays[lo_index * S * U + s * U + u] = lo_coef.addm(mult, &P);
//                     }
//                 }
//             }
//         }
//     }
// }
// pub fn rearrange(&mut self) {
//     // for l in (Self::logT() + Self::logU() - Self::logOrd())..(Self::logT() + Self::logU()) {
//     //     for t in 0..T / (1 << l + 1) {
//     //         for r in 0..(1 << l) {
//     //             let lo_index = (2 * r) * T / (1 << l + 1) + t;
//     //             let hi_index = (2 * r + 1) * T / (1 << l + 1) + t;
//     //             // println!("threads: {lo_index}, {hi_index}");
//     //             // let idx = Self::logT() - l - 1;
//     //             // let mask = 1 << idx;
//     //             // let sigma = tau ^ mask;
//     //             for s in 0..S {
//     //                 // let i = s * (1 << l) + r;
//     //                 // let zeta = Self::get_zeta(Self::logS() + l + 2, 2 * i);
//     //                 for u in 0..U {
//     //                     let lo_coef = self.arrays[(lo_index * S * U + s * U + u) as usize];
//     //                     let hi_coef = self.arrays[(hi_index * S * U + s * U + u) as usize];
//     //                     // let mult = hi_coef.mulm(zeta, &P);
//     //                     self.arrays[(hi_index * S * U + s * U + u) as usize] = lo_coef; //.subm(mult, &P);
//     //                     self.arrays[(lo_index * S * U + s * U + u) as usize] = hi_coef;
//     //                     //.addm(mult, &P);
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }
// }
// pub fn check(&self, k: usize, print: bool) -> Vec<u32> {
//     let d = D / (1 << k);
//     let mut correct_reduced_coefs = vec![0; D];
//     for i in 0..(1 << k) {
//         let zeta = Self::get_zeta(k + 1, i);
//         let result = self.reduce(d, zeta);
//         for j in 0..d {
//             correct_reduced_coefs[i * d + j] = result[j];
//         }
//     }
//     if print {
//         println!("\nCPU COEFS");
//         for i in 0..D {
//             println!("{i}: {}", correct_reduced_coefs[i]);
//         }
//     }
//     let mut correct_reduced_arrays = vec![0; D];
//     Ring::coefs_to_arrays(&mut correct_reduced_arrays, &correct_reduced_coefs);
//     for j in 0..D {
//         assert_eq![correct_reduced_arrays[j], self.arrays[j], "index: {j}"];
//     }
//     correct_reduced_coefs
// }
// pub fn print(&self) {
//     println!("\nCPU ARRAYS");
//     for t in 0..T {
//         println!("thread {t}");
//         for j in 0..S * U {
//             println!("{j}: {}", self.arrays[t * S * U + j]);
//         }
//     }
// }
// pub fn alpha_logS(l: u32, i: u32) -> u32 {
//     let mut acc = 0;
//     for z in 0..LOG_S {
//         let shifted = i >> l;
//         let bit = (shifted << (32 - z - 1)) >> 32 - 1;
//         acc = acc + bit * (1 << z);
//     }
//     acc
// }
// pub fn beta(l: u32, i: u32) -> u32 {
//     // // here i is like the output of alpha.
//     // // that means the bottom, least significant bit is i_k, the top most significant bit is i_1.
//     // // there are k bits. but we only depend on the last l coordinates, those are the least significant bits.
//     // // we'll go through the l least significant bits, starting with i think l-1.
//     // // just look at at l least significant bits.
//     // let mut acc = 0;
//     // for z in 0..l {
//     //     let bit = (i << (32 - z - 1)) >> 32 - 1;
//     //     acc = acc + bit * T / (1 << (l - z));

//     //     // most significant bit goes with
//     //     // l-1 -> 1
//     //     // l-2 -> 2
//     //     // l-3 -> 3
//     //     // l-l -> l
//     //     // sum_{z=1}^l i_{log(S)+z} (T / 2^z)
//     //     // sum_{z=l}^{1} i_{log(S)+z} (T / 2^z)
//     // }
//     // acc
//     0
// }
