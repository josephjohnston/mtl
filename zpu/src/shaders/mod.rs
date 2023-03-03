pub mod params {
    pub const ORD: usize = 8;
    pub const SD: usize = 1;
    pub const TD: usize = 5;
    pub const UD: usize = 1;
    pub const VD: usize = 0;
    pub const WD: usize = 4;
    pub const BD: usize = 0;
    pub const S: usize = 1 << SD;
    pub const T: usize = 1 << TD;
    pub const U: usize = 1 << UD;
    pub const D: usize = S * T * U;
    pub const V: usize = 1 << VD;
    pub const W: usize = 1 << WD;
    pub const B: usize = 1 << BD;
}

use params::*;
use std::fs::File;
use std::io::Write;

pub fn logS() -> usize {
    (S as f32).log2() as usize
}
pub fn logT() -> usize {
    (T as f32).log2() as usize
}
pub fn logU() -> usize {
    (U as f32).log2() as usize
}
pub fn logOrd() -> usize {
    (ORD as f32).log2() as usize
}

pub fn gen() {
    let vec_suffix = if U > 1 { format!("{U}") } else { String::new() };
    let mut writer = Writer::init(
        "go".into(),
        vec![
            format!("device uchar{vec_suffix} *input"),
            format!("device uint{vec_suffix} *output"),
            "ushort t [[thread_index_in_simdgroup]]".into(),
            "ushort w [[simdgroup_index_in_threadgroup]]".into(),
            "ushort b [[threadgroup_position_in_grid]]".into(),
        ],
    );
    // karat(
    //     &mut writer,
    //     1,
    //     "A".into(),
    //     0,
    //     "B".into(),
    //     0,
    //     "C".into(),
    //     0,
    //     "D".into(),
    //     0,
    //     0,
    // );
    read_code(&mut writer);
    decompose_within_threads_code(&mut writer);
    // decompose_across_threads_code(&mut writer);
    write_code(&mut writer);
    writer.flush().unwrap();
}

// how to gen this code?
struct Writer {
    file: File,
    indent: usize,
    string: String,
}
impl Writer {
    pub fn init(name: String, inputs: Vec<String>) -> Self {
        let file = File::options()
            .truncate(true)
            .write(true)
            .open("src/shaders/unrolled.metal")
            .unwrap();
        let mut myself = Self {
            file,
            indent: 0,
            string: String::with_capacity(1 << 10),
        };
        myself
            .line(format!("#include \"fp.h\""))
            .empty_line()
            .line(format!("kernel void {name}("))
            .indent += 1;
        for i in 0..inputs.len() - 1 {
            myself.line(format!("{},", inputs[i]));
        }
        myself.line(format!("{})", inputs[inputs.len() - 1])).indent -= 1;
        myself.line(format!("{{")).indent += 1;
        myself
    }
    pub fn flush(&mut self) -> std::io::Result<usize> {
        self.indent -= 1;
        self.line(format!("}}"));
        self.file.write(self.string.as_bytes())
    }
    pub fn lines(&mut self, lines: Vec<String>) -> &mut Self {
        for (_, line) in lines.into_iter().enumerate() {
            self.string += &format!("\n{}", " ".repeat(self.indent * 4));
            self.string += &line;
        }
        self
    }
    pub fn line(&mut self, line: String) -> &mut Self {
        self.lines(vec![line])
    }
    pub fn empty_line(&mut self) -> &mut Self {
        self.lines(vec!["".into()])
    }
    pub fn start_block(&mut self) -> &mut Self {
        self.line(format!("{{")).indent += 1;
        self
    }
    pub fn end_block(&mut self) -> &mut Self {
        self.indent -= 1;
        self.line(format!("}}"))
    }
}

fn read_code(w: &mut Writer) {
    let vec_suffix = if U > 1 { format!("{U}") } else { String::new() };
    w.lines(vec![
        format!("uint{vec_suffix} array[{}];", S),
        format!(
            "uint in_global_index_prefix = b * {} + w * {} + t * {};",
            W * S * T * 1,
            S * T * 1,
            1
        ),
    ]);
    for s in 0..S {
        let global_index = format!("in_global_index_prefix + {}", s * T * 1);
        let local_index = s;
        w.line(format!(
            "array[{local_index}] = uint{vec_suffix}(input[{global_index}]);"
        ));
        // for u in 0..U {
        //     let global_index = format!("in_global_index_prefix + {}", s * T * U + u);
        //     let local_index = s * U + u;
        //     w.line(format!("array[{local_index}] = input[{global_index}];"));
        // }
    }
}

fn write_code(w: &mut Writer) {
    w.line(format!(
        "uint out_global_index_prefix = b * {} + w * {} + t * {};",
        W * S * T * 1,
        S * T * 1,
        1,
    ));
    for s in 0..S {
        let global_index = format!("out_global_index_prefix + {}", s * T * 1);
        let local_index = s;
        w.line(format!("output[{global_index}] = array[{local_index}];"));
        // for u in 0..U {
        //     let global_index = format!("out_global_index_prefix + {}", s * T * U + u);
        //     let local_index = s * U + u;
        //     w.line(format!("output[{global_index}] = array[{local_index}];"));
        // }
    }
}
fn multiply_code(w: &mut Writer) {
    w.line(format!("uint aggregate[{S}];"));
    for s in 0..S {
        w.line(format!(
            "uint zeta = get_zeta({}, {});",
            logS() + logT() + logU() - logOrd() - 1 + 2,
            s / 2
        ));
        for u0 in 0..U {
            if u0 > 0 {
                w.line(format!(
                    "array[{}] = mul(array[{}], zeta);",
                    s * U + (U - u0),
                    s * U + (U - u0)
                ));
            }
            w.line(format!("state = add(mul(state, mult_val), add_val);"));
            for u1 in 0..U {
                if u0 == 0 {
                    w.line(format!(
                        "aggregate[{}] = mul(array[{}], state);",
                        s * U + u1,
                        s * U + u1
                    ));
                } else {
                    w.line(format!(
                        "aggregate[{}] = add(aggregate[{}], mul(array[{}], state));",
                        s * U + u1,
                        s * U + u1,
                        (s * U + u1 + (ORD - u0)) % ORD
                    ));
                }
            }
        }
        for u in 0..U {
            w.line(format!(
                "output[global_index_prefix + {}] = aggregate[{}];",
                s * T * U + u,
                s * U + u
            ));
        }
    }
}
// fn karat(
//     w: &mut Writer,
//     k: usize,
//     A: String, //Vec<usize>,
//     off_A: usize,
//     B: String, //Vec<usize>,
//     off_B: usize,
//     C: String, //Vec<usize>,
//     off_C: usize,
//     D: String, //Vec<usize>,
//     off_D: usize,
//     zero_instance: usize,
// ) {
//     if k == 0 {
//         if zero_instance == 0 {
//             w.line(format!(
//                 "D[{off_D}] = D[{off_D}] + (A[{off_A}] + B[{off_B}]) * C[{off_C}]"
//             ));
//         } else if zero_instance == 1 {
//             w.line(format!(
//                 "D[{off_D}] = D[{off_D}] + (A[{off_A}] + B[{off_B}]) * C[{off_C}];"
//             ));
//             // w.line(format!("D[{}] = 0;", off_D + 1));
//         } else if zero_instance == 2 {
//             w.line(format!(
//                 "D[{off_D}] = (A[{off_A}] + B[{off_B}]) * C[{off_C}];"
//             ));
//         }
//         return;
//     }
//     for i in 0..k {
//         w.line(format!(
//             "{D}[{}] = {D}[{}] + {D}[{}];",
//             off_D + k + i,
//             off_D + k + i,
//             off_D + i
//         ));
//         w.line(format!(
//             "{D}[{}] = {A}[{}] + {A}[{}] + {B}[{}] + {B}[{}];",
//             off_D + (3 * k - 1) + i,
//             off_A + i,
//             off_A + k + i,
//             off_B + i,
//             off_B + k + i
//         ));
//     }
//     karat(
//         w,
//         k / 2,
//         format!("{C}"),
//         off_C,
//         format!("{C}"),
//         off_C + k,
//         format!("{D}"),
//         off_D + (3 * k - 1),
//         format!("{D}"),
//         off_D + k,
//         0,
//     );
//     for i in 0..k {
//         if k == 1 {
//             w.line(format!(
//                 "{D}[{}] = {D}[{}];",
//                 off_D + (3 * k - 1) + i,
//                 off_D + k + i
//             ));
//         } else {
//             w.line(format!(
//                 "{D}[{}] = {D}[{}] + {D}[{}];",
//                 off_D + (3 * k - 1) + i,
//                 off_D + k + i,
//                 off_D + 2 * k + i
//             ));
//         }
//     }
//     karat(
//         w,
//         k / 2,
//         format!("{A}"),
//         off_A,
//         format!("{B}"),
//         off_B,
//         format!("{C}"),
//         off_C,
//         format!("{D}"),
//         off_D,
//         1,
//     );
//     for i in 0..k {
//         if i < k - 1 {
//             w.line(format!(
//                 "{D}[{}] = {D}[{}] - {D}[{}]",
//                 off_D + 2 * k + i,
//                 off_D + 2 * k + i,
//                 off_D + k + i
//             ));
//         }
//         w.line(format!(
//             "{D}[{}] = {D}[{}] - {D}[{}];",
//             off_D + k + i,
//             off_D + (3 * k - 1) + i,
//             off_D + i
//         ));
//     }
//     karat(
//         w,
//         k / 2,
//         format!("{A}"),
//         off_A + k,
//         format!("{B}"),
//         off_B + k,
//         format!("{C}"),
//         off_C + k,
//         format!("{D}"),
//         off_D + 2 * k,
//         2,
//     );
//     for i in 0..k {
//         w.line(format!(
//             "{D}[{}] = {D}[{}] - {D}[{}];",
//             off_D + k + i,
//             off_D + k + i,
//             off_D + 2 * k + i
//         ));
//         if i < k - 1 {
//             w.line(format!(
//                 "{D}[{}] = {D}[{}] - {D}[{}];",
//                 off_D + 2 * k + i,
//                 off_D + 2 * k + i,
//                 off_D + 3 * k + i
//             ));
//         }
//     }
// }
fn decompose_within_threads_code(w: &mut Writer) {
    // let k_loop_start = format!(
    //     "for (ushort k = 0; k < {}; k++) {{",
    //     (S as f32).log2() as u32
    // );
    // code += &k_loop_start;
    for k in 0..(S as f32).log2() as usize {
        for i in 0..(1 << k) {
            for s in 0..(S / (1 << (k + 1))) {
                for u in 0..U {
                    let zeta = match k + 2 {
                        2 => prims_2[i],
                        3 => prims_3[i],
                        4 => prims_4[i],
                        5 => prims_5[i],
                        6 => prims_6[i],
                        7 => prims_7[i],
                        8 => prims_8[i],
                        _ => panic!(),
                    };
                    let hi_index = (2 * i + 1) * (S / (1 << (k + 1))) * U + s * U + u;
                    let lo_index = (2 * i) * (S / (1 << (k + 1))) * U + s * U + u;
                    w.start_block()
                        .lines(vec![
                            format!("uint mult = mul(array[{hi_index}],{zeta});"),
                            format!("array[{hi_index}] = sub(array[{lo_index}],mult);"),
                            format!("array[{lo_index}] = add(array[{lo_index}],mult);"),
                        ])
                        .end_block();
                    //                     format!(
                    //                         "{{
                    // uint mult = mul(array[{hi_index}],{zeta});
                    // array[{hi_index}] = sub(array[{lo_index}],mult);
                    // array[{lo_index}] = add(array[{lo_index}],mult);
                    // }}
                    // "
                    //                     ));
                }
            }
        }
    }
    // let k_loop_end = format!("}}");
    // code += &k_loop_end;
    // println!("{code}");
}

// fn decompose_across_threads_code(f: &mut File) {
//     let mut code = String::with_capacity(1 << 10);
//     let logT = (T as f32).log2() as u32;
//     let logS = (S as f32).log2() as u32;
//     code += &format!(
//         "
//     ushort tau = t;
// "
//     );
//     for l in 0..6 - logS {
//         let k = logS + l + 2;
//         let r_shift = logT - l;
//         let idx = logT - l - 1;
//         let mask = 1 << idx;
//         code += &format!(
//             "
// {{
//     ushort sigma = tau ^ {mask};
//     bool upper = tau > sigma;
//     ushort r = tau >> ({r_shift});
// "
//         );
//         for s in 0..S {
//             let i_prefix = s * (1 << l);
//             code += &format!(
//                 "
// {{
//     ushort i = {i_prefix} + r;
//     uint mult = upper ? get_zeta({k}, i) : 1;
// "
//             );
//             for u in 0..U {
//                 let index = s * U + u;
//                 code += &format!(
//                     "
// {{
//     uint tau_coef = mul(array[{index}],mult);
//     uint sigma_coef = simd_shuffle_xor(tau_coef, {mask});
//     array[{index}] = upper ? sub(sigma_coef,tau_coef) : add(tau_coef,sigma_coef);
// }}
//                 "
//                 );
//             }
//             code += &format!(
//                 "
// }}
//         "
//             );
//         }
//         code += &format!(
//             "
// }}
//         "
//         );
//     }
//     println!("{code}");
// }

const prims_2: [u32; 1] = [1048576];
const prims_3: [u32; 2] = [1024, 1073741824];
const prims_4: [u32; 4] = [32, 33554432, 32768, 4144559881];
const prims_5: [u32; 8] = [
    16707839, 4261539330, 4274061053, 4274061061, 534650848, 534912992, 4144037761, 4144037505,
];
const prims_6: [u32; 16] = [
    364914777, 2274230434, 1464515241, 1441048032, 3120762142, 45032751, 196321259, 947271947,
    4184386525, 3065742370, 919248094, 2278185239, 171586511, 531848519, 1274452609, 3982137898,
];
const prims_7: [u32; 32] = [
    1297642494, 3327754660, 2526751946, 2129504484, 3020261559, 3810020456, 3847465774, 3970313073,
    2689265513, 2766251085, 1388567172, 3842193303, 2954927500, 3786866165, 3159035588, 2626659467,
    2504948723, 1488462141, 3151931493, 2392531113, 3830654479, 1816320888, 1128492723, 3708275820,
    3081630698, 3779079003, 2234052728, 1762621666, 3037601520, 786573619, 1139377988, 4065946328,
];
const prims_8: [u32; 64] = [
    493853244, 2630285104, 871589258, 2389324427, 2968537725, 2882271469, 2221324090, 3728040527,
    1792685315, 4069366704, 338212691, 10783282, 2529644974, 2406181663, 2011959971, 3933190337,
    1122335902, 562582394, 1578728461, 1480102279, 889615164, 2589549385, 302463957, 819753580,
    1816837538, 2239014032, 599443123, 394504728, 1081892113, 2521481523, 210614787, 2069158492,
    2122591668, 3775120428, 1819528072, 180144644, 1486373247, 1670676750, 529152400, 3265551773,
    206566095, 3541041349, 1934204752, 1889168591, 3720435503, 1998977010, 2331859679, 2078683782,
    1026655235, 2800282348, 1058032482, 1155858166, 3909251897, 2761418424, 4043927916, 1373075368,
    2126899470, 276736331, 1013149118, 3965176830, 1805271112, 1462002270, 3979203491, 3886952625,
];

// SCHOOLBOOK
// // uint2 seed = seeds[w * T + t];
// uint mult_val = 2091658123;
// uint add_val = 1523138830;
// uint state = 1;
// uint aggregate[S];
// for (ushort s = 0; s < S; s++)
// {
//     uint zeta = get_zeta(logS + logT + logU - logORD - 1 + 2, s / 2);
//     for (ushort u0 = 0; u0 < U; u0++)
//     {
//         if (u0 > 0)
//         {
//             array[s * U + (U - u0)] = mul(array[s * U + (U - u0)], zeta);
//         }
//         state = add(mul(state, mult_val), add_val);
//         for (ushort u1 = 0; u1 < U; u1++)
//         {
//             if (u0 == 0)
//             {
//                 aggregate[s * U + u1] = mul(array[s * U + u1], state);
//             }
//             else
//             {
//                 aggregate[s * U + u1] = add(aggregate[s * U + u1], mul(array[(s * U + u1 + (ORD - u0)) % ORD], state));
//             }
//         }
//     }
//     for (ushort u = 0; u < U; u++)
//     {
//         output[global_index_prefix + s * T * U + u] = aggregate[s * U + u];
//     }
// }

// C code for Karat

// #include <stdio.h>

// void karat(
//     int k,
//     int A[], int off_A,
//     int B[], int off_B,
//     int C[], int off_C,
//     int D[], int off_D,
//     int zero_instance)
// {
//     if (k == 0)
//     {
//         if(zero_instance == 0) {
//             D[off_D] = D[off_D] + (A[off_A] + B[off_B]) * C[off_C];
//         } else if(zero_instance == 1) {
//             D[off_D] = D[off_D] + (A[off_A] + B[off_B]) * C[off_C];
//             D[off_D+1] = 0;
//         } else if(zero_instance == 2) {
//             D[off_D] = (A[off_A] + B[off_B]) * C[off_C];
//         }
//         return;
//     }
//     for (int i = 0; i < k; i++)
//     {
//         // 1
//         D[off_D + k + i] = D[off_D + k + i] + D[off_D + i];
//         // 2
//         D[off_D + (3 * k - 1) + i] = A[off_A + i] + A[off_A + k + i] + B[off_B + i] + B[off_B + k + i];
//     }
//     karat(k/2, C, off_C, C, off_C + k, D, off_D + (3 * k - 1), D, off_D + k, 0);
//     for (int j = 0; j < k; j++)
//     {
//         if(k==1) {
//             D[off_D + (3 * k - 1) + j] = D[off_D + k + j];
//         } else {
//             D[off_D + (3 * k - 1) + j] = D[off_D + k + j] + D[off_D + 2 * k + j];
//         }
//     }
//     karat(k / 2, A, off_A, B, off_B, C, off_C, D, off_D, 1);
//     for (int i = 0; i < k; i++)
//     {
//         if (i < k - 1)
//         {
//             D[off_D + 2 * k + i] = D[off_D + 2 * k + i] - D[off_D + k + i];
//         }
//         D[off_D + k + i] = D[off_D + (3 * k - 1) + i] - D[off_D + i];
//     }
//     karat(k / 2, A, off_A + k, B, off_B + k, C, off_C + k, D, off_D + 2 * k, 2);
//     for (int i = 0; i < k; i++)
//     {
//         D[off_D + k + i] = D[off_D + k + i] - D[off_D + 2 * k + i];
//         if (i < k - 1)
//         {
//             D[off_D + 2 * k + i] = D[off_D + 2 * k + i] - D[off_D + 3 * k + i];
//         }
//     }
// }

// int main()
// {
//     int A[] = {-1, 10,1,1};
//     int B[] = {3, 4,1,1};
//     int C[] = {20, 31,1,1};
//     int D[] = {-2, -3, 1, 1, 0, 0,0,0};
//     // karat(2 / 2, A, 2, B, 2, C, 2, D, 4, 2);
//     karat(2, A, 0, B, 0, C, 0, D, 0, 0);
//     for(int loop = 0; loop < 8; loop++)
//       printf("%d ", D[loop]);

//     return 0;
// }
