mod library_writer;
pub use library_writer::LibraryWriter;

use super::params::*;

fn get_index(e: &str, f: &str, g: &str, s: &str, t: &str) -> String {
    format!(
        "{e} * {} + {f} * {} + {g} * {} + {s} * {} + {t} * {}",
        F * G * S * T,
        G * S * T,
        S * T,
        T,
        1,
    )
}

pub fn gen() {
    let library_name = "unrolled";
    let mut w = LibraryWriter::init(
        format!("src/shaders/{library_name}.metal"),
        vec![format!("#include \"arithmetic.h\"")],
    );
    w.init_kernel(
        format!("go"),
        vec![
            format!("device uchar *input"),
            format!("threadgroup uint *shared"),
            format!("device uint *output"),
            format!("uint e [[threadgroup_position_in_grid]]"),
            format!("ushort w_global [[simdgroup_index_in_threadgroup]]"),
            format!("ushort t_local [[thread_index_in_simdgroup]]"),
        ],
    );
    w.lines(vec![
        format!("uint array[{S}];"),
        format!("uint acc[{S}] = {{0}};"),
        format!("ushort g = w_global >> {LOG_W};"),
        format!("ushort w = w_global & ({W} - 1);"),
        format!("ushort tau = w * {X} + t_local;"),
        format!(
            "uint global_read_index_prefix = {};",
            get_index("e", "0", "g", "0", "tau")
        ),
    ]);
    w.empty_line()
        .begin_for(format!("ushort f = 0"), format!("f < {F}"), format!("f++"));
    {
        read_code(&mut w, false);
        decompose_with_chain(&mut w, false, false, false, false);
        collect_into_threads_code(&mut w, false, false);
        schoolbook_multiplication_code(&mut w, false, false, false);
        // karatsuba_multiplication_code(&mut w, true, true, false, true);
        // dual_karatsuba_multiplication_code(&mut w, false, false, false);

        // read_code(&mut w, true);
        // decompose_with_chain(&mut w, true, true, true, true);
        // collect_into_threads_code(&mut w, true, true);
        // schoolbook_multiplication_code(&mut w, true, true, true);
        // // karatsuba_multiplication_code(&mut w, true);
        // dual_karatsuba_multiplication_code(&mut w, true);
    }
    w.end_for();
    write_output_code(&mut w, false, false);
    w.flush().unwrap();
}

fn read_code(w: &mut LibraryWriter, unroll: bool) {
    w.comment(format!("READ INPUT"));
    let write_integrand = |w: &mut LibraryWriter, s: &str| {
        w.line(format!(
            "array[{s}] = uint(input[global_read_index_prefix + {}]);",
            get_index("0", "0", "0", s, "0")
        ));
    };
    if unroll {
        for s in 0..S {
            w.begin_scope();
            {
                write_integrand(w, &format!("{s}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
        {
            write_integrand(w, "s");
        }
        w.end_for();
    }
    w.line(format!(
        "global_read_index_prefix += {};",
        get_index("0", "1", "0", "0", "0")
    ));
}

fn decompose_with_chain(
    w: &mut LibraryWriter,
    unroll_j: bool,
    unroll_across_warps: bool,
    unroll_across_threads: bool,
    unroll_within_threads: bool,
) {
    w.empty_line().comment(format!("DECOMPOSE WITH CHAIN"));
    let write_j_integrand = |w: &mut LibraryWriter, j: &str| {
        w.lines(vec![
            format!("ushort log_power_S_to_j = {j} * {LOG_S};"),
            format!("ushort log_Dj = ({LOG_S} + {LOG_T}) - log_power_S_to_j;"),
            format!("ushort r = tau >> log_Dj;"),
            format!("ushort Tj = {T} >> log_power_S_to_j;"),
            format!("ushort log_Tj = {LOG_T} - log_power_S_to_j;"),
            format!("ushort i = (tau >> log_Tj) & ({S} - 1);"),
            format!("ushort t = tau & (Tj - 1);"),
        ]);
        w.empty_line()
            .line(format!("if ({LOG_X} < log_Dj)"))
            .comment(format!("TRANSPOSE ACROSS WARPS"))
            .begin_scope();
        {
            transpose_across_warps_code(w, unroll_across_warps);
        }
        w.end_scope()
            .line(format!("else"))
            .comment(format!("TRANSPOSE ACROSS THREADS"))
            .begin_scope();
        {
            transpose_across_threads_code(w, unroll_across_threads);
        }
        w.end_scope();
        decompose_within_threads_code_j(w, j, unroll_j);
    };
    w.begin_scope();
    {
        decompose_within_threads_code_0(w, unroll_within_threads);
        if unroll_j {
            for j in 1..J + 1 {
                w.begin_scope();
                {
                    write_j_integrand(w, &format!("{j}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort j = 1"), format!("j <= {J}"), format!("j++"));
            {
                write_j_integrand(w, "j");
            }
            w.end_for();
        }
    }
    w.end_scope();
}

fn transpose_across_warps_code(w: &mut LibraryWriter, unroll: bool) {
    let write_read_integrand = |w: &mut LibraryWriter, s: &str| {
        if !unroll {
            w.line(format!("if ({s} == i)"))
                .begin_scope()
                .line(format!("continue;"))
                .end_scope();
        }
        w.line(format!(
            "ushort index = g * {D} + (r * {S} + i) * Dj + {s} * Tj + t;"
        ));
        w.line(format!("array[{s}] = shared[index];"));
    };
    let write_write_integrand = |w: &mut LibraryWriter, s: &str| {
        if !unroll {
            w.line(format!("if ({s} == i)"))
                .begin_scope()
                .line(format!("continue;"))
                .end_scope();
        }
        w.line(format!(
            "ushort index = g * {D} + (r * {S} + {s}) * Dj + i * Tj + t;"
        ));
        w.line(format!("shared[index] = array[{s}];"));
    };
    w.line(format!("ushort Dj = 1 << log_Dj;"));
    if unroll {
        for s in 0..S {
            w.begin_scope();
            {
                write_write_integrand(w, &format!("{s}"));
            }
            w.end_scope();
        }
        w.line(format!(
            "threadgroup_barrier(metal::mem_flags::mem_threadgroup);"
        ));
        for s in 0..S {
            w.begin_scope();
            {
                write_read_integrand(w, &format!("{s}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
        {
            write_write_integrand(w, &format!("s"));
        }
        w.end_for();
        w.line(format!(
            "threadgroup_barrier(metal::mem_flags::mem_threadgroup);"
        ));
        w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
        {
            write_read_integrand(w, &format!("s"));
        }
        w.end_for();
    }
}

fn transpose_across_threads_code(w: &mut LibraryWriter, unroll: bool) {
    let write_integrand = |w: &mut LibraryWriter, s: &str| {
        w.lines(vec![
            format!("ushort index = {s} ^ i;"),
            format!("array[index] = metal::simd_shuffle_xor(array[index], {s} * Tj);"),
        ]);
    };
    if unroll {
        for s in 0..S {
            w.begin_scope();
            {
                write_integrand(w, &format!("{s}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
        {
            write_integrand(w, "s");
        }
        w.end_for();
    }
}

fn decompose_within_threads_code_0(w: &mut LibraryWriter, unroll_k: bool) {
    w.comment(format!("DECOMPOSE WITHIN THREADS 0"));
    let write_s_integrand = |w: &mut LibraryWriter, s: &str| {
        w.lines(vec![
            format!("ushort hi_index = hi_index_prefix + {s};"),
            format!("uint mult = mul(array[hi_index], zeta);"),
            format!("ushort lo_index = lo_index_prefix + {s};"),
            format!("array[hi_index] = sub(array[lo_index], mult);"),
            format!("array[lo_index] = add(array[lo_index], mult);"),
        ]);
    };
    if unroll_k {
        for k in 0..LOG_S {
            let s_bound = S / (1 << (k + 1));
            for i in 0..(1 << k) {
                w.begin_scope();
                {
                    let lo_index_prefix = (2 * i) * s_bound;
                    let hi_index_prefix = lo_index_prefix + s_bound;
                    let zeta = ZETAS[(1 << k) - 1 + i];
                    w.lines(vec![
                        format!("uint zeta = {zeta};"),
                        format!("ushort lo_index_prefix = {lo_index_prefix};"),
                        format!("ushort hi_index_prefix = {hi_index_prefix};"),
                    ]);
                    for s in 0..s_bound {
                        w.begin_scope();
                        {
                            write_s_integrand(w, &format!("{s}"));
                        }
                        w.end_scope();
                    }
                }
                w.end_scope();
            }
        }
    } else {
        w.begin_for(
            format!("ushort k = 0"),
            format!("k < {LOG_S}"),
            format!("k++"),
        );
        {
            w.line(format!("ushort s_bound = 1 << ({LOG_S} - (k + 1));"));
            w.begin_for(
                format!("ushort i = 0"),
                format!("i < (1 << k)"),
                format!("i++"),
            );
            {
                w.lines(vec![
                    format!("uint zeta = zetas((1 << k) - 1 + i);"),
                    format!("ushort lo_index_prefix = (2 * i) * s_bound;"),
                    format!("ushort hi_index_prefix = lo_index_prefix + s_bound;"),
                ]);
                w.begin_for(
                    format!("ushort s = 0"),
                    format!("s < s_bound"),
                    format!("s++"),
                );
                {
                    write_s_integrand(w, "s");
                }
                w.end_for();
            }
            w.end_for();
        }
        w.end_for();
    }
}

fn decompose_within_threads_code_j(w: &mut LibraryWriter, j: &str, unroll: bool) {
    w.empty_line()
        .comment(format!("DECOMPOSE WITHIN THREADS j > 0"));
    let write_integrand = |w: &mut LibraryWriter, s: &str| {
        w.lines(vec![
            format!("ushort hi_index = hi_index_prefix + {s};"),
            format!("uint mult = mul(array[hi_index], zeta);"),
            format!("ushort lo_index = lo_index_prefix + {s};"),
            format!("array[hi_index] = sub(array[lo_index], mult);"),
            format!("array[lo_index] = add(array[lo_index], mult);"),
        ]);
    };
    w.line(format!("ushort r_new = r * {S} + i;"));
    if unroll {
        let k_bound = if j.parse::<usize>().unwrap() < J {
            LOG_S
        } else {
            K
        };
        for k in 0..k_bound {
            let s_bound = S / (1 << (k + 1));
            for i_new in 0..(1 << k) {
                w.begin_scope();
                {
                    let lo_index_prefix = (2 * i_new) * s_bound;
                    let hi_index_prefix = lo_index_prefix + s_bound;
                    w.lines(vec![
                        format!("ushort component_index = r_new * (1 << {k}) + {i_new};"),
                        format!(
                            "uint zeta = zetas((1 << ({j} * {LOG_S} + {k})) - 1 + component_index);"
                        ),
                        format!("ushort lo_index_prefix = {lo_index_prefix};"),
                        format!("ushort hi_index_prefix = {hi_index_prefix};"),
                    ]);
                    for s in 0..s_bound {
                        w.begin_scope();
                        {
                            write_integrand(w, &format!("{s}"));
                        }
                        w.end_scope();
                    }
                }
                w.end_scope();
            }
        }
    } else {
        w.line(format!("ushort k_bound = {j} < {J} ? {LOG_S} : {K};"));
        w.begin_for(
            format!("ushort k = 0"),
            format!("k < k_bound"),
            format!("k++"),
        );
        {
            w.line(format!("ushort s_bound = 1 << ({LOG_S} - (k + 1));"));
            w.begin_for(
                format!("ushort i_new = 0"),
                format!("i_new < (1 << k)"),
                format!("i_new++"),
            );
            {
                w.lines(vec![
                    format!("ushort component_index = r_new * (1 << k) + i_new;"),
                    format!("uint zeta = zetas((1 << ({j} * {LOG_S} + k)) - 1 + component_index);"),
                    format!("ushort lo_index_prefix = (2 * i_new) * s_bound;"),
                    format!("ushort hi_index_prefix = lo_index_prefix + s_bound;"),
                ]);
                w.begin_for(
                    format!("ushort s = 0"),
                    format!("s < s_bound"),
                    format!("s++"),
                );
                {
                    write_integrand(w, "s");
                }
                w.end_for();
            }
            w.end_for();
        }
        w.end_for();
    }
}

fn collect_into_threads_code(w: &mut LibraryWriter, unroll_mask: bool, unroll_v: bool) {
    w.empty_line()
        .comment(format!("COLLECT COMPONENTS INTO THREADS"));
    let write_v_integrand = |w: &mut LibraryWriter, mask: &str, v: &str| {
        w.lines(vec![
            format!("ushort index = scaled_other + {v};"),
            format!("array[index] = metal::simd_shuffle_xor(array[index], {mask});"),
        ]);
    };
    let write_mask_integrand = |w: &mut LibraryWriter, mask: &str| {
        w.lines(vec![
            format!("ushort other = t ^ {mask};"),
            format!("ushort scaled_other = other * ({S} / {T_J});"),
        ]);
        if unroll_v {
            for v in 0..S / T_J {
                w.begin_scope();
                {
                    write_v_integrand(w, mask, &format!("{v}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(
                format!("ushort v = 0"),
                format!("v < {S} / {T_J}"),
                format!("v++"),
            );
            {
                write_v_integrand(w, mask, "v");
            }
            w.end_for();
        }
    };
    w.begin_scope();
    {
        w.line(format!("ushort t = tau & ({T_J} - 1);"));
        if unroll_mask {
            for mask in 0..T_J {
                w.begin_scope();
                {
                    write_mask_integrand(w, &format!("{mask}"))
                }
                w.end_for();
            }
        } else {
            w.begin_for(
                format!("ushort mask = 0"),
                format!("mask < {T_J}"),
                format!("mask++"),
            );
            {
                write_mask_integrand(w, "mask")
            }
            w.end_for();
        }
    }
    w.end_scope();
}

fn schoolbook_multiplication_code(
    w: &mut LibraryWriter,
    unroll_u: bool,
    unroll_outer: bool,
    unroll_inner: bool,
) {
    w.empty_line().comment(format!("SCHOOLBOOK MULTIPLICATION"));
    let s_bound = S / (1 << K);
    let write_inner_integrand = |w: &mut LibraryWriter, u: &str, s2: &str, t2: &str| {
        w.lines(vec![
            format!("ushort array_index = {t2} * ({S} / {T_J}) + {u} * {s_bound} + {s2};"),
            format!("ushort delta = (coef_index + ({s2} * {T_J} + {t2})) % ({T_J} * {s_bound});"),
            format!("ushort acc_index = {u} * ({T_J} * {s_bound}) + delta;"),
            format!("acc[acc_index] = add(acc[acc_index], mul(array[array_index], minors[coef_index]));"),
        ]);
    };
    let write_outer_integrand = |w: &mut LibraryWriter, u: &str, s1: &str, t1: &str| {
        w.line(format!("ushort coef_index = {s1} * {T_J} + {t1};"))
            .line(format!("if (coef_index > 0)"))
            .begin_scope();
        {
            w.lines(vec![
                format!("ushort new_bottom_index = {t1} == 0 ? ({u} + 1) * ({S} / (1 << {K})) - {s1} : ({T_J} - {t1}) * ({S} / {T_J}) + ({u} + 1) * {s_bound} - ({s1} + 1);"),
                format!("array[new_bottom_index] = mul(array[new_bottom_index], zeta);")
            ]);
        }
        w.end_scope();
        if unroll_inner {
            for s2 in 0..s_bound {
                for t2 in 0..T_J {
                    w.begin_scope();
                    {
                        write_inner_integrand(w, u, &format!("{s2}"), &format!("{t2}"));
                    }
                    w.end_scope();
                }
            }
        } else {
            w.begin_for(
                format!("ushort s2 = 0"),
                format!("s2 < {s_bound}"),
                format!("s2++"),
            );
            {
                w.begin_for(
                    format!("ushort t2 = 0"),
                    format!("t2 < {T_J}"),
                    format!("t2++"),
                );
                {
                    write_inner_integrand(w, u, "s2", "t2");
                }
                w.end_for();
            }
            w.end_for();
        }
    };
    let write_u_integrand = |w: &mut LibraryWriter, u: &str| {
        w.lines(vec![
            format!("ushort component_index = tau * ((1 << {K}) / {T_J}) + {u};"),
            format!(
                "uint zeta = zetas((1 << ({J} * {LOG_S} + {K} - 1)) - 1 + (component_index >> 1));"
            ),
            format!("zeta = component_index & 1 ? sub(0, zeta) : zeta;"),
        ]);
        if unroll_outer {
            for s1 in 0..s_bound {
                for t1 in 0..T_J {
                    w.begin_scope();
                    {
                        write_outer_integrand(w, u, &format!("{s1}"), &format!("{t1}"));
                    }
                    w.end_scope();
                }
            }
        } else {
            w.begin_for(
                format!("ushort s1 = 0"),
                format!("s1 < {s_bound}"),
                format!("s1++"),
            );
            {
                w.begin_for(
                    format!("ushort t1 = 0"),
                    format!("t1 < {T_J}"),
                    format!("t1++"),
                );
                {
                    write_outer_integrand(w, u, "s1", "t1");
                }
                w.end_for();
            }
            w.end_for();
        }
    };
    w.line(format!(
        "uint minors[{T_J} * {S} / (1 << {K})] = {{3614796953, 1208427060, 1889015752, 3198863462, 3614796953, 1208427060, 1889015752,3198863462}};"
    ));
    if unroll_u {
        for u in 0..(1 << K) / T_J {
            w.begin_scope();
            {
                write_u_integrand(w, &format!("{u}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(
            format!("ushort u = 0"),
            format!("u < (1 << {K}) / {T_J}"),
            format!("u++"),
        );
        {
            write_u_integrand(w, "u");
        }
        w.end_for();
    }
}

fn karatsuba_multiplication_code(
    w: &mut LibraryWriter,
    unroll_addition: bool,
    unroll_u: bool,
    unroll_mult: bool,
    unroll_reduction: bool,
) {
    w.empty_line().comment(format!("KARATSUBA MULTIPLICATION"));
    fn karatsuba(
        w: &mut LibraryWriter,
        left: &str,
        left_index: &str,
        middle: &str,
        right: &str,
        right_index: &str,
        big_theta: usize,
        unroll: bool,
    ) {
        w.comment(format!("Theta = {big_theta}"));
        let middle_index = &format!("({big_theta} - 2)");
        // 1: base case
        if big_theta == 1 {
            let var_0 = format!("{left}[{left_index}]");
            let var_1 = format!("{right}[{right_index}]");
            w.line(format!("{var_0} = mul({var_0}, {var_1});"));
            return;
        }
        // 3: middle sums
        {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_2 = |theta: &str| format!("{left}[{left_index} + {big_theta} / 2 + {theta}]");
            let var_3 =
                |theta: &str| format!("{middle}[{middle_index} + {big_theta} / 2 + {theta}]");
            let var_4 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            let var_5 = |theta: &str| format!("{right}[{right_index} + {big_theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        format!(
                            "{} = add({}, {});",
                            var_0(theta_str),
                            var_1(theta_str),
                            var_2(theta_str)
                        ),
                        format!(
                            "{} = add({}, {});",
                            var_3(theta_str),
                            var_4(theta_str),
                            var_5(theta_str)
                        ),
                    ]);
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2"),
                    format!("theta++"),
                );
                {
                    w.lines(vec![
                        format!(
                            "{} = add({}, {});",
                            var_0("theta"),
                            var_1("theta"),
                            var_2("theta")
                        ),
                        format!(
                            "{} = add({}, {});",
                            var_3("theta"),
                            var_4("theta"),
                            var_5("theta")
                        ),
                    ]);
                }
                w.end_for();
            }
        }
        // 4: recurse
        w.begin_scope();
        {
            // bottoms
            karatsuba(
                w,
                left,
                left_index,
                middle,
                right,
                right_index,
                big_theta / 2,
                unroll,
            );
        }
        w.end_scope();
        w.begin_scope();
        {
            // middles
            karatsuba(
                w,
                middle,
                middle_index,
                middle,
                middle,
                &format!("{middle_index} + {big_theta} / 2"),
                big_theta / 2,
                unroll,
            );
        }
        w.end_for();
        w.begin_scope();
        {
            // tops
            karatsuba(
                w,
                left,
                &format!("{left_index} + {big_theta} / 2"),
                middle,
                right,
                &format!("{right_index} + {big_theta} / 2"),
                big_theta / 2,
                unroll,
            );
        }
        w.end_scope();
        // 5: clarification
        // 6: middle differences
        {
            let var_0 = format!("{middle}[{middle_index} + {big_theta} / 2 - 1]");
            let var_1 = format!("{left}[{left_index} + {big_theta} / 2 - 1]");
            w.line(format!("{var_0} = sub({var_0}, {var_1});"));
        }
        if 2 < big_theta {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_2 =
                |theta: &str| format!("{middle}[{middle_index} + {big_theta} / 2 + {theta}]");
            let var_3 = |theta: &str| format!("{right}[{right_index} + {big_theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 - 1 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        format!(
                            "{} = sub({}, {});",
                            var_0(theta_str),
                            var_0(theta_str),
                            var_1(theta_str)
                        ),
                        format!(
                            "{} = sub({}, {});",
                            var_2(theta_str),
                            var_2(theta_str),
                            var_3(theta_str)
                        ),
                    ]);
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2 - 1"),
                    format!("theta++"),
                );
                {
                    w.lines(vec![
                        format!(
                            "{} = sub({}, {});",
                            var_0("theta"),
                            var_0("theta"),
                            var_1("theta")
                        ),
                        format!(
                            "{} = sub({}, {});",
                            var_2("theta"),
                            var_2("theta"),
                            var_3("theta")
                        ),
                    ]);
                }
                w.end_for();
            }
        }
        // 7: outliers
        {
            let var_0 = format!("{right}[{right_index} + {big_theta} / 2 - 1]");
            let var_1 = format!("{left}[{left_index} + {big_theta} - 1]");
            let var_2 = format!("{middle}[{middle_index} + {big_theta} / 2 - 1]");
            w.lines(vec![
                format!("{var_0} = {var_1};"),
                format!("{var_1} = sub({var_2}, {var_0});"),
            ]);
        }
        // 8: symmetric difference
        if 2 < big_theta {
            let var_0 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {big_theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 - 1 {
                    let theta_str = &format!("{theta}");
                    w.line(format!(
                        "{} = sub({}, {});",
                        var_0(theta_str),
                        var_1(theta_str),
                        var_0(theta_str)
                    ));
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2 - 1"),
                    format!("theta++"),
                );
                {
                    w.line(format!(
                        "{} = sub({}, {});",
                        var_0("theta"),
                        var_1("theta"),
                        var_0("theta")
                    ));
                }
                w.end_for();
            }
        }
        // 9: finall upper left and lower right
        if 2 < big_theta {
            let var_0 = |theta: &str| format!("{left}[{left_index} + {big_theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_2 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            let var_3 =
                |theta: &str| format!("{middle}[{middle_index} + {big_theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 - 1 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        format!(
                            "{} = sub({}, {});",
                            var_0(theta_str),
                            var_1(theta_str),
                            var_2(theta_str)
                        ),
                        format!(
                            "{} = add({}, {});",
                            var_2(theta_str),
                            var_3(theta_str),
                            var_2(theta_str)
                        ),
                    ]);
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2 - 1"),
                    format!("theta++"),
                );
                {
                    w.lines(vec![
                        format!(
                            "{} = sub({}, {});",
                            var_0("theta"),
                            var_1("theta"),
                            var_2("theta")
                        ),
                        format!(
                            "{} = add({}, {});",
                            var_2("theta"),
                            var_3("theta"),
                            var_2("theta")
                        ),
                    ]);
                }
                w.end_for();
            }
        }
    }
    let add_to_acc = |w: &mut LibraryWriter| {
        if unroll_addition {
            for mask in 0..T_J {
                for v in 0..S / T_J {
                    let new_index = v * T_J + mask;
                    let old_index = mask * S / T_J + v;
                    w.line(format!(
                        "acc[{new_index}] = add(acc[{new_index}], array[{old_index}]);"
                    ));
                }
            }
        } else {
            w.begin_for(
                format!("ushort mask = 0"),
                format!("mask < {T_J}"),
                format!("mask++"),
            );
            {
                w.begin_for(
                    format!("ushort v = 0"),
                    format!("v < {S} / {T_J}"),
                    format!("v++"),
                );
                {
                    w.lines(vec![
                        format!("ushort new_index = v * {T_J} + mask;"),
                        format!("ushort old_index = mask * ({S} / {T_J}) + v;"),
                        format!("acc[new_index] = add(acc[new_index], array[old_index]);"),
                    ]);
                }
                w.end_for();
            }
            w.end_for();
        }
    };
    let write_reduction_integrand = |w: &mut LibraryWriter, theta: &str| {
        w.lines(vec![
            format!("uint mult = mul(minors[{theta}], zeta);"),
            format!("acc[acc_offset + {theta}] = add(acc[acc_offset + {theta}], mult);"),
        ]);
    };
    let big_theta = T_J * S / (1 << K);
    let write_u_integrand = |w: &mut LibraryWriter, u: &str| {
        w.line(format!(
            "ushort acc_offset = {u} * ({T_J} * {S} / (1 << {K}));"
        ));
        karatsuba(
            w,
            "acc",
            "acc_offset",
            "middle",
            "minors",
            "0",
            big_theta,
            unroll_mult,
        );
        // finite field reduction
        w.lines(vec![
            format!("ushort component_index = tau * ((1 << {K}) / {T_J}) + {u};"),
            // format!("uint zeta = get_zeta({J} * {LOG_S} + {K} + 1, component_index);"),
            format!(
                "uint zeta = zetas((1 << ({J} * {LOG_S} + {K} - 1)) - 1 + (component_index >> 1));"
            ),
            format!("zeta = component_index & 1 ? sub(0, zeta) : zeta;"),
        ]);
        if unroll_reduction {
            for theta in 0..big_theta - 1 {
                w.begin_scope();
                {
                    write_reduction_integrand(w, &format!("{theta}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(
                format!("ushort theta = 0"),
                format!("theta < {big_theta} - 1"),
                format!("theta++"),
            );
            {
                write_reduction_integrand(w, "theta");
            }
            w.end_for();
        }
    };
    add_to_acc(w);
    w.line(format!(
        "uint minors[{}] = {{3614796953, 1208427060, 1889015752, 3198863462, 3614796953, 1208427060, 1889015752, 3198863462}};",
        big_theta
    ));
    w.line(format!("uint middle[{}] = {{0}};", big_theta * 2 - 1));
    if unroll_u {
        for u in 0..(1 << K) / T_J {
            w.begin_scope();
            {
                write_u_integrand(w, &format!("{u}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(
            format!("ushort u = 0"),
            format!("u < (1 << {K}) / {T_J}"),
            format!("u++"),
        );
        {
            write_u_integrand(w, "u");
        }
        w.end_for();
    }
}

fn dual_karatsuba_multiplication_code(
    w: &mut LibraryWriter,
    unroll_addition: bool,
    unroll_u: bool,
    unroll_mult: bool,
) {
    w.empty_line()
        .comment(format!("DUAL KARATSUBA MULTIPLICATION"));
    fn dual_karatsuba(
        w: &mut LibraryWriter,
        left: &str,
        left_index: &str,
        middle: &str,
        right: &str,
        right_index: &str,
        big_theta: usize,
        preserve: bool,
        unroll: bool,
    ) {
        w.comment(format!("Theta = {big_theta}"));
        let middle_index = &format!("({big_theta} / 2 - 1)");
        // 1: base case
        if big_theta == 1 {
            let var_0 = format!("{left}[{left_index}]");
            let var_1 = format!("{right}[{right_index}]");
            w.line(format!("{var_0} = mul({var_0}, {var_1});"));
            return;
        }
        // 2: space for middle already allocated
        // 3: copy upper half of left to middle
        {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {big_theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.line(format!("{} = {};", var_0(theta_str), var_1(theta_str)));
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2"),
                    format!("theta++"),
                );
                {
                    w.line(format!("{} = {};", var_0("theta"), var_1("theta")));
                }
                w.end_for();
            }
        }
        // 4: recursively multiply tops
        w.begin_scope();
        {
            dual_karatsuba(
                w,
                middle,
                middle_index,
                middle,
                right,
                &format!("{right_index} + {big_theta} / 2"),
                big_theta / 2,
                true,
                unroll,
            );
        }
        w.end_scope();
        // 5: add left and right lower to upper halves
        {
            let var_0 = |theta: &str| format!("{left}[{left_index} + {big_theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_2 = |theta: &str| format!("{right}[{right_index} + {big_theta} / 2 + {theta}]");
            let var_3 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        format!(
                            "{} = add({}, {});",
                            var_0(theta_str),
                            var_0(theta_str),
                            var_1(theta_str)
                        ),
                        format!(
                            "{} = add({}, {});",
                            var_2(theta_str),
                            var_2(theta_str),
                            var_3(theta_str)
                        ),
                    ]);
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2"),
                    format!("theta++"),
                );
                {
                    w.lines(vec![
                        format!(
                            "{} = add({}, {});",
                            var_0("theta"),
                            var_0("theta"),
                            var_1("theta")
                        ),
                        format!(
                            "{} = add({}, {});",
                            var_2("theta"),
                            var_2("theta"),
                            var_3("theta")
                        ),
                    ]);
                }
                w.end_for();
            }
        }
        // 6: recursively multiply bottoms and middles
        w.begin_scope();
        {
            // bottoms
            dual_karatsuba(
                w,
                left,
                left_index,
                middle,
                right,
                right_index,
                big_theta / 2,
                preserve,
                unroll,
            );
        }
        w.end_scope();
        w.begin_scope();
        {
            // middles
            dual_karatsuba(
                w,
                left,
                &format!("{left_index} + {big_theta} / 2"),
                middle,
                right,
                &format!("{right_index} + {big_theta} / 2"),
                big_theta / 2,
                preserve,
                unroll,
            );
        }
        w.end_scope();
        // 7: subtract bottom and top from middle
        {
            let var_0 = |theta: &str| format!("{left}[{left_index} + {big_theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_2 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        format!(
                            "{} = sub({}, {});",
                            var_0(theta_str),
                            var_0(theta_str),
                            var_1(theta_str)
                        ),
                        format!(
                            "{} = sub({}, {});",
                            var_0(theta_str),
                            var_0(theta_str),
                            var_2(theta_str)
                        ),
                    ]);
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2"),
                    format!("theta++"),
                );
                {
                    w.lines(vec![
                        format!(
                            "{} = sub({}, {});",
                            var_0("theta"),
                            var_0("theta"),
                            var_1("theta")
                        ),
                        format!(
                            "{} = sub({}, {});",
                            var_0("theta"),
                            var_0("theta"),
                            var_2("theta")
                        ),
                    ]);
                }
                w.end_for();
            }
        }
        // 8: finite field reduction
        {
            w.line(
            format!("{left}[{left_index}] = add({left}[{left_index}], mul(zeta, {middle}[{middle_index} + {big_theta} / 2 - 1]));")
        );
            let var_0 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_1 = |power_basis_middle_theta: &str| {
                format!("{middle}[{middle_index} + {power_basis_middle_theta}")
            };
            if unroll {
                for theta in 1..big_theta / 2 {
                    let power_basis_theta = rho(theta, big_theta / 2);
                    let power_basis_middle_theta = rho(power_basis_theta - 1, big_theta / 2);
                    let theta_str = &format!("{theta}");
                    w.line(format!(
                        "{} = add({}, {}]);",
                        var_0(theta_str),
                        var_0(theta_str),
                        var_1(&format!("{power_basis_middle_theta}"))
                    ));
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 1"),
                    format!("theta < {big_theta} / 2"),
                    format!("theta++"),
                );
                {
                    w.lines(vec![
                        format!("ushort power_basis_theta = rho(theta, {big_theta} / 2);"),
                        format!("ushort power_basis_middle_theta = rho(power_basis_theta - 1, {big_theta} / 2);"),
                        format!(
                            "{} = add({}, {}]);",
                            var_0("theta"),
                            var_0("theta"),
                            var_1("power_basis_middle_theta")
                        )
                    ]);
                }
                w.end_for();
            }
        }
        // 9: preserving right
        if preserve {
            let var_0 = |theta: &str| format!("{right}[{right_index} + {big_theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            if unroll {
                for theta in 0..big_theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.line(format!(
                        "{} = sub({}, {});",
                        var_0(theta_str),
                        var_0(theta_str),
                        var_1(theta_str)
                    ));
                }
            } else {
                w.begin_for(
                    format!("ushort theta = 0"),
                    format!("theta < {big_theta} / 2"),
                    format!("theta++"),
                );
                {
                    w.line(format!(
                        "{} = sub({}, {});",
                        var_0("theta"),
                        var_0("theta"),
                        var_1("theta")
                    ));
                }
                w.end_for();
            }
        }
    }
    let add_to_acc = |w: &mut LibraryWriter| {
        if unroll_addition {
            let s_bound = S / (1 << K);
            let u_bound = (1 << K) / T_J;
            for mask in 0..T_J {
                for s in 0..s_bound {
                    for u in 0..u_bound {
                        let source_index = mask * S / T_J + u * s_bound + s;
                        let dest_index =
                            u * (T_J * s_bound) + rho(mask, T_J) * s_bound + rho(s, s_bound);
                        w.line(format!(
                            "acc[{dest_index}] = add(acc[{dest_index}], array[{source_index}]);"
                        ));
                    }
                }
            }
        } else {
            w.lines(vec![
                format!("ushort s_bound = {S} / (1 << {K});"),
                format!("ushort u_bound = (1 << {K}) / {T_J};"),
            ]);
            w.begin_for(
                format!("ushort mask = 0"),
                format!("mask < {T_J}"),
                format!("mask++"),
            );
            {
                w.begin_for(
                    format!("ushort s = 0"),
                    format!("s < s_bound"),
                    format!("s++"),
                );
                {
                    w.begin_for(
                        format!("ushort u = 0"),
                        format!("u < u_bound"),
                        format!("u++"),
                    );
                    {
                        w.lines(vec![
                            format!("ushort source_index = mask * ({S} / {T_J}) + u * s_bound + s;"),
                            format!("ushort dest_index = u * ({T_J} * s_bound) + rho(mask, {T_J}) * s_bound + rho(s, s_bound);"),
                            format!("acc[dest_index] = add(acc[dest_index], array[source_index]);"),
                        ]);
                    }
                    w.end_for();
                }
                w.end_for();
            }
            w.end_for();
        }
    };
    let big_theta = T_J * S / (1 << K);
    let write_u_integrand = |w: &mut LibraryWriter, u: &str| {
        // zeta
        w.lines(vec![
            format!("ushort component_index = tau * ((1 << {K}) / {T_J}) + {u};"),
            // format!("uint zeta = get_zeta({J} * {LOG_S} + {K} + 1, component_index);"),
            format!(
                "uint zeta = zetas((1 << ({J} * {LOG_S} + {K} - 1)) - 1 + (component_index >> 1));"
            ),
            format!("zeta = component_index & 1 ? sub(0, zeta) : zeta;"),
        ]);
        dual_karatsuba(
            w,
            "acc",
            &format!("{u} * ({T_J} * {S} / (1 << {K}))"),
            "middle",
            "minors",
            "0",
            big_theta,
            true,
            unroll_mult,
        );
    };
    add_to_acc(w);
    w.line(format!(
        "uint minors[{}] = {{3614796953, 3614796953, 1889015752, 1889015752, 1208427060, 1208427060, 3198863462, 3198863462}};",
        // 8: {{3614796953, 3614796953, 1889015752, 1889015752, 1208427060, 1208427060, 3198863462, 3198863462}}
        // 4: {{3614796953, 1889015752, 1208427060, 3198863462}}
        // "uint minors[{}] = {{3614796953, 1889015752, 1208427060, 3198863462}};",
        big_theta
    ));
    w.line(format!("uint middle[{}] = {{0}};", big_theta - 1));
    if unroll_u {
        for u in 0..(1 << K) / T_J {
            w.begin_scope();
            {
                write_u_integrand(w, &format!("{u}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(
            format!("ushort u = 0"),
            format!("u < (1 << {K}) / {T_J}"),
            format!("u++"),
        );
        {
            write_u_integrand(w, "u");
        }
        w.end_for();
    }
}

fn write_output_code(w: &mut LibraryWriter, dual_basis: bool, unroll: bool) {
    w.empty_line().comment(format!("WRITE OUTPUT"));
    w.lines(vec![
        format!("ushort gamma = t_local >> ({LOG_X} - {LOG_S});"),
        format!("ushort delta = t_local & ({X} / {S} - 1);"),
        format!(
            "ushort global_write_index_prefix = ({}) + w * {X} * {S} + gamma * {X} + delta * {S};",
            get_index("e", "0", "g", "0", "0")
        ),
    ]);
    let write_s_integrand = |w: &mut LibraryWriter, s: &str| {
        if X < S {
            // TODO when needed
        } else {
            if dual_basis {
                // LOG(T_J) = LOG(T/S^J) = LOG_T - J*LOG_S
                let log_component_size = (LOG_T - J * LOG_S) + LOG_S - K;
                let component_size = 1 << log_component_size;
                w.lines(vec![
                    format!("ushort index = (gamma + {s}) & ({S} - 1);"),
                    format!("ushort u = index >> {log_component_size};"),
                    format!("ushort coef_index = index & ({component_size} - 1);"),
                    format!(
                        "output[global_write_index_prefix + index] = acc[u * {component_size} + rho(coef_index, {component_size})];"
                    ),
                ]);
            } else {
                w.lines(vec![
                    format!("ushort index = (gamma + {s}) & ({S} - 1);"),
                    format!("output[global_write_index_prefix + index] = acc[index];"),
                ]);
            }
        }
    };
    if unroll {
        for s in 0..S {
            w.begin_scope();
            {
                write_s_integrand(w, &format!("{s}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
        {
            write_s_integrand(w, "s");
        }
        w.end_for();
    }
}

fn rho(int: usize, range: usize) -> usize {
    if range == 1 {
        match int {
            0 => 0,
            _ => panic!(),
        }
    } else if range == 2 {
        match int {
            0 => 0,
            1 => 1,
            _ => panic!(),
        }
    } else if range == 4 {
        match int {
            0 => 0,
            1 => 2,
            2 => 1,
            3 => 3,
            _ => panic!(),
        }
    } else if range == 8 {
        match int {
            0 => 0,
            1 => 4,
            2 => 2,
            3 => 6,
            4 => 1,
            5 => 5,
            6 => 3,
            7 => 7,
            _ => panic!(),
        }
    } else if range == 16 {
        match int {
            0 => 0,
            1 => 8,
            2 => 4,
            3 => 12,
            4 => 2,
            5 => 10,
            6 => 6,
            7 => 14,
            8 => 1,
            9 => 9,
            10 => 5,
            11 => 13,
            12 => 3,
            13 => 11,
            14 => 7,
            15 => 15,
            _ => panic!(),
        }
    } else {
        panic!("QUERYING RHO OUT OF RANGE");
    }
}
