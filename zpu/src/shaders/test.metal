#include <metal_stdlib>
#include "fp.h"
using namespace metal;

template <ushort S, ushort U>
void Kappa(
    device uchar *input,
    device uint *output,
    ushort W,
    ushort B,
    ushort t,
    ushort w,
    ushort b)
{
    ushort T = 32;
    uint warp_start_index = b * W * T + w * T;
    uint logS = ushort(log2(half(S)));
    uint array[S * U];
    // READING INPUT
    for (ushort s = 0; s < S; s++)
    {
        for (ushort u = 0; u < U; u++)
        {
            array[s * U + u] = input[warp_start_index * S * U + s * T * U + t * U + u];
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
                }
            }
        }
    }
    // DECOMPOSING ACROSS THREADS
    ushort logT = ushort(log2(half(T)));
    for (ushort l = 0; l < logT; l++)
    {
        ushort idx = logT - l - 1;
        ushort mask = 1 << idx;
        ushort tau = t;
        ushort sigma = tau ^ mask;
        for (ushort s = 0; s < S; s++)
        {
            for (ushort u = 0; u < U; u++)
            {
                uint tau_coef = array[s * U + u];
                // uint to_send = tau_coef;
                ushort r = t >> (logT - l);
                uint i = s * (1 << l) + r;
                uint zeta = get_zeta(logS + l + 2, i);
                // get_zeta(l + 2, 2 * i);
                if (sigma < tau)
                {
                    tau_coef = mul(tau_coef, zeta);
                }
                uint sigma_coef = simd_shuffle_xor(tau_coef, mask);
                // if (sigma < tau)
                // {
                //     array[s * U + u] = sigma_coef - tau_coef;
                // }
                // else
                // {
                //     array[s * U + u] = tau_coef + sigma_coef;
                // }
                array[s * U + u] = sigma < tau ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }
    }
    // MULTIPLYING
    // WRITING
    for (ushort s = 0; s < S; s++)
    {
        for (ushort u = 0; u < U; u++)
        {
            output[warp_start_index * S * U + s * T * U + t * U + u] = array[s * U + u];
        }
    }
}

kernel void go(
    device uchar *input,
    device uint *output,
    constant ushort &W,
    constant ushort &B,
    ushort t [[thread_index_in_simdgroup]],
    ushort w [[simdgroup_index_in_threadgroup]],
    ushort b [[threadgroup_position_in_grid]])
{
    Kappa<1 << 0, 1 << 3>(input, output, W, B, t, w, b);
}
