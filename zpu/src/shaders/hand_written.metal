// #include <metal_stdlib>
// #include <metal_atomic>
#include "arithmetic.h"

struct Params
{
    const uint E;
    const ushort F;
    const ushort G;
    const ushort X;
    const ushort DEG;
};

uint get_index(
    uint e,
    uint F, uint f,
    ushort G, ushort g,
    ushort S, ushort s,
    ushort T, ushort tau,
    ushort U, ushort u)
{
    return ((((e * F + f) * G + g) * S + s) * T + tau) * U + u;
}

ushort log(ushort val)
{
    return ushort(metal::log2(half(val)));
}

template <ushort S, ushort T, ushort U>
void Algorithm_1(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    const uint e,
    const ushort w_global,
    const ushort t_local,
    constant const Params &params)
{
    const ushort F = params.F;
    const ushort G = params.G;

    ushort W = T / params.X;
    uint array[S * U];

    ushort w = w_global & (W - 1);
    ushort tau = w * 32 + t_local;
    ushort g = w_global / W;
    uint global_read_index_prefix = get_index(e, F, 0, G, g, S, 0, T, tau, U, 0);

    for (ushort f = 0; f < F; f++)
    {
        // READ INPUT
        for (ushort s = 0; s < S; s++)
        {
            for (ushort u = 0; u < U; u++)
            {
                uint global_read_index = global_read_index_prefix + get_index(0, F, 0, G, 0, S, s, T, 0, U, u);
                array[s * U + u] = uint(input[global_read_index]);
            }
        }
        global_read_index_prefix += get_index(0, F, 1, G, 0, S, 0, T, 0, U, 0);

        // DECOMPOSE WITHIN THREADS
        for (ushort k = 0; k < log(S); k++)
        {
            ushort s_bound = (S / (1 << (k + 1)));
            for (ushort i = 0; i < (1 << k); i++)
            {
                uint zeta = zetas((1 << k) - 1 + i);
                ushort lo_index_prefix = (2 * i) * s_bound;
                ushort hi_index_prefix = lo_index_prefix + s_bound;
                for (ushort s = 0; s < s_bound; s++)
                {
                    for (ushort u = 0; u < U; u++)
                    {
                        ushort hi_index = (hi_index_prefix + s) * U + u;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = (lo_index_prefix + s) * U + u;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
            }
        }

        // DECOMPOSE ACROSS WARPS
        ushort tau_warp = w;
        for (ushort l = 0; l < log(W); l++)
        {
            ushort idx = log(W) - l - 1;
            ushort mask = 1 << idx;
            ushort sigma_warp = tau_warp ^ mask;
            bool upper = sigma_warp < tau_warp;
            ushort r = tau >> (log(T) - l);
            for (ushort s = 0; s < S; s++)
            {
                uint i = s * (1 << l) + r;
                uint mult = upper ? zetas((1 << (log(S) + l)) - 1 + i) : 1;
                for (ushort u = 0; u < U; u++)
                {
                    ushort local_index = s * U + u;
                    uint tau_coef = mul(array[local_index], mult);
                    array[local_index] = tau_coef;
                    uint tau_index = g * S * T * U + (s * T + tau) * U + u;
                    shared[tau_index] = tau_coef;
                }
            }
            threadgroup_barrier(metal::mem_flags::mem_threadgroup);
            for (ushort s = 0; s < S; s++)
            {
                for (ushort u = 0; u < U; u++)
                {
                    ushort local_index = s * U + u;
                    uint sigma_index = g * S * T * U + (s * T + (sigma_warp * 32 + t_local)) * U + u;
                    uint sigma_coef = shared[sigma_index];
                    uint tau_coef = array[local_index];
                    array[local_index] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
                }
            }
        }

        // DECOMPOSE ACROSS THREADS
        for (ushort l = log(W); l < log(T) + log(U) - log(params.DEG); l++)
        {
            ushort idx = log(T) - l - 1;
            ushort mask = 1 << idx;
            ushort sigma = tau ^ mask;
            bool upper = sigma < tau;
            ushort r = tau >> (log(T) - l);
            for (ushort s = 0; s < S; s++)
            {
                uint i = s * (1 << l) + r;
                uint mult = upper ? zetas((1 << (log(S) + l)) - 1 + i) : 1;
                for (ushort u = 0; u < U; u++)
                {
                    ushort local_index = s * U + u;
                    uint tau_coef = mul(array[local_index], mult);
                    uint sigma_coef = metal::simd_shuffle_xor(tau_coef, mask);
                    array[local_index] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
                }
            }
        }

        // // MULTIPLY
        // {
        //     uint mult_val = 2091658123;
        //     uint add_val = 1523138830;
        //     uint state = 1;
        //     uint acc[S];

        //     for (ushort s = 0; s < S; s++)
        //     {
        //         // uint zeta = get_zeta(logS + logT + logU - logORD - 1 + 2, s / 2);
        //         for (ushort u0 = 0; u0 < U; u0++)
        //         {
        //             if (u0 > 0)
        //             {
        //                 array[s * U + (U - u0)] = mul(array[s * U + (U - u0)], zeta);
        //             }
        //             state = add(mul(state, mult_val), add_val);
        //             for (ushort u1 = 0; u1 < U; u1++)
        //             {
        //                 if (u0 == 0)
        //                 {
        //                     acc[s * U + u1] = mul(array[s * U + u1], state);
        //                 }
        //                 else
        //                 {
        //                     acc[s * U + u1] = add(acc[s * U + u1], mul(array[(s * U + u1 + (ORD - u0)) % ORD], state));
        //                 }
        //             }
        //         }
        //         for (ushort u = 0; u < U; u++)
        //         {
        //             output[global_index_prefix + s * T * U + u] = acc[s * U + u];
        //         }
        //     }
        // }

        // WRITE TO GLOBAL MEMORY
        uint global_write_index_prefix = get_index(e, F, f, G, g, S, 0, T, tau, U, 0);
        for (ushort s = 0; s < S; s++)
        {
            for (ushort u = 0; u < U; u++)
            {
                uint global_write_index = global_write_index_prefix + get_index(0, F, 0, G, 0, S, s, T, 0, U, u);
                output[global_write_index] = array[s * U + u];
            }
        }
    }
}

template <ushort S, ushort T, ushort T_J, ushort J, ushort K>
void Algorithm_2(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    const uint e,
    const ushort w_global,
    const ushort t_local,
    constant const Params &params)
{
    const ushort F = params.F;
    const ushort G = params.G;

    uint array[S];
    uint acc[S] = {0};
    const ushort W = T / params.X;
    const ushort g = w_global / W;
    const ushort w = w_global & (W - 1);
    const ushort tau = w * params.X + t_local;
    uint global_read_index_prefix = get_index(e, F, 0, G, g, S, 0, T, tau, 1, 0);

    for (ushort f = 0; f < F; f++)
    {
        // READ INPUT
        for (ushort s = 0; s < S; s++)
        {
            uint global_read_index = global_read_index_prefix + get_index(0, F, 0, G, 0, S, s, T, 0, 1, 0);
            array[s] = uint(input[global_read_index]);
        }
        global_read_index_prefix += get_index(0, F, 1, G, 0, S, 0, T, 0, 1, 0);

        // DECOMPOSE WITH CHAIN
        ushort t = tau;
        {
            ushort D = S * T;
            ushort Dj = D;
            ushort Tj = T;
            ushort r = 0;
            ushort i = 0;

            for (ushort j = 0; j <= J; j++)
            {
                if (0 < j)
                {
                    Dj /= S;
                    Tj /= S;
                    i = (tau / Tj) & (S - 1);
                    t &= (Tj - 1);

                    if (params.X < Dj)
                    // TRANSPOSE ACROSS WARPS
                    {
                        for (ushort s = 0; s < S; s++)
                        {
                            if (s == i)
                            {
                                continue;
                            }
                            ushort index = g * D + (r * S + s) * Dj + i * Tj + t;
                            shared[index] = array[s];
                        }
                        threadgroup_barrier(metal::mem_flags::mem_threadgroup);
                        for (ushort s = 0; s < S; s++)
                        {
                            if (s == i)
                            {
                                continue;
                            }
                            ushort index = g * D + (r * S + i) * Dj + s * Tj + t;
                            array[s] = shared[index];
                        }
                    }
                    else
                    // TRANSPOSE ACROSS THREADS
                    {
                        for (ushort s = 0; s < S; s++)
                        {
                            ushort mask = s * Tj;
                            ushort index = s ^ i;
                            array[index] = metal::simd_shuffle_xor(array[index], mask);
                            // ushort sigma = (r * S[j] + s_iter) * T[j] + t;
                            // array[index] = metal::simd_shuffle(array[index], sigma);
                        }
                    }
                }

                // DECOMPOSE WITHIN THREADS
                ushort r_new = r * S + i;
                ushort k_bound = j < J ? log(S) : K;
                for (ushort k = 0; k < k_bound; k++)
                {
                    ushort s_bound = (S / (1 << (k + 1)));
                    for (ushort i_new = 0; i_new < (1 << k); i_new++)
                    {
                        ushort component_index = r_new * (1 << k) + i_new;
                        uint zeta = zetas((1 << (j * log(S) + k)) - 1 + component_index);
                        ushort lo_index_prefix = (2 * i_new) * s_bound;
                        ushort hi_index_prefix = lo_index_prefix + s_bound;
                        for (ushort s = 0; s < s_bound; s++)
                        {
                            ushort hi_index = hi_index_prefix + s;
                            uint mult = mul(array[hi_index], zeta);
                            ushort lo_index = lo_index_prefix + s;
                            array[hi_index] = sub(array[lo_index], mult);
                            array[lo_index] = add(array[lo_index], mult);
                        }
                    }
                }

                r = tau / Tj;
            }
        }

        // uint tmp[S] = {0};
        // COLLECT IRREDUCIBLES INTO THREADS
        for (ushort mask = 0; mask < T_J; mask++)
        {
            for (ushort v = 0; v < S / T_J; v++)
            {
                ushort other = t ^ mask;
                ushort index = other * (S / T_J) + v;
                array[index] = metal::simd_shuffle_xor(array[index], mask);
            }
        }

        // SCHOOLBOOK
        {
            // uint mult_val = 2091658123;
            // uint add_val = 1523138830;
            // uint state = 1;
            uint minors[T_J * S / (1 << K)] = {3614796953, 1208427060, 1889015752, 3198863462};
            // {3614796953, 1208427060, 1889015752, 3198863462};
            // {1, 0, 0, 0};

            ushort s_bound = S / (1 << K);

            for (ushort u = 0; u < (1 << K) / T_J; u++)
            {
                // zeta
                ushort component_index = tau * ((1 << K) / T_J) + u;
                ushort zeta_index = (1 << (J * log(S) + K - 1)) - 1 + component_index / 2;
                uint zeta = zetas(zeta_index);
                if (component_index % 2 == 1)
                {
                    zeta = sub(0, zeta);
                }
                //  outer
                for (ushort s1 = 0; s1 < s_bound; s1++)
                {
                    for (ushort t1 = 0; t1 < T_J; t1++)
                    {
                        ushort coef_index = s1 * T_J + t1;
                        if (coef_index > 0)
                        {
                            ushort new_bottom_index = t1 == 0 ? (u + 1) * (S / (1 << K)) - s1 : (T_J - t1) * (S / T_J) + (u + 1) * s_bound - (s1 + 1);
                            array[new_bottom_index] = mul(array[new_bottom_index], zeta);
                        }
                        // state = add(mul(state, mult_val), add_val);
                        // inner
                        for (ushort s2 = 0; s2 < s_bound; s2++)
                        {
                            for (ushort t2 = 0; t2 < T_J; t2++)
                            {
                                ushort array_index = t2 * (S / T_J) + u * S / (1 << K) + s2;

                                ushort delta = (coef_index + (s2 * T_J + t2)) % (T_J * S / (1 << K));
                                ushort acc_index = u * (T_J * S / (1 << K)) + delta;
                                acc[acc_index] = add(acc[acc_index], mul(array[array_index], minors[coef_index]));
                            }
                        }
                    }
                }

                // for (ushort s2 = 0; s2 < s_bound; s2++)
                // {
                //     for (ushort t2 = 0; t2 < Tj; t2++)
                //     {
                //         ushort array_index = t2 * (S / Tj) + u * s_bound + s2;
                //         ushort acc_index = t2 * s_bound + s2;
                //         array[array_index] = acc[acc_index];
                //     }
                // }
            }
        }

        // for (ushort mask = 0; mask < T_J; mask++)
        // {
        //     for (ushort s = 0; s < S / (1 << K); s++)
        //     {
        //         for (ushort u = 0; u < (1 << K) / T_J; u++)
        //         {
        //             ushort array_index = mask * (S / T_J) + (u * (S / (1 << K)) + s);
        //             ushort off_index = u * (T_J * S / (1 << K)) + s * T_J + mask;
        //             acc[off_index] = array[array_index];
        //         }
        //     }
        // }

        // WRITE OUTPUT
        uint global_write_index_prefix = get_index(e, F, f, G, g, S, 0, T, 0, 1, 0);
        // ushort tau_lower = tau & (Tj - 1);
        for (ushort s = 0; s < S; s++)
        {
            // uint index = global_write_index_prefix + r * Dj + s * Tj + tau_lower;
            // uint index = (r * Tj + tau_lower) * S + s;
            uint index = tau * S + s;
            output[index] = acc[s];
        }
    }
}

kernel void
go(
    device uchar *input,
    device uint *output,
    constant const Params &params,
    threadgroup uint *shared,
    const uint e [[threadgroup_position_in_grid]],
    const ushort w_global [[simdgroup_index_in_threadgroup]],
    const ushort t_local [[thread_index_in_simdgroup]])
{
    ushort algo = 2;
    if (algo == 1)
    {
        // template <S, T, U>
        Algorithm_1<(1 << 2), (1 << 6), (1 << 0)>(input, shared, output, e, w_global, t_local, params);
    }
    else if (algo == 2)
    {
        // template <S, T_0, T_J = T_0/S^J, J, K>
        Algorithm_2<(1 << 2), (1 << 5), (1 << 1), 2, 1>(input, shared, output, e, w_global, t_local, params);
    }
}

// old unrolled schoolbook
// let write_reshuffle = | w : &mut LibraryWriter |
// {
//     w.begin_for(
//         format !("ushort mask = 0"),
//         format !("mask < Tj"),
//         format !("mask++"), );
//     {
//         w.begin_for(
//             format !("ushort v = 0"),
//             format !("v < {S} / Tj"),
//             format !("v++"), );
//         {
//             w.lines(vec ![
//                 format !("ushort other = t ^ mask;"),
//                 format !("ushort new_index = v * Tj + other;"),
//                 format !("ushort old_index = other * ({S} / Tj) + v;"),
//                 format !("tmp[old_index] = array[new_index];"),
//             ]);
//         }
//         w.end_for();
//     }
//     w.end_for();
// };
// let write_u_integrand = | w : &mut LibraryWriter, u : &str |
// {
//     w.line(format !("uint acc[{T_J} * {S} / (1 << {K})] = {{0}};"));
//     w.comment(format !("zeta"));
//     w.lines(vec ![
//         format !("ushort component_index = (r * Tj + t) * ((1 << {K}) / Tj) + u;"),
//         format !(
//             "ushort zeta_index = (1 << (({J} - 1) * {LOG_S} + {K} + 1)) - 1 + component_index / 2;"),
//         format !("uint zeta = zetas(zeta_index);"),
//         format !("if (component_index % 2) {{ zeta = sub(0, zeta); }}"),
//     ]);
//     w.comment(format !("inner"));
//     // now for the quadratic time with loops
// };
// w.line(format !("uint tmp[{S}] = {{0}};"));
// w.begin_scope();
// {
//     w.lines(vec ![
//         format !("uint mult_val = 2091658123;"),
//         format !("uint add_val = 1523138830;"),
//         format !("uint state = 1;"),
//         format !("ushort s_bound = {S} / (1 << {K});"),
//     ]);
//     w.begin_for(
//         format !("ushort u = 0"),
//         format !("u < (1 << {K}) / Tj"),
//         format !("u++"), );
//     {
//         write_u_integrand(w, "u");
//     }
//     w.end_for();
//     write_reshuffle(w);
// }
// w.end_scope();