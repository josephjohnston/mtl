#include "fp.h"
using namespace metal;

kernel void go(
    device uchar *input,
    device uint *output,
    ushort t [[thread_index_in_simdgroup]],
    ushort w [[simdgroup_index_in_threadgroup]],
    ushort b [[threadgroup_position_in_grid]])
{
    uint array[32];

    uint in_global_index_prefix = b * 16384 + w * 1024 + t * 4;

    array[0] = input[in_global_index_prefix + 0];

    array[1] = input[in_global_index_prefix + 1];

    array[2] = input[in_global_index_prefix + 2];

    array[3] = input[in_global_index_prefix + 3];

    array[4] = input[in_global_index_prefix + 128];

    array[5] = input[in_global_index_prefix + 129];

    array[6] = input[in_global_index_prefix + 130];

    array[7] = input[in_global_index_prefix + 131];

    array[8] = input[in_global_index_prefix + 256];

    array[9] = input[in_global_index_prefix + 257];

    array[10] = input[in_global_index_prefix + 258];

    array[11] = input[in_global_index_prefix + 259];

    array[12] = input[in_global_index_prefix + 384];

    array[13] = input[in_global_index_prefix + 385];

    array[14] = input[in_global_index_prefix + 386];

    array[15] = input[in_global_index_prefix + 387];

    array[16] = input[in_global_index_prefix + 512];

    array[17] = input[in_global_index_prefix + 513];

    array[18] = input[in_global_index_prefix + 514];

    array[19] = input[in_global_index_prefix + 515];

    array[20] = input[in_global_index_prefix + 640];

    array[21] = input[in_global_index_prefix + 641];

    array[22] = input[in_global_index_prefix + 642];

    array[23] = input[in_global_index_prefix + 643];

    array[24] = input[in_global_index_prefix + 768];

    array[25] = input[in_global_index_prefix + 769];

    array[26] = input[in_global_index_prefix + 770];

    array[27] = input[in_global_index_prefix + 771];

    array[28] = input[in_global_index_prefix + 896];

    array[29] = input[in_global_index_prefix + 897];

    array[30] = input[in_global_index_prefix + 898];

    array[31] = input[in_global_index_prefix + 899];

    {
        uint mult = mul(array[16], 1048576);
        array[16] = sub(array[0], mult);
        array[0] = add(array[0], mult);
    }
    {
        uint mult = mul(array[17], 1048576);
        array[17] = sub(array[1], mult);
        array[1] = add(array[1], mult);
    }
    {
        uint mult = mul(array[18], 1048576);
        array[18] = sub(array[2], mult);
        array[2] = add(array[2], mult);
    }
    {
        uint mult = mul(array[19], 1048576);
        array[19] = sub(array[3], mult);
        array[3] = add(array[3], mult);
    }
    {
        uint mult = mul(array[20], 1048576);
        array[20] = sub(array[4], mult);
        array[4] = add(array[4], mult);
    }
    {
        uint mult = mul(array[21], 1048576);
        array[21] = sub(array[5], mult);
        array[5] = add(array[5], mult);
    }
    {
        uint mult = mul(array[22], 1048576);
        array[22] = sub(array[6], mult);
        array[6] = add(array[6], mult);
    }
    {
        uint mult = mul(array[23], 1048576);
        array[23] = sub(array[7], mult);
        array[7] = add(array[7], mult);
    }
    {
        uint mult = mul(array[24], 1048576);
        array[24] = sub(array[8], mult);
        array[8] = add(array[8], mult);
    }
    {
        uint mult = mul(array[25], 1048576);
        array[25] = sub(array[9], mult);
        array[9] = add(array[9], mult);
    }
    {
        uint mult = mul(array[26], 1048576);
        array[26] = sub(array[10], mult);
        array[10] = add(array[10], mult);
    }
    {
        uint mult = mul(array[27], 1048576);
        array[27] = sub(array[11], mult);
        array[11] = add(array[11], mult);
    }
    {
        uint mult = mul(array[28], 1048576);
        array[28] = sub(array[12], mult);
        array[12] = add(array[12], mult);
    }
    {
        uint mult = mul(array[29], 1048576);
        array[29] = sub(array[13], mult);
        array[13] = add(array[13], mult);
    }
    {
        uint mult = mul(array[30], 1048576);
        array[30] = sub(array[14], mult);
        array[14] = add(array[14], mult);
    }
    {
        uint mult = mul(array[31], 1048576);
        array[31] = sub(array[15], mult);
        array[15] = add(array[15], mult);
    }
    {
        uint mult = mul(array[8], 1024);
        array[8] = sub(array[0], mult);
        array[0] = add(array[0], mult);
    }
    {
        uint mult = mul(array[9], 1024);
        array[9] = sub(array[1], mult);
        array[1] = add(array[1], mult);
    }
    {
        uint mult = mul(array[10], 1024);
        array[10] = sub(array[2], mult);
        array[2] = add(array[2], mult);
    }
    {
        uint mult = mul(array[11], 1024);
        array[11] = sub(array[3], mult);
        array[3] = add(array[3], mult);
    }
    {
        uint mult = mul(array[12], 1024);
        array[12] = sub(array[4], mult);
        array[4] = add(array[4], mult);
    }
    {
        uint mult = mul(array[13], 1024);
        array[13] = sub(array[5], mult);
        array[5] = add(array[5], mult);
    }
    {
        uint mult = mul(array[14], 1024);
        array[14] = sub(array[6], mult);
        array[6] = add(array[6], mult);
    }
    {
        uint mult = mul(array[15], 1024);
        array[15] = sub(array[7], mult);
        array[7] = add(array[7], mult);
    }
    {
        uint mult = mul(array[24], 1073741824);
        array[24] = sub(array[16], mult);
        array[16] = add(array[16], mult);
    }
    {
        uint mult = mul(array[25], 1073741824);
        array[25] = sub(array[17], mult);
        array[17] = add(array[17], mult);
    }
    {
        uint mult = mul(array[26], 1073741824);
        array[26] = sub(array[18], mult);
        array[18] = add(array[18], mult);
    }
    {
        uint mult = mul(array[27], 1073741824);
        array[27] = sub(array[19], mult);
        array[19] = add(array[19], mult);
    }
    {
        uint mult = mul(array[28], 1073741824);
        array[28] = sub(array[20], mult);
        array[20] = add(array[20], mult);
    }
    {
        uint mult = mul(array[29], 1073741824);
        array[29] = sub(array[21], mult);
        array[21] = add(array[21], mult);
    }
    {
        uint mult = mul(array[30], 1073741824);
        array[30] = sub(array[22], mult);
        array[22] = add(array[22], mult);
    }
    {
        uint mult = mul(array[31], 1073741824);
        array[31] = sub(array[23], mult);
        array[23] = add(array[23], mult);
    }
    {
        uint mult = mul(array[4], 32);
        array[4] = sub(array[0], mult);
        array[0] = add(array[0], mult);
    }
    {
        uint mult = mul(array[5], 32);
        array[5] = sub(array[1], mult);
        array[1] = add(array[1], mult);
    }
    {
        uint mult = mul(array[6], 32);
        array[6] = sub(array[2], mult);
        array[2] = add(array[2], mult);
    }
    {
        uint mult = mul(array[7], 32);
        array[7] = sub(array[3], mult);
        array[3] = add(array[3], mult);
    }
    {
        uint mult = mul(array[12], 33554432);
        array[12] = sub(array[8], mult);
        array[8] = add(array[8], mult);
    }
    {
        uint mult = mul(array[13], 33554432);
        array[13] = sub(array[9], mult);
        array[9] = add(array[9], mult);
    }
    {
        uint mult = mul(array[14], 33554432);
        array[14] = sub(array[10], mult);
        array[10] = add(array[10], mult);
    }
    {
        uint mult = mul(array[15], 33554432);
        array[15] = sub(array[11], mult);
        array[11] = add(array[11], mult);
    }
    {
        uint mult = mul(array[20], 32768);
        array[20] = sub(array[16], mult);
        array[16] = add(array[16], mult);
    }
    {
        uint mult = mul(array[21], 32768);
        array[21] = sub(array[17], mult);
        array[17] = add(array[17], mult);
    }
    {
        uint mult = mul(array[22], 32768);
        array[22] = sub(array[18], mult);
        array[18] = add(array[18], mult);
    }
    {
        uint mult = mul(array[23], 32768);
        array[23] = sub(array[19], mult);
        array[19] = add(array[19], mult);
    }
    {
        uint mult = mul(array[28], 4144559881);
        array[28] = sub(array[24], mult);
        array[24] = add(array[24], mult);
    }
    {
        uint mult = mul(array[29], 4144559881);
        array[29] = sub(array[25], mult);
        array[25] = add(array[25], mult);
    }
    {
        uint mult = mul(array[30], 4144559881);
        array[30] = sub(array[26], mult);
        array[26] = add(array[26], mult);
    }
    {
        uint mult = mul(array[31], 4144559881);
        array[31] = sub(array[27], mult);
        array[27] = add(array[27], mult);
    }

    ushort tau = t;

    {
        ushort sigma = tau ^ 16;
        bool upper = tau > sigma;
        ushort r = tau >> (5);

        {
            ushort i = 0 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[0], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[0] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[1], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[1] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[2], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[2] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[3], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[3] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 1 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[4], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[4] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[5], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[5] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[6], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[6] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[7], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[7] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 2 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[8], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[8] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[9], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[9] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[10], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[10] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[11], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[11] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 3 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[12], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[12] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[13], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[13] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[14], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[14] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[15], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[15] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 4 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[16], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[16] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[17], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[17] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[18], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[18] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[19], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[19] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 5 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[20], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[20] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[21], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[21] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[22], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[22] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[23], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[23] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 6 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[24], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[24] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[25], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[25] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[26], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[26] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[27], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[27] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 7 + r;
            uint mult = upper ? get_zeta(5, i) : 1;

            {
                uint tau_coef = mul(array[28], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[28] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[29], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[29] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[30], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[30] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[31], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 16);
                array[31] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }
    }

    {
        ushort sigma = tau ^ 8;
        bool upper = tau > sigma;
        ushort r = tau >> (4);

        {
            ushort i = 0 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[0], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[0] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[1], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[1] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[2], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[2] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[3], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[3] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 2 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[4], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[4] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[5], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[5] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[6], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[6] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[7], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[7] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 4 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[8], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[8] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[9], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[9] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[10], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[10] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[11], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[11] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 6 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[12], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[12] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[13], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[13] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[14], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[14] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[15], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[15] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 8 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[16], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[16] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[17], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[17] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[18], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[18] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[19], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[19] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 10 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[20], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[20] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[21], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[21] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[22], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[22] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[23], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[23] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 12 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[24], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[24] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[25], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[25] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[26], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[26] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[27], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[27] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 14 + r;
            uint mult = upper ? get_zeta(6, i) : 1;

            {
                uint tau_coef = mul(array[28], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[28] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[29], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[29] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[30], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[30] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[31], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 8);
                array[31] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }
    }

    {
        ushort sigma = tau ^ 4;
        bool upper = tau > sigma;
        ushort r = tau >> (3);

        {
            ushort i = 0 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[0], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[0] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[1], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[1] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[2], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[2] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[3], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[3] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 4 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[4], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[4] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[5], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[5] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[6], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[6] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[7], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[7] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 8 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[8], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[8] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[9], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[9] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[10], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[10] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[11], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[11] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 12 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[12], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[12] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[13], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[13] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[14], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[14] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[15], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[15] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 16 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[16], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[16] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[17], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[17] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[18], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[18] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[19], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[19] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 20 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[20], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[20] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[21], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[21] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[22], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[22] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[23], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[23] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 24 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[24], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[24] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[25], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[25] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[26], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[26] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[27], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[27] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }

        {
            ushort i = 28 + r;
            uint mult = upper ? get_zeta(7, i) : 1;

            {
                uint tau_coef = mul(array[28], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[28] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[29], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[29] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[30], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[30] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }

            {
                uint tau_coef = mul(array[31], mult);
                uint sigma_coef = simd_shuffle_xor(tau_coef, 4);
                array[31] = upper ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }
    }

    uint out_global_index_prefix = b * 16384 + w * 1024 + t * 4;

    output[out_global_index_prefix + 0] = array[0];

    output[out_global_index_prefix + 1] = array[1];

    output[out_global_index_prefix + 2] = array[2];

    output[out_global_index_prefix + 3] = array[3];

    output[out_global_index_prefix + 128] = array[4];

    output[out_global_index_prefix + 129] = array[5];

    output[out_global_index_prefix + 130] = array[6];

    output[out_global_index_prefix + 131] = array[7];

    output[out_global_index_prefix + 256] = array[8];

    output[out_global_index_prefix + 257] = array[9];

    output[out_global_index_prefix + 258] = array[10];

    output[out_global_index_prefix + 259] = array[11];

    output[out_global_index_prefix + 384] = array[12];

    output[out_global_index_prefix + 385] = array[13];

    output[out_global_index_prefix + 386] = array[14];

    output[out_global_index_prefix + 387] = array[15];

    output[out_global_index_prefix + 512] = array[16];

    output[out_global_index_prefix + 513] = array[17];

    output[out_global_index_prefix + 514] = array[18];

    output[out_global_index_prefix + 515] = array[19];

    output[out_global_index_prefix + 640] = array[20];

    output[out_global_index_prefix + 641] = array[21];

    output[out_global_index_prefix + 642] = array[22];

    output[out_global_index_prefix + 643] = array[23];

    output[out_global_index_prefix + 768] = array[24];

    output[out_global_index_prefix + 769] = array[25];

    output[out_global_index_prefix + 770] = array[26];

    output[out_global_index_prefix + 771] = array[27];

    output[out_global_index_prefix + 896] = array[28];

    output[out_global_index_prefix + 897] = array[29];

    output[out_global_index_prefix + 898] = array[30];

    output[out_global_index_prefix + 899] = array[31];
}

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
    uint global_index_prefix = b * W * S * T * U + w * S * T * U + t * U;
    uint logS = ushort(log2(half(S)));
    uint array[S * U];

    // READING INPUT
    for (ushort s = 0; s < S; s++)
    {
        for (ushort u = 0; u < U; u++)
        {
            array[s * U + u] = input[global_index_prefix + s * T * U + u];
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
    ushort logT = uint(log2(half(T)));
    for (ushort l = 0; l <= 6 - logS; l++)
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
                    tau_coef = mul(tau_coef, zeta);
                }
                uint sigma_coef = simd_shuffle_xor(tau_coef, mask);
                array[s * U + u] = sigma < tau ? sub(sigma_coef, tau_coef) : add(tau_coef, sigma_coef);
            }
        }
    }
    // WRITING
    for (ushort s = 0; s < S; s++)
    {
        for (ushort u = 0; u < U; u++)
        {
            output[global_index_prefix + s * T * U + u] = array[s * U + u];
        }
    }
}

kernel void go_slow(
    device uchar *input,
    device uint *output,
    constant ushort &W,
    constant ushort &B,
    ushort t [[thread_index_in_simdgroup]],
    ushort w [[simdgroup_index_in_threadgroup]],
    ushort b [[threadgroup_position_in_grid]])
{
    Kappa<1 << 3, 1 << 2>(input, output, W, B, t, w, b);
}