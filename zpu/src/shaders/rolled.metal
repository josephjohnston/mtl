#include <metal_stdlib>
#include <metal_atomic>
#include "fp.h"
using namespace metal;

// uint addresss(ushort b, ushort v, ushort w, ushort s, ushort t, ushort u)
// {
//     return ((((b * V + v) * W + w) * S + s) * T + t) * U + u;
// }

template <ushort S, ushort U>
void Kappa(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    ushort V,
    ushort W,
    ushort B,
    ushort t,
    ushort w,
    ushort b)
{

    ushort T = 32;
    ushort ORD = 8;
    ushort logW = ushort(log2(half(W)));
    ushort logS = ushort(log2(half(S)));
    ushort logT = uint(log2(half(T)));
    ushort logU = uint(log2(half(U)));
    ushort logOrd = uint(log2(half(ORD)));
    uint array[S * U];
    uint acc[S * U] = {0};
    uint mult_val = 2091658123;
    uint add_val = 1523138830;
    uint state = 1;

    uint global_index_prefix = ((((0 * W + w) * B + b) * S + 0) * T + t) * U + 0;
    for (ushort v = 0; v < V; v++)
    {
        // READING INPUT
        uint local_index_prefix = global_index_prefix + ((((v * W + 0) * B + 0) * S + 0) * T + 0) * U + 0;
        for (ushort s = 0; s < S; s++)
        {
            for (ushort u = 0; u < U; u++)
            {
                uint local_index = local_index_prefix + ((((0 * W + 0) * B + 0) * S + s) * T + 0) * U + u;
                array[s * U + u] = uint(input[local_index]);
            }
        }
        // DECOMPOSING WITHIN THREADS
        for (ushort k = 0; k < logS; k++)
        {
            for (ushort i = 0; i < (1 << k); i++)
            {
                for (ushort s = 0; s < S / (1 << (k + 1)); s++)
                {
                    for (ushort u = 0; u < U; u++)
                    {
                        ushort hi_index = (2 * i + 1) * (S / (1 << (k + 1))) * U + s * U + u;
                        uint zeta = get_zeta(k + 2, i);
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = (2 * i) * (S / (1 << (k + 1))) * U + s * U + u;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                        // ushort hi_index = (2 * i + 1) * (S / (1 << (k + 1))) * U + s * U + u;
                        // uint zeta = get_zeta(k + 2, i);
                        // uint mult = mul(array[hi_index][u], zeta);
                        // ushort lo_index = (2 * i) * (S / (1 << (k + 1))) * U + s * U + u;
                        // array[hi_index][u] = sub(array[lo_index][u], mult);
                        // array[lo_index][u] = add(array[lo_index][u], mult);
                    }
                }
            }
        }
        // DECOMPOSING ACROSS THREADS
        for (ushort l = 0; l < logT + logU - logOrd; l++)
        {
            ushort idx = logT - l - 1;
            ushort mask = 1 << idx;
            ushort tau = t;
            ushort sigma = tau ^ mask;
            ushort r = tau >> (logT - l);
            for (ushort s = 0; s < S; s++)
            {
                uint i = s * (1 << l) + r;
                uint zeta = get_zeta(logS + l + 2, i);
                for (uint u = 0; u < U; u++)
                {
                    uint tau_coef = array[s * U + u];
                    if (sigma < tau)
                    {
                        // for (uint u = 0; u < U; u++)
                        // {
                        tau_coef = mul(tau_coef, zeta);
                        // }
                    }
                    uint sigma_coef = simd_shuffle_xor(tau_coef, mask);
                    // for (uint u = 0; u < U; u++)
                    // {
                    array[s * U + u] = sigma < tau ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
                }
            }
        }

        // MULTIPLYING AND ACCUMULATING WITHIN WARPS
        // SCHOOLBOOK
        for (ushort s = 0; s < S; s++)
        {
            uint zeta = get_zeta(logS + logT + logU - logOrd - 1 + 2, s / 2);
            for (ushort u0 = 0; u0 < U; u0++)
            {
                if (u0 > 0)
                {
                    array[s * U + (U - u0)] = mul(array[s * U + (U - u0)], zeta);
                }
                state = add(mul(state, mult_val), add_val);
                for (ushort u1 = 0; u1 < U; u1++)
                {
                    acc[s * U + u1] = add(acc[s * U + u1], mul(array[(s * U + u1 + (ORD - u0)) % ORD], state));
                }
            }
        }

        // // KARATSUBA
        // // uint2 seed = seeds[w * T + t];
        // uint state = 1;
        // uint acc[S * U];
        // uint aux[U];
        // for (uchar u = 0; u < U; u++)
        // {
        //     aux[u] = 0;
        // }
        // for (ushort s = 0; s < S; s++)
        // {
        //     // generate s seed values
        //     uint constants[U];
        //     for (uchar u = 0; u < U; u++)
        //     {
        //         constants[u] = generate(seed, state);
        //     }
        //     uint zeta = get_zeta(logS + logT + logU - logORD - 1 + 2, s / 2);
        // }
    }

    // // *** SHARED MEMORY ***
    // uint r = 1;
    // uint U2 = U;
    // uint S2 = S * U / U2;
    // for (ushort m = 0; m * r < logW; m++)
    // {
    //     // WRITE
    //     uint group_size = W / (1 << (m + 1) * r);
    //     uint group = w >> logW - (m + 1) * r;
    //     if (group != 0)
    //     {
    //         uint element_start_index = w * S * T * U;
    //         for (ushort s = 0; s < S2; s++)
    //         {
    //             for (ushort u = 0; u < U2; u++)
    //             {
    //                 uint shared_index = element_start_index + s * T * U2 + t * U2 + u;
    //                 shared[shared_index] = acc[s * U + u];
    //             }
    //         }
    //     }
    //     // SYNC
    //     // threadgroup_barrier(mem_flags::mem_none);
    //     if (group != 0)
    //     {
    //         return;
    //     }
    //     // READ
    //     for (ushort e = 1; e < 1 << r; e++)
    //     {
    //         uint element_start_index = (e * group_size + w) * S * T * U;
    //         for (ushort s = 0; s < S2; s++)
    //         {
    //             for (ushort u = 0; u < U2; u++)
    //             {
    //                 uint coef_index = element_start_index + s * T * U2 + t * U2 + u;
    //                 acc[s * U + u] = add(acc[s * U + u], shared[coef_index]);
    //             }
    //         }
    //     }
    // }

    // *** SIMD ***

    // WRITING TO GLOBAL MEMORY
    for (ushort s = 0; s < S; s++)
    {
        for (ushort u = 0; u < U; u++)
        {
            uint index_prefix = ((((0 * W + w) * B + b) * S + s) * T + t) * U + u;
            // uint shared_index = w * (S * T * U) + s * T * U + t * U + u;
            output[index_prefix] = acc[s * U + u];
        }
    }
}

kernel void go(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    constant ushort &V,
    constant ushort &W,
    constant ushort &B,
    ushort t [[thread_index_in_simdgroup]],
    ushort w [[simdgroup_index_in_threadgroup]],
    ushort b [[threadgroup_position_in_grid]])
{
    Kappa<1 << 1, 1 << 1>(input, shared, output, V, W, B, t, w, b);
}
