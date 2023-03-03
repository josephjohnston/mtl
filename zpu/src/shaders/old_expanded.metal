// template <uint D, uint S>
void expanded4(
    device uchar *input,
    device uint *output,
    uint g,
    uint t)
{
    uint vals[16];
    // // READING INPUT
    for (ushort s = 0; s < 16; s++)
    {
        vals[s] = input[g * (1 << 14) + s * ((1 << 14) / (1 << 4)) + t];
    }
    // vals[0] = input[g * (1 << 14) + 0 * ((1 << 14) / (1 << 4)) + t];
    // vals[1] = input[g * (1 << 14) + 1 * ((1 << 14) / (1 << 4)) + t];
    // vals[2] = input[g * (1 << 14) + 2 * ((1 << 14) / (1 << 4)) + t];
    // vals[3] = input[g * (1 << 14) + 3 * ((1 << 14) / (1 << 4)) + t];
    // vals[4] = input[g * (1 << 14) + 4 * ((1 << 14) / (1 << 4)) + t];
    // vals[5] = input[g * (1 << 14) + 5 * ((1 << 14) / (1 << 4)) + t];
    // vals[6] = input[g * (1 << 14) + 6 * ((1 << 14) / (1 << 4)) + t];
    // vals[7] = input[g * (1 << 14) + 7 * ((1 << 14) / (1 << 4)) + t];
    // vals[8] = input[g * (1 << 14) + 8 * ((1 << 14) / (1 << 4)) + t];
    // vals[9] = input[g * (1 << 14) + 9 * ((1 << 14) / (1 << 4)) + t];
    // vals[10] = input[g * (1 << 14) + 10 * ((1 << 14) / (1 << 4)) + t];
    // vals[11] = input[g * (1 << 14) + 11 * ((1 << 14) / (1 << 4)) + t];
    // vals[12] = input[g * (1 << 14) + 12 * ((1 << 14) / (1 << 4)) + t];
    // vals[13] = input[g * (1 << 14) + 13 * ((1 << 14) / (1 << 4)) + t];
    // vals[14] = input[g * (1 << 14) + 14 * ((1 << 14) / (1 << 4)) + t];
    // vals[15] = input[g * (1 << 14) + 15 * ((1 << 14) / (1 << 4)) + t];

    // READING CONSTANTS
    // REDUCING IN THREAD
    // 0
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 0;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 1
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 1;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 2
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 2;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 2;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 3
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 3;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 3;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 4
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 4;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 4;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 5
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 5;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 5;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 6
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 6;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 6;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 7
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 7;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 7;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 8
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (0 + 1))) + 8;
        uint lo_index = (2 * 0) * (16 / (1 << (0 + 1))) + 8;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }

    // 0
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (1 + 1))) + 0;
        uint lo_index = (2 * 0) * (16 / (1 << (1 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (1 + 1))) + 0;
        uint lo_index = (2 * 1) * (16 / (1 << (1 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 1
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (1 + 1))) + 1;
        uint lo_index = (2 * 0) * (16 / (1 << (1 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (1 + 1))) + 1;
        uint lo_index = (2 * 1) * (16 / (1 << (1 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 2
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (1 + 1))) + 2;
        uint lo_index = (2 * 0) * (16 / (1 << (1 + 1))) + 2;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (1 + 1))) + 2;
        uint lo_index = (2 * 1) * (16 / (1 << (1 + 1))) + 2;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // 3
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (1 + 1))) + 3;
        uint lo_index = (2 * 0) * (16 / (1 << (1 + 1))) + 3;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (1 + 1))) + 3;
        uint lo_index = (2 * 1) * (16 / (1 << (1 + 1))) + 3;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }

    // s=0
    // for (ushort i = 0; i < 4; i++)
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (2 + 1))) + 0;
        uint lo_index = (2 * 0) * (16 / (1 << (2 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (2 + 1))) + 0;
        uint lo_index = (2 * 1) * (16 / (1 << (2 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 2 + 1) * (16 / (1 << (2 + 1))) + 0;
        uint lo_index = (2 * 2) * (16 / (1 << (2 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 3 + 1) * (16 / (1 << (2 + 1))) + 0;
        uint lo_index = (2 * 3) * (16 / (1 << (2 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    // s=1
    // for (ushort i = 0; i < 4; i++)
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (2 + 1))) + 1;
        uint lo_index = (2 * 0) * (16 / (1 << (2 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (2 + 1))) + 1;
        uint lo_index = (2 * 1) * (16 / (1 << (2 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 2 + 1) * (16 / (1 << (2 + 1))) + 1;
        uint lo_index = (2 * 2) * (16 / (1 << (2 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 3 + 1) * (16 / (1 << (2 + 1))) + 1;
        uint lo_index = (2 * 3) * (16 / (1 << (2 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }

    // for (ushort i = 0; i < 8; i++)
    {
        uint hi_index = (2 * 0 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 0) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 1) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 2 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 2) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 3 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 3) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 4 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 4) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 5 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 5) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 6 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 6) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 7 + 1) * (16 / (1 << (3 + 1))) + 0;
        uint lo_index = (2 * 7) * (16 / (1 << (3 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }

    // for (ushort k = 0; k < ushort(log2(half(16))); k++)
    // {
    //     for (ushort i = 0; i < (1 << k); i++)
    //     {
    //         for (uint s = 0; s < 16 / (1 << (k + 1)); s++)
    //         {
    //             uint hi_index = (2 * i + 1) * (16 / (1 << (k + 1))) + s;
    //             uint lo_index = (2 * i) * (16 / (1 << (k + 1))) + s;
    //             uint mult = vals[hi_index] * (1 << 20);
    //             vals[hi_index] = vals[lo_index] + mult;
    //             vals[lo_index] = vals[lo_index] - mult;
    //         }
    //     }
    // }
    // ksi: 136
    // kis: 120
    // ski:

    // // REDUCING ACROSS THREADS
    // // MULTIPLYING
    // // WRITING
    // output[g * (1 << 14) + 0 * ((1 << 14) / (1 << 4)) + t] = vals[0];
    // output[g * (1 << 14) + 1 * ((1 << 14) / (1 << 4)) + t] = vals[1];
    // output[g * (1 << 14) + 2 * ((1 << 14) / (1 << 4)) + t] = vals[2];
    // output[g * (1 << 14) + 3 * ((1 << 14) / (1 << 4)) + t] = vals[3];
    // output[g * (1 << 14) + 4 * ((1 << 14) / (1 << 4)) + t] = vals[4];
    // output[g * (1 << 14) + 5 * ((1 << 14) / (1 << 4)) + t] = vals[5];
    // output[g * (1 << 14) + 6 * ((1 << 14) / (1 << 4)) + t] = vals[6];
    // output[g * (1 << 14) + 7 * ((1 << 14) / (1 << 4)) + t] = vals[7];
    // output[g * (1 << 14) + 8 * ((1 << 14) / (1 << 4)) + t] = vals[8];
    // output[g * (1 << 14) + 9 * ((1 << 14) / (1 << 4)) + t] = vals[9];
    // output[g * (1 << 14) + 10 * ((1 << 14) / (1 << 4)) + t] = vals[10];
    // output[g * (1 << 14) + 11 * ((1 << 14) / (1 << 4)) + t] = vals[11];
    // output[g * (1 << 14) + 12 * ((1 << 14) / (1 << 4)) + t] = vals[12];
    // output[g * (1 << 14) + 13 * ((1 << 14) / (1 << 4)) + t] = vals[13];
    // output[g * (1 << 14) + 14 * ((1 << 14) / (1 << 4)) + t] = vals[14];
    // output[g * (1 << 14) + 15 * ((1 << 14) / (1 << 4)) + t] = vals[15];
    for (ushort s = 0; s < 16; s++)
    {
        output[g * (1 << 14) + s * ((1 << 14) / (1 << 4)) + t] = vals[s];
    }
}

void expanded2(
    device uchar *input,
    device uint *output,
    uint g,
    uint t)
{
    // uint vals[2];
    // // READING INPUT
    // for (ushort s = 0; s < 4; s++)
    // {
    //     vals[s] = input[g * (1 << 12) + s * ((1 << 12) / (1 << 2)) + t];
    // }
    // // READING CONSTANTS
    // // REDUCING IN THREAD

    // // k=0
    // // s=0
    // for (ushort k = 0; k < ushort(log2(half(4))); k++)
    // {
    //     for (uint s = 0; s < 4 / (1 << (k + 1)); s++)
    //     {
    //         for (ushort i = 0; i < (1 << k); i++)
    //         {
    //             uint hi_index = (2 * i + 1) * (4 / (1 << (k + 1))) + s;
    //             uint lo_index = (2 * i) * (4 / (1 << (k + 1))) + s;
    //             uint mult = vals[hi_index] * (1 << 20);
    //             vals[hi_index] = vals[lo_index] + mult;
    //             vals[lo_index] = vals[lo_index] - mult;
    //         }
    //     }
    // }

    // // REDUCING ACROSS THREADS
    // // MULTIPLYING
    // // WRITING
    // for (ushort s = 0; s < 4; s++)
    // {
    //     output[g * (1 << 12) + s * ((1 << 12) / (1 << 2)) + t] = vals[s];
    // }

    uint vals[4];
    // READING INPUT
    for (ushort s = 0; s < 4; s++)
    {
        vals[s] = input[g * (1 << 12) + s * ((1 << 12) / 4) + t];
    }

    // k=0
    // {
    {
        uint hi_index = (2 * 0 + 1) * (4 / (1 << (0 + 1))) + 0;
        uint lo_index = (2 * 0) * (4 / (1 << (0 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 0 + 1) * (4 / (1 << (0 + 1))) + 1;
        uint lo_index = (2 * 0) * (4 / (1 << (0 + 1))) + 1;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }

    // }
    // k=1
    {
        uint hi_index = (2 * 0 + 1) * (4 / (1 << (1 + 1))) + 0;
        uint lo_index = (2 * 0) * (4 / (1 << (1 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }
    {
        uint hi_index = (2 * 1 + 1) * (4 / (1 << (1 + 1))) + 0;
        uint lo_index = (2 * 1) * (4 / (1 << (1 + 1))) + 0;
        uint mult = vals[hi_index] * (1 << 20);
        vals[hi_index] = vals[lo_index] + mult;
        vals[lo_index] = vals[lo_index] - mult;
    }

    for (ushort s = 0; s < 4; s++)
    {
        output[g * (1 << 12) + s * ((1 << 12) / 4) + t] = vals[s];
    }
}

#include <metal_stdlib>
#include "fp.h"
using namespace metal;

template <uint S>
void Kappa(
    uint D,
    uint T,
    uint U,
    device uchar *input,
    device uint *output,
    uint g,
    uint t)
{
    // uint D = S * T * U;
    uint array[S];
    // READING INPUT
    for (ushort s = 0; s < S; s++)
    {
        array[s] = input[g * D + s * (D / S) + t];
    }
    // DECOMPOSING WITHIN THREADS
    uint logS = ushort(log2(half(S)));
    for (ushort k = 0; k < logS; k++)
    {
        for (ushort i = 0; i < (1 << k); i++)
        {
            for (uint s = 0; s < S / (1 << (k + 1)); s++)
            {
                uint hi_index = (2 * i + 1) * (S / (1 << (k + 1))) + s;
                uint zeta = get_zeta(k + 2, i);
                uint mult = mul(array[hi_index], zeta);
                uint lo_index = (2 * i) * (S / (1 << (k + 1))) + s;
                array[hi_index] = sub(array[lo_index], mult);
                array[lo_index] = add(array[lo_index], mult);
            }
        }
    }
    // DECOMPOSING ACROSS THREADS
    ushort logT = ushort(log2(half(T)));
    for (ushort l = 0; l < logT; l++)
    {
        uint idx = logT - l - 1;
        uint mask = 1 << idx;
        uint tau = t;
        uint sigma = tau ^ mask;
        for (ushort s = 0; s < S; s++)
        {
            for (ushort u = 0; u < U; u++)
            {
                uint tau_coef = array[s * U + u];
                // uint to_send = tau_coef;
                uint r = t >> (logT - l);
                uint i = s * (1 << l) + r;
                uint zeta = 2;
                // get_zeta(logS + l + 2, i);
                //  get_zeta(l + 2, 2 * i);
                if (sigma < tau)
                {
                    tau_coef = mul(tau_coef, zeta);
                }
                uint sigma_coef = simd_shuffle_xor(tau_coef, mask);
                //  if (sigma < tau)
                //  {
                //      array[s * U + u] = sigma_coef - tau_coef;
                //  }
                //  else
                //  {
                //      array[s * U + u] = tau_coef + sigma_coef;
                //  }
                array[s * U + u] = sigma < tau ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }
    }
    // MULTIPLYING
    // WRITING
    for (ushort s = 0; s < S; s++)
    {
        output[g * D + s * (D / S) + t] = array[s];
    }
}

kernel void go(
    device uchar *input,
    device uint *output,
    constant uint &D,
    constant uint &T,
    constant uint &U,
    uint g [[threadgroup_position_in_grid]],
    uint t [[thread_position_in_threadgroup]])
{
    Kappa<1 << 1>(D, T, U, input, output, g, t);
}
