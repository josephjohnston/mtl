use super::params::*;

fn get_index(e: &str, f: &str, g: &str, s: &str, t: &str, u: &str) -> String {
    // format!("(((({e} * {F} + {f}) * {G} + {g}) * {S} + {s}) * {T} + {t}) * {U} + {u};")
    let x = NUMS[9];
    format!(
        "{e} * {} + {f} * {} + {g} * {} + {s} * {} + {t} * {} + {u} * {}",
        F * G * S * T * U,
        G * S * T * U,
        S * T * U,
        T * U,
        U,
        1,
    )
}

pub fn gen() {
    // let vec_suffix = if U > 1 { format!("{U}") } else { String::new() };
    let mut w = Writer::init(
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

    let global_read_index_prefix = get_index("e", "0", "g", "0", "tau", "0");
    w.lines(vec![
        format!("uint array[{}];", S * U),
        format!("ushort w = w_global & ((1 << {LOG_W}) - 1);"),
        format!("ushort tau = w * 32 + t_local;"),
        format!("ushort g = w_global / {W};"),
        // format!("uint acc[{}] = {{0}};", S * U),
        // format!("uint mult_val = 2091658123;"),
        // format!("uint add_val = 1523138830;"),
        // "uint state = 1;"),
        format!("uint global_read_index_prefix = {global_read_index_prefix};"),
    ]);
    w.empty_line();
    w.begin_for(format!("ushort f = 0"), format!("f < {F}"), format!("f++"));
    {
        read_code(&mut w, false, false);
        // decompose_with_chain(&mut w, true, true, true, true);
        decompose_with_chain(&mut w, false, false, false, false);
        collect_into_threads_code(&mut w, false, false);
        // schoolbook_multiplication_code(&mut w);
        // karatsuba_multiplication_code(&mut w, true);
        dual_karatsuba_multiplication_code(&mut w, true);
        write_output_code(&mut w, false);

        // read_code(&mut w, true, true);
        // decompose_within_threads_code(&mut w, true, true);
        // decompose_across_warps_code(&mut w, true, true, true);
        // decompose_across_threads_code(&mut w, true, true, true);

        // read_code(&mut w, false, false);
        // decompose_within_threads_code_0(&mut w, false, false);
        // decompose_across_warps_code(&mut w, false, false, false);
        // decompose_across_threads_code(&mut w, false, false, false);

        // write_code(&mut w, "f", false, false);
    }
    w.end_for();
    // write_code(&mut w, false, false);
    w.flush().unwrap();
}

fn read_code(w: &mut Writer, unroll_s: bool, unroll_u: bool) {
    let write_u_integrand = |w: &mut Writer, s: &str, u: &str| {
        w.lines(vec![
            format!(
                "uint global_read_index = global_read_index_prefix + {};",
                get_index("0", "0", "0", s, "0", u)
            ),
            format!("array[{s} * {U} + {u}] = uint(input[global_read_index]);"),
        ]);
    };
    let write_s_integrand = |w: &mut Writer, s: &str| {
        if unroll_u {
            for u in 0..U {
                w.begin_scope();
                {
                    write_u_integrand(w, s, &format!("{u}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort u = 0"), format!("u < {U}"), format!("u++"));
            {
                write_u_integrand(w, s, "u");
            }
            w.end_for();
        }
    };

    w.empty_line().comment(format!("READ INPUT"));
    if unroll_s {
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
    w.line(format!(
        "global_read_index_prefix += {};",
        get_index("0", "1", "0", "0", "0", "0")
    ));
}

fn write_code(w: &mut Writer, f: &str, unroll_s: bool, unroll_u: bool) {
    let write_u_integrand = |w: &mut Writer, s: &str, u: &str| {
        w.lines(vec![
            format!(
                "uint global_write_index = global_index_prefix + {};",
                get_index("0", f, "0", s, "0", u)
            ),
            format!("output[global_write_index] = array[{s} * {U} + {u}];"),
        ]);
    };
    let write_s_integrand = |w: &mut Writer, s: &str| {
        if unroll_u {
            for u in 0..U {
                w.begin_scope();
                {
                    write_u_integrand(w, s, &format!("{u}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort u = 0"), format!("u < {U}"), format!("u++"));
            {
                write_u_integrand(w, s, "u");
            }
            w.end_for();
        }
    };

    w.empty_line().comment(format!("WRITE OUTPUT"));
    // .line(format!(
    //     "uint global_index_prefix = 0;" // get_index("e", "0", "g", "0", "tau", "0")
    // ));
    if unroll_s {
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

fn transpose_across_warps_code(w: &mut Writer, unroll: bool) {
    let write_write_integrand = |w: &mut Writer, s: &str| {
        // w.line(format!("if ({s} == i)"))
        //     .begin_scope()
        //     .line(format!("continue;"))
        //     .end_scope();
        w.line(format!(
            "ushort index = g * {D} + (r * {S} + {s}) * Dj + i * Tj + t;"
        ));
        // w.line(format!("ushort index = g * {D} + {r} * {Dj} * {S} + ({i_iter} * {S} + {s}) * {Tj} + {t};"));
        w.line(format!("shared[index] = array[{s}];"));
    };
    let write_read_integrand = |w: &mut Writer, s: &str| {
        // w.line(format!("if ({s} == i)"))
        //     .begin_scope()
        //     .line(format!("continue;"))
        //     .end_scope();
        w.line(format!(
            "ushort index = g * {D} + (r * {S} + i) * Dj + {s} * Tj + t;"
        ));
        w.line(format!("array[{s}] = shared[index];"));
    };

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

fn transpose_across_threads_code(w: &mut Writer, unroll: bool) {
    let write_integrand = |w: &mut Writer, s: &str| {
        w.lines(vec![
            format!("ushort mask = {s} * Tj;"),
            format!("ushort index = {s} ^ i;"),
            format!("array[index] = metal::simd_shuffle_xor(array[index], mask);"),
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

fn decompose_with_chain(
    w: &mut Writer,
    unroll_j: bool,
    unroll_across_warps: bool,
    unroll_across_threads: bool,
    unroll_within_threads: bool,
) {
    w.empty_line().comment(format!("DECOMPOSE WITH CHAIN"));
    let write_j_integrand = |w: &mut Writer, j: &str| {
        w.lines(vec![
            format!("Dj /= {S};"),
            format!("Tj /= {S};"),
            // format!("r = tau / Dj;"),
            format!("i = (tau / Tj) & ({S} - 1);"),
            format!("t = tau & (Tj - 1);"),
        ]);
        w.empty_line()
            .line(format!("if ({X} < Dj)"))
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
        decompose_within_threads_code_j(w, j, unroll_within_threads);
        w.empty_line().line(format!("r = tau / Tj;"));
    };
    w.lines(vec![
        format!("ushort Dj = {D};"),
        format!("ushort Tj = {T};"),
        format!("ushort r = 0;"),
        format!("ushort i = 0;"),
        format!("ushort t = tau;"),
    ]);
    decompose_within_threads_code_0(w, unroll_within_threads, true);
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

fn write_output_code(w: &mut Writer, unroll: bool) {
    w.empty_line().comment(format!("WRITE OUTPUT"));
    let write_integrand = |w: &mut Writer, s: &str| {
        w.lines(vec![
            // format!("uint index = global_write_index_prefix + r * Dj + {s} * Tj + tau_lower;"),
            format!("uint index = (r * Tj + tau_lower) * {S} + s;"),
            format!("output[index] = array[{s}];"),
        ]);
    };
    w.lines(vec![
        format!(
            "uint global_write_index_prefix = {};",
            get_index("e", "0", "g", "0", "0", "0")
        ),
        // format!("r = tau / Tj;"),
        format!("ushort tau_lower = tau & (Tj - 1);"),
    ]);
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

fn decompose_within_threads_code_0(w: &mut Writer, unroll_k: bool, unroll_u: bool) {
    let write_u_integrand = |w: &mut Writer, s: &str, u: &str| {
        w.lines(vec![
            format!("ushort hi_index = (hi_index_prefix + {s}) * {U} + {u};"),
            format!("uint mult = mul(array[hi_index], zeta);"),
            format!("ushort lo_index = (lo_index_prefix + {s}) * {U} + {u};"),
            format!("array[hi_index] = sub(array[lo_index], mult);"),
            format!("array[lo_index] = add(array[lo_index], mult);"),
        ]);
    };
    let write_s_integrand = |w: &mut Writer, s: &str| {
        if unroll_u {
            for u in 0..U {
                w.begin_scope();
                {
                    write_u_integrand(w, s, &format!("{u}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort u = 0"), format!("u < {U}"), format!("u++"));
            {
                write_u_integrand(w, s, "u");
            }
            w.end_for();
        }
    };

    w.empty_line()
        .comment(format!("DECOMPOSE WITHIN THREADS 0"));
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
                        // zetas((1 << k) - 1 + {i})"),
                        format!("ushort lo_index_prefix = {lo_index_prefix};"),
                        format!("ushort hi_index_prefix = {hi_index_prefix};"),
                    ]);
                    for s in 0..s_bound {
                        write_s_integrand(w, &format!("{s}"));
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
            w.line(format!("ushort s_bound = {S} / (1 << (k + 1));"));
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

fn decompose_within_threads_code_j(w: &mut Writer, j: &str, unroll: bool) {
    w.empty_line()
        .comment(format!("DECOMPOSE WITHIN THREADS j"));
    let write_integrand = |w: &mut Writer, s: &str| {
        w.lines(vec![
            format!("ushort hi_index = hi_index_prefix + {s};"),
            format!("uint mult = mul(array[hi_index], zeta);"),
            format!("ushort lo_index = lo_index_prefix + {s};"),
            format!("array[hi_index] = sub(array[lo_index], mult);"),
            format!("array[lo_index] = add(array[lo_index], mult);"),
        ]);
    };
    w.lines(vec![
        format!("ushort r_new = r * {S} + i;"),
        format!("ushort k_bound = j < {J} ? {LOG_S} : {K};"),
    ]);
    if unroll {
        // for k in 0..k_bound {
        //     let s_bound = S / (1 << (k + 1));
        //     for i_new in 0..(1 << k) {
        //         w.begin_scope();
        //         {
        //             let lo_index_prefix = (2 * i_new) * s_bound;
        //             let hi_index_prefix = lo_index_prefix + s_bound;
        //             w.lines(vec![
        //                 format!("ushort component_index = r_new * (1 << {k}) + {i_new};"),
        //                 format!(
        //                     "uint zeta = zetas((1 << ({j} * {LOG_S} + {k})) - 1 + component_index);"
        //                 ),
        //                 format!("ushort lo_index_prefix = {lo_index_prefix};"),
        //                 format!("ushort hi_index_prefix = {hi_index_prefix};"),
        //             ]);
        //             for s in 0..s_bound {
        //                 w.begin_scope();
        //                 {
        //                     write_integrand(w, &format!("{s}"));
        //                 }
        //                 w.end_scope();
        //             }
        //         }
        //         w.end_scope();
        //     }
        // }
    } else {
        w.begin_for(
            format!("ushort k = 0"),
            format!("k < k_bound"),
            format!("k++"),
        );
        {
            w.line(format!("ushort s_bound = {S} / (1 << (k + 1));"));
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

fn decompose_across_warps_code(w: &mut Writer, unroll_l: bool, unroll_s: bool, unroll_u: bool) {
    let write_u_integrand = |w: &mut Writer, is_first: bool, s: &str, u: &str| {
        w.line(format!("ushort local_index = {s} * {U} + {u};"));
        if is_first {
            w.lines(vec![
                format!("uint tau_coef = mul(array[local_index], mult);"),
                format!("array[local_index] = tau_coef;"),
                format!("uint tau_index = g*{D} + ({s} * {T} + t) * {U} + {u};"),
                format!("shared[tau_index] = tau_coef;"),
            ]);
        } else {
            w.lines(vec![
                format!("uint sigma_index = g*{D} + ({s} * {T} + (sigma_warp * 32 + t_local)) * {U} + {u};"),
                format!("uint sigma_coef = shared[sigma_index];"),
                format!("uint tau_coef = array[local_index];"),
                format!("array[local_index] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);")
            ]);
        }
    };
    let write_s_integrand = |w: &mut Writer, is_first: bool, l: &str, s: &str| {
        if is_first {
            w.lines(vec![
                format!("uint i = {s} * (1 << {l}) + r;"),
                format!("uint mult = upper ? zetas((1 << ({LOG_S} + {l})) - 1 + i) : 1;"),
            ]);
        }
        if unroll_u {
            for u in 0..U {
                w.begin_scope();
                {
                    write_u_integrand(w, is_first, s, &format!("{u}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort u = 0"), format!("u < {U}"), format!("u++"));
            {
                write_u_integrand(w, is_first, s, "u");
            }
            w.end_for();
        }
    };
    let write_l_integrand = |w: &mut Writer, l: &str| {
        w.lines(vec![
            format!("ushort idx = {LOG_W} - {l} - 1;"),
            format!("ushort mask = 1 << idx;"),
            format!("ushort sigma_warp = tau_warp ^ mask;"),
            format!("bool upper = sigma_warp < tau_warp;"),
            format!("ushort r = t >> ({LOG_T} - {l});"),
        ]);
        if unroll_s {
            for s in 0..S {
                w.begin_scope();
                {
                    write_s_integrand(w, true, l, &format!("{s}"));
                }
                w.end_scope();
            }
            w.line(format!(
                "threadgroup_barrier(metal::mem_flags::mem_threadgroup);"
            ));
            for s in 0..S {
                w.begin_scope();
                {
                    write_s_integrand(w, false, l, &format!("{s}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
            {
                write_s_integrand(w, true, l, "s");
            }
            w.end_for();
            w.line(format!(
                "threadgroup_barrier(metal::mem_flags::mem_threadgroup);"
            ));
            w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
            {
                write_s_integrand(w, false, l, "s");
            }
            w.end_for();
        }
    };

    w.empty_line().comment(format!("DECOMPOSING ACROSS WARPS"));
    w.line(format!("ushort tau_warp = w;"));
    if unroll_l {
        for l in 0..LOG_W {
            w.begin_scope();
            {
                write_l_integrand(w, &format!("{l}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(
            format!("ushort l = 0"),
            format!("l < {LOG_W}"),
            format!("l++"),
        );
        {
            write_l_integrand(w, "l");
        }
        w.end_for();
    }
}

fn decompose_across_threads_code(w: &mut Writer, unroll_l: bool, unroll_s: bool, unroll_u: bool) {
    let write_u_integrand = |w: &mut Writer, s: &str, u: &str| {
        w.lines(vec![
            format!("ushort local_index = {s} * {U} + {u};"),
            format!("uint tau_coef = mul(array[local_index], mult);"),
            format!("uint sigma_coef = metal::simd_shuffle_xor(tau_coef, mask);"),
            format!("array[local_index] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);")
        ]);
    };
    let write_s_integrand = |w: &mut Writer, l: &str, s: &str| {
        w.lines(vec![
            format!("uint i = {s} * (1 << {l}) + r;"),
            format!("uint mult = upper ? zetas((1 << ({LOG_S} + {l})) - 1 + i) : 1;"),
        ]);
        if unroll_u {
            for u in 0..U {
                w.begin_scope();
                {
                    write_u_integrand(w, s, &format!("{u}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort u = 0"), format!("u < {U}"), format!("u++"));
            {
                write_u_integrand(w, s, "u");
            }
            w.end_for();
        }
    };
    let write_l_integrand = |w: &mut Writer, l: &str| {
        w.lines(vec![
            format!("ushort idx = {LOG_T} - {l} - 1;"),
            format!("ushort mask = 1 << idx;"),
            format!("ushort sigma = tau ^ mask;"),
            format!("bool upper = sigma < tau;"),
            format!("ushort r = tau >> ({LOG_T} - {l});"),
        ]);
        if unroll_s {
            for s in 0..S {
                w.begin_scope();
                {
                    write_s_integrand(w, l, &format!("{s}"));
                }
                w.end_scope();
            }
        } else {
            w.begin_for(format!("ushort s = 0"), format!("s < {S}"), format!("s++"));
            {
                write_s_integrand(w, l, "s");
            }
            w.end_for();
        }
    };

    w.empty_line()
        .comment(format!("DECOMPOSING ACROSS THREADS"));
    w.line(format!("ushort tau = t;"));
    if unroll_l {
        for l in LOG_W..LOG_T + LOG_U - LOG_ORD {
            w.begin_scope();
            {
                write_l_integrand(w, &format!("{l}"));
            }
            w.end_scope();
        }
    } else {
        w.begin_for(
            format!("ushort l = {LOG_W}"),
            format!("l < {}", LOG_T + LOG_U - LOG_ORD),
            format!("l++"),
        );
        {
            write_l_integrand(w, "l");
        }
        w.end_for();
    }
}

fn collect_into_threads_code(w: &mut Writer, unroll_mask: bool, unroll_v: bool) {
    w.empty_line()
        .comment(format!("COLLECT IRREDUCIBLES INTO THREADS"));
    let write_reshuffle = |w: &mut Writer| {
        w.begin_for(
            format!("ushort mask = 0"),
            format!("mask < Tj"),
            format!("mask++"),
        );
        {
            w.begin_for(
                format!("ushort v = 0"),
                format!("v < {S} / Tj"),
                format!("v++"),
            );
            {
                w.lines(vec![
                    format!("ushort other = t ^ mask;"),
                    format!("ushort new_index = v * Tj + other;"),
                    format!("ushort old_index = other * ({S} / Tj) + v;"),
                    format!("array[old_index] = tmp[new_index];"),
                ]);
            }
            w.end_for();
        }
        w.end_for();
    };

    let write_v_integrand = |w: &mut Writer, mask: &str, v: &str| {
        w.lines(vec![
            format!("ushort other = t ^ {mask};"),
            format!("ushort index = other * ({S} / {T_J}) + {v};"),
            format!("tmp[index] = metal::simd_shuffle_xor(array[index], {mask});"),
        ]);
    };
    let write_mask_integrand = |w: &mut Writer, mask: &str| {
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
    w.line(format!("uint tmp[{S}] = {{0}};"));
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
    write_reshuffle(w);
}

fn karatsuba_multiplication_code(w: &mut Writer, unroll: bool) {
    w.empty_line().comment(format!("KARATSUBA MULTIPLICATION"));
    fn karatsuba(
        w: &mut Writer,
        left: &str,
        left_index: &str,
        middle: &str,
        right: &str,
        right_index: &str,
        Theta: usize,
        unroll: bool,
    ) {
        w.comment(format!("Theta = {Theta}"));
        let middle_index = &format!("({Theta} - 2)");
        // 1: base case
        if Theta == 1 {
            let var_0 = format!("{left}[{left_index}]");
            let var_1 = format!("{right}[{right_index}]");
            w.line(format!("{var_0} = mul({var_0}, {var_1});"));
            return;
        }
        // 3: middle sums
        {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_2 = |theta: &str| format!("{left}[{left_index} + {Theta} / 2 + {theta}]");
            let var_3 = |theta: &str| format!("{middle}[{middle_index} + {Theta} / 2 + {theta}]");
            let var_4 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            let var_5 = |theta: &str| format!("{right}[{right_index} + {Theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 {
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
                    format!("theta < {Theta} / 2"),
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
                Theta / 2,
                unroll,
            );
            // middles
            karatsuba(
                w,
                middle,
                middle_index,
                middle,
                middle,
                &format!("{middle_index} + {Theta} / 2"),
                Theta / 2,
                unroll,
            );
            // tops
            karatsuba(
                w,
                left,
                &format!("{left_index} + {Theta} / 2"),
                middle,
                right,
                &format!("{right_index} + {Theta} / 2"),
                Theta / 2,
                unroll,
            );
        }
        w.end_scope();
        // 5: clarification
        // 6: middle differences
        {
            let var_0 = format!("{middle}[{middle_index} + {Theta} / 2 - 1]");
            let var_1 = format!("{left}[{left_index} + {Theta} / 2 - 1]");
            w.line(format!("{var_0} = sub({var_0}, {var_1});"));
        }
        if 2 < Theta {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_2 = |theta: &str| format!("{middle}[{middle_index} + {Theta} / 2 + {theta}]");
            let var_3 = |theta: &str| format!("{right}[{right_index} + {Theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 - 1 {
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
                    format!("theta < {Theta} / 2 - 1"),
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
            let var_0 = format!("{right}[{right_index} + {Theta} / 2 - 1]");
            let var_1 = format!("{left}[{left_index} + {Theta} - 1]");
            let var_2 = format!("{middle}[{middle_index} + {Theta} / 2 - 1]");
            w.lines(vec![
                format!("{var_0} = {var_1};"),
                format!("{var_1} = sub({var_2}, {var_0});"),
            ]);
        }
        // 8: symmetric difference
        if 2 < Theta {
            let var_0 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {Theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 - 1 {
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
                    format!("theta < {Theta} / 2 - 1"),
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
        if 2 < Theta {
            let var_0 = |theta: &str| format!("{left}[{left_index} + {Theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_2 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            let var_3 = |theta: &str| format!("{middle}[{middle_index} + {Theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 - 1 {
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
                    format!("theta < {Theta} / 2 - 1"),
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

    let Theta = T_J * S / (1 << K);
    w.line(format!("ushort u = 0;"));
    // extra space
    w.line(format!("uint middle[{}] = {{0}};", Theta * 2 - 1));
    w.line(format!(
        "uint minors[{}] = {{3614796953, 1208427060, 1889015752, 3198863462}};",
        Theta
    ));
    karatsuba(w, "array", "0", "middle", "minors", "0", Theta, unroll);

    // reduction
    w.lines(vec![
        format!("ushort component_index = (r * Tj + t) * ((1 << {K}) / Tj) + u;"),
        format!(
            "ushort zeta_index = (1 << (({J} - 1) * {LOG_S} + {K} + 1)) - 1 + component_index / 2;"
        ),
        format!("uint zeta = zetas(zeta_index);"),
        format!("if (component_index % 2) {{ zeta = sub(0, zeta); }}"),
    ]);
    w.begin_for(
        format!("ushort theta = 0"),
        format!("theta < {Theta} - 1"),
        format!("theta++"),
    );
    {
        w.lines(vec![
            format!("uint mult = mul(minors[theta], zeta);"),
            format!("array[theta] = add(array[theta], mult);"),
        ]);
    }
    w.end_for();
}

fn dual_karatsuba_multiplication_code(w: &mut Writer, unroll: bool) {
    fn dual_karatsuba(
        w: &mut Writer,
        left: &str,
        left_index: &str,
        middle: &str,
        right: &str,
        right_index: &str,
        Theta: usize,
        unroll: bool,
    ) {
        w.comment(format!("Theta = {Theta}"));
        let middle_index = &format!("({Theta} - 2)");
        // 1: base case
        if Theta == 1 {
            let var_0 = format!("{left}[{left_index}]");
            let var_1 = format!("{right}[{right_index}]");
            w.line(format!("{var_0} = mul({var_0}, {var_1});"));
            return;
        }
        // 3: copy upper half of left to middle
        {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {Theta} / 2 + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.line(format!("{} = {};", var_0(theta_str), var_1(theta_str)));
                }
            }
        }
        // 4: recursively multiply bottoms and tops
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
                Theta / 2,
                unroll,
            );
            // tops
            dual_karatsuba(
                w,
                middle,
                middle_index,
                middle,
                right,
                &format!("{right_index} + {Theta} / 2"),
                Theta / 2,
                unroll,
            );
        }
        w.end_scope();
        // 5: add left and right lower to upper halves
        {
            let var_0 = |theta: &str| format!("{left}[{left_index} + {Theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            let var_2 = |theta: &str| format!("{right}[{right_index} + {Theta} / 2 + {theta}]");
            let var_3 = |theta: &str| format!("{right}[{right_index} + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        format!(
                            "{} = {} + {};",
                            var_0(theta_str),
                            var_0(theta_str),
                            var_1(theta_str)
                        ),
                        format!(
                            "{} = {} + {};",
                            var_2(theta_str),
                            var_2(theta_str),
                            var_3(theta_str)
                        ),
                    ]);
                }
            }
        }
        // 6: recursively multiply middles
        {
            dual_karatsuba(
                w,
                left,
                &format!("{left_index} + {Theta} / 2"),
                middle,
                right,
                &format!("{right_index} + {Theta} / 2"),
                Theta / 2,
                unroll,
            );
        }
        // 7: subtract bottom and top from middle
        {
            let var_0 = |theta: &str| format!("{left}[{left_index} + {Theta} / 2 + {theta}]");
            let var_1 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_2 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 {
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
            }
        }
        // 8: finite field reduction
        {
            let var_0 = |theta: &str| format!("{middle}[{middle_index} + {theta}]");
            let var_1 = |theta: &str| format!("{left}[{left_index} + {theta}]");
            if unroll {
                for theta in 0..Theta / 2 {
                    let theta_str = &format!("{theta}");
                    w.lines(vec![
                        // format!("uint mult = ;", var_0(theta_str)),
                        format!(
                            "{} = add({}, mul({}, zeta));",
                            var_1(theta_str),
                            var_1(theta_str),
                            var_0(theta_str)
                        ),
                    ]);
                }
            }
        }
    }

    let Theta = T_J * S / (1 << K);
    w.line(format!("ushort u = 0;"));
    // extra space
    w.line(format!("uint middle[{}] = {{0}};", Theta));
    w.line(format!(
        "uint minors[{}] = {{1, 0, 0, 0}};",
        // {{3614796953, 1208427060, 1889015752, 3198863462}};",
        Theta
    ));
    w.lines(vec![
        format!("ushort component_index = (r * Tj + t) * ((1 << {K}) / Tj) + u;"),
        format!(
            "ushort zeta_index = (1 << (({J} - 1) * {LOG_S} + {K} + 1)) - 1 + component_index / 2;"
        ),
        format!("uint zeta = zetas(zeta_index);"),
        format!("if (component_index % 2) {{ zeta = sub(0, zeta); }}"),
    ]);
    dual_karatsuba(w, "array", "0", "middle", "minors", "0", Theta, unroll);
}

fn schoolbook_multiplication_code(w: &mut Writer) {
    let write_reshuffle = |w: &mut Writer| {
        w.begin_for(
            format!("ushort mask = 0"),
            format!("mask < Tj"),
            format!("mask++"),
        );
        {
            w.begin_for(
                format!("ushort v = 0"),
                format!("v < {S} / Tj"),
                format!("v++"),
            );
            {
                w.lines(vec![
                    format!("ushort other = t ^ mask;"),
                    format!("ushort new_index = v * Tj + other;"),
                    format!("ushort old_index = other * ({S} / Tj) + v;"),
                    format!("tmp[old_index] = array[new_index];"),
                ]);
            }
            w.end_for();
        }
        w.end_for();
    };
    let write_u_integrand = |w: &mut Writer, u: &str| {
        w.line(format!("uint acc[{T_J} * {S} / (1 << {K})] = {{0}};"));
        w.comment(format!("zeta"));
        w.lines(vec![
            format!("ushort component_index = (r * Tj + t) * ((1 << {K}) / Tj) + u;"),
            format!(
                "ushort zeta_index = (1 << (({J} - 1) * {LOG_S} + {K} + 1)) - 1 + component_index / 2;"
            ),
            format!("uint zeta = zetas(zeta_index);"),
            format!("if (component_index % 2) {{ zeta = sub(0, zeta); }}"),
        ]);
        w.comment(format!("inner"));
        // now for the quadratic time with loops
    };
    w.empty_line()
        .comment(format!("MULTIPLY IRREDUCIBLES WITHIN THREADS"));
    w.line(format!("uint tmp[{S}] = {{0}};"));
    w.begin_scope();
    {
        w.lines(vec![
            format!("uint mult_val = 2091658123;"),
            format!("uint add_val = 1523138830;"),
            format!("uint state = 1;"),
            format!("ushort s_bound = {S} / (1 << {K});"),
        ]);
        w.begin_for(
            format!("ushort u = 0"),
            format!("u < (1 << {K}) / Tj"),
            format!("u++"),
        );
        {
            write_u_integrand(w, "u");
        }
        w.end_for();
        write_reshuffle(w);
    }
    w.end_scope();
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

use std::fs::File;
use std::io::Write;
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
            .line(format!("#include \"arithmetic.h\""))
            .empty_line()
            .line(format!("kernel void {name}("))
            .indent();
        for i in 0..inputs.len() - 1 {
            myself.line(format!("{},", inputs[i].clone()));
        }
        myself
            .line(format!("{})", inputs[inputs.len() - 1]))
            .outdent()
            .line(format!("{{"))
            .indent();
        myself
    }
    pub fn indent(&mut self) -> &mut Self {
        self.indent += 1;
        self
    }
    pub fn outdent(&mut self) -> &mut Self {
        self.indent -= 1;
        self
    }
    pub fn flush(&mut self) -> std::io::Result<usize> {
        self.outdent().line(format!("}}"));
        let result = self.file.write(self.string.as_bytes());
        self.indent = 0;
        self.string = String::new();
        result
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
        self.lines(vec![format!("")])
    }
    pub fn begin_scope(&mut self) -> &mut Self {
        self.line(format!("{{")).indent()
    }
    pub fn end_scope(&mut self) -> &mut Self {
        self.outdent().line(format!("}}"))
    }
    pub fn begin_for(&mut self, init: String, cond: String, update: String) -> &mut Self {
        let opening = format!("for ({init}; {cond}; {update})");
        self.line(opening).begin_scope()
    }
    pub fn end_for(&mut self) -> &mut Self {
        self.end_scope()
    }
    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.line(format!("// {comment}"))
    }
}
