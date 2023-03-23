use hal::*;

use super::params::*;

#[repr(C)]
struct Params {
    E: u32,
    F: u16,
    G: u16,
    X: u16,
    ORD: u16,
}

pub fn go(input_vals: &Vec<u8>, to_hide: usize) -> Vec<u32> {
    let mut infos = GPU::current_gpus();
    let info = infos.remove(0);
    let gpu = GPU::new(info);
    let queue = gpu.new_queue("queue".into(), 4);

    let shaders_url_str = "file:///Users/josephjohnston/saga/zpu/src/shaders/";
    let archive = gpu.new_archive(format!("{SHADER}_macos"), false, shaders_url_str);
    // let archive = gpu.load_archive("risc0".into(), shaders_url_str);
    let pipeline = archive.load_pipeline("go".into()).unwrap();
    drop(archive);

    let block_size = Size::new(T * G, 1, 1);
    let grid_size = Size::new(E, 1, 1);
    let mut input = gpu.new_buffer::<u8>("input".into(), E * F * G * D, true);
    for (i, x) in input.as_mut_slice().iter_mut().enumerate() {
        *x = input_vals[i] as u8;
    }
    let output = gpu.new_buffer::<u32>("output".into(), E * F * G * D, true);

    let mut timestamp_sampler = gpu.new_timestamp_sampler(5, false);
    let batch = queue.new_batch(false);

    let cpass = batch.new_compute_pass(Some(&mut timestamp_sampler));
    cpass.set_buffer(0, &input, 0);
    cpass.set_threadgroup_memory_length(1 << 16, 0); // T * E * 4
    cpass.set_buffer(1, &output, 0);

    let params = Params {
        E: E as u32,
        F: F as u16,
        G: G as u16,
        X: X as u16,
        ORD: ORD as u16,
    };
    cpass.set_bytes(
        2,
        &params as *const Params as *const std::ffi::c_void,
        1 * std::mem::size_of::<u32>() + 4 * std::mem::size_of::<u16>(),
    );
    // cpass.set_imageblock_size(1 << 5, 1);
    cpass.set_pipeline(&pipeline);
    cpass.dispatch(block_size, grid_size);
    cpass.end_encoding();

    println!("\nPRNG DONE!");
    batch.commit();
    batch.wait_until_completed();

    timestamp_sampler.get_timestamps();
    println!("GPU COEFS:");
    for (i, x) in output.as_slice().iter().enumerate() {
        if i % (1 << to_hide) == 0 {
            println!("{i}: {x}");
        }
    }

    output.as_slice().to_vec()
}

// for (ushort k = 0; k < ushort(log2(half(S))); k++)
// {
//     for (ushort i = 0; i < (1 << k); i++)
//     {
//         for (uint s = 0; s < S / (1 << (k + 1)); s++)
//         {
//             uint hi_index = (2 * i + 1) * (S / (1 << (k + 1))) + s;
//             uint mult = array[hi_index] * (1 << 20);
//             uint lo_index = (2 * i) * (S / (1 << (k + 1))) + s;
//             array[hi_index] = array[lo_index] - mult;
//             array[lo_index] = array[lo_index] + mult;
//         }
//     }
// }

// fn zeta(k: u32, index: u32) -> u32 {
//     let prims_1 = [(1 << 20), (1 << 30).mulm(1 << 30, &P)];
//     let prims_2 = [
//         1 << 10,
//         (1 << 30).mulm(1 << 20, &P),
//         1 << 30,
//         (1 << 30).mulm(1 << 30, &P).mulm(1 << 10, &P),
//     ];
//     let prims_3 = [
//         // 5, 45 => 10
//         1 << 5,
//         (1 << 30).mulm(1 << 15, &P),
//         // 25, 65 => 50
//         1 << 25,
//         (1 << 30).mulm(1 << 30, &P).mulm(1 << 5, &P),
//         // 15, 55 => 30
//         1 << 15,
//         (1 << 30).mulm(1 << 25, &P),
//         // 35, 75 => 79
//         (1 << 30).mulm(1 << 30, &P).mulm(1 << 15, &P),
//         (1 << 30).mulm(1 << 5, &P),
//     ];
//     let prims_4 = [
//         // 3743342369, 16707839, 134217856, // 2
//         // 4274061061, 534650848, 16716031, 4144037761, 4274061053,
//         // // 17
//         // 134217856, 4144037505, 4144037761, 134217600, 3743604513, 534650848, 3743342369,
//         // 534912992, 4274061053, 4194308, 4274061061, 4194300, 16716031, 4261539330, 16707839,
//         // 4261547522,
//         // new
//         16707839, 4261547522, 4261539330, 16716031, 4274061053, 4194308, 4274061061, 4194300,
//         534650848, 3743604513, 534912992, 3743342369, 4144037761, 134217600, 4144037505,
//         134217856,
//         //
//         // 4261547522, 16707839, 16716031, 4261539330, 4194308, 4274061053, 4194300, 4274061061,
//         // 3743604513, 534650848, 3743342369, 534912992, 134217600, 4144037761, 134217856,
//         // 4144037505,
//     ];
//     let prims_5 = [
//         364914777, 3913340584, 2274230434, 2004024927, 1464515241, 2813740120, 1441048032,
//         2837207329, 3120762142, 1157493219, 45032751, 4233222610, 196321259, 4081934102, 947271947,
//         3330983414, 4184386525, 93868836, 3065742370, 1212512991, 919248094, 3359007267,
//         2278185239, 2000070122, 171586511, 4106668850, 531848519, 3746406842, 1274452609,
//         3003802752, 3982137898, 296117463,
//     ];
//     if k == 1 {
//         prims_1[index as usize]
//     } else if k == 2 {
//         prims_2[index as usize]
//     } else if k == 3 {
//         prims_3[index as usize]
//     } else if k == 4 {
//         prims_4[index as usize]
//     } else if k == 5 {
//         prims_5[index as usize]
//     } else {
//         0
//     }
// }

// let numsA: [i32; 32] = [
//     -10, -4, 2, -23, -33, 38, 55, 38, 28, -64, -47, -44, 19, -36, 46, -47, 30, -28, -32, -61,
//     22, 29, -44, 21, 30, 62, 30, 58, 6, -64, 36, 18,
// ];
// let numsB: [i32; 32] = [
//     55, -63, -16, 1, 7, 19, 24, -9, -47, -1, -37, 28, -9, 57, -46, -41, 2, 15, -44, -49, -34,
//     16, -16, 13, 61, -2, -39, -8, 55, 42, -56, -33,
// ];
// let numsC: [i32; 32] = [
//     -23, 14, -30, -59, 21, 51, 29, -25, -29, -50, 35, 1, 62, -43, 42, -35, 26, 47, 41, -56,
//     -58, -4, -49, -18, -31, -13, -4, -41, 58, 45, -62, -4,
// ];
// let numsD: [i32; 32] = [
//     8, 10, -59, 38, 10, 13, 24, -7, -52, 28, 32, 50, -64, 33, 10, -55, 14, 42, -10, 47, -33,
//     -56, -42, 22, 54, -12, -19, 11, -28, -25, 16, 36,
// ];

// if i < 32 {
//     *x = numsA[i];
// } else if 32 <= i && i < 64 {
//     *x = numsB[i - 32];
// } else if 64 <= i && i < 96 {
//     *x = numsC[i - 64];
// } else if 96 <= i && i < 128 {
//     *x = numsD[i - 96];
// }

// compute correctly
// println!("calculating CPU version");
// let mut out = vec![0; D * G];
// for g in 0..G {
//     for t in 0..T {
//         let mut array: [u32; S] = [0; S];
//         // READING INPUT
//         for s in 0..S {
//             array[s] = input.as_slice()[g * D + s * (D / S) + t] as u32;
//         }
//         // DECOMPOSING WITHIN THREADS
//         for k in 0..((S as f32).log2() as u32) {
//             for i in 0..(1 << k) {
//                 for s in 0..S / (1 << (k + 1)) {
//                     for u in 0..U {
//                         let hi_index =
//                             ((2 * i + 1) * (S / (1 << (k + 1))) * U + s * U + u) as usize;
//                         // let mult = vals[hi_index].mulm(zeta(k + 1, 2 * i), &P);
//                         let mult = array[hi_index] * (1 << 20);
//                         let lo_index =
//                             ((2 * i) * (S / (1 << (k + 1))) * U + s * U + u) as usize;
//                         // array[hi_index] = array[lo_index].subm(mult, &P);
//                         // array[lo_index] = array[lo_index].addm(mult, &P);
//                         array[hi_index] = array[lo_index] - mult;
//                         array[lo_index] = array[lo_index] + mult;
//                         // let hi_index = (2 * i + 1) * (S / (1 << (k + 1))) + s;
//                         // let lo_index = (2 * i) * (S / (1 << (k + 1))) + s;
//                         // let mult = vals[hi_index] * (1 << 20);
//                         // vals[hi_index] = vals[lo_index] + mult;
//                         // vals[lo_index] = vals[lo_index] - mult;
//                     }
//                 }
//             }
//         }
//         // DECOMPOSING ACROSS THREADS
//         for l in 0..((T as f32).log2() as u32) {
//             for t in 0..T / (1 << l + 1) {
//                 for r in 0..(1 << l) {
//                     let lo_index = (2 * r) * T / (1 << l + 1) + t;
//                     let hi_index = (2 * r + 1) * T / (1 << l + 1) + t;
//                     // println!("threads: {lo_index}, {hi_index}");
//                     // let idx = Self::logT() - l - 1;
//                     // let mask = 1 << idx;
//                     // let sigma = tau ^ mask;
//                     for s in 0..S {
//                         let i = s * (1 << l) + r;
//                         let zeta = 1 << 20; //Self::zeta(Self::logS() + l + 2, 2 * i);
//                         for u in 0..U {
//                             let lo_coef = array[(lo_index * S * U + s * U + u) as usize];
//                             let hi_coef = array[(hi_index * S * U + s * U + u) as usize];
//                             let mult = hi_coef * zeta;
//                             array[(hi_index * S * U + s * U + u) as usize] =
//                                 lo_coef - mult;
//                             array[(lo_index * S * U + s * U + u) as usize] =
//                                 lo_coef + mult;
//                         }
//                     }
//                 }
//             }
//         }
//         // WRITING OUTPUT
//         for s in 0..S {
//             out[g * D + s * (D / S) + t] = vals[s];
//         }
//     }
// }
// // compute expected output
// println!("calculating correct");
// let mut correct = vec![0; D * G];
// for g in 0..G {
//     for j in 0..D {
//         correct[g * D + j] = input.as_slice()[g * D + j] as u32;
//     }
//     for k in 0..((S as f32).log2() as u32) {
//         for j in 0..(1 << k) {
//             for l in 0..D / (1 << (k + 1)) {
//                 let hi_index = g * D + j * (D / (1 << k)) + D / (1 << (k + 1)) + l;
//                 let lo_index = g * D + j * (D / (1 << k)) + l;
//                 // let mult = ref_in[hi_index].mulm(1 << 20, &P);
//                 // ref_out[hi_index] = ref_in[lo_index].addm(mult, &P);
//                 // ref_out[lo_index] = ref_in[lo_index].subm(mult, &P);
//                 let mult = correct[hi_index] * (1 << 20);
//                 correct[hi_index] = correct[lo_index] + mult;
//                 correct[lo_index] = correct[lo_index] - mult;
//                 // println!("hi: {}, lo: {}", correct[hi_index], correct[lo_index]);
//                 // println!(
//                 //     "hi_val: {}, lo_val: {}, mult: {}, lo: {}",
//                 //     ref_in[hi_index], ref_in[lo_index], mult, ref_out[lo_index]
//                 // )
//             }
//         }
//     }
// }
// // outputs
// println!("comparing outputs");
// for g in 0..G {
//     for j in 0..D {
//         println!("{}: {}", g * D + j, output[g * D + j],);
//         // println!("C: {}: {}", g * G + j, correct[g * G + j]);
//         // assert_eq!(correct[g * D + j], out[g * D + j]);
//         // assert_eq!(output.as_slice()[g * D + j], out[g * D + j]);
//     }
// }
