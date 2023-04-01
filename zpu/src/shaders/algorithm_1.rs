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

fn read_code(w: &mut Writer, unroll_s: bool, unroll_u: bool) {
    let write_u_integrand = |w: &mut Writer, s: &str, u: &str| {
        w.lines(vec![
            format!(
                "uint global_read_index = global_read_index_prefix + {};",
                get_index("0", "0", "0", s, "0")
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
