
#include "arithmetic.h"

kernel void go(
    device uint *input,
    threadgroup uint *shared,
    device uint *output,
    device uint *constants,
    uint e [[threadgroup_position_in_grid]],
    ushort w_global [[simdgroup_index_in_threadgroup]],
    ushort t_local [[thread_index_in_simdgroup]])
{
    uint array[8];
    uint acc[8] = {0};
    ushort g = w_global >> 2;
    ushort w = w_global & (4 - 1);
    ushort tau = w * 32 + t_local;
    uint global_read_index_prefix = e * 1024 + 0 * 1024 + g * 1024 + 0 * 128 + tau * 1;
    
    for (ushort f = 0; f < 1; f++)
    {
        // READ INPUT
        {
            array[0] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 0 * 128 + 0 * 1]);
        }
        {
            array[1] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 1 * 128 + 0 * 1]);
        }
        {
            array[2] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 2 * 128 + 0 * 1]);
        }
        {
            array[3] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 3 * 128 + 0 * 1]);
        }
        {
            array[4] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 4 * 128 + 0 * 1]);
        }
        {
            array[5] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 5 * 128 + 0 * 1]);
        }
        {
            array[6] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 6 * 128 + 0 * 1]);
        }
        {
            array[7] = uint(input[global_read_index_prefix + 0 * 1024 + 0 * 1024 + 0 * 1024 + 7 * 128 + 0 * 1]);
        }
        global_read_index_prefix += 0 * 1024 + 1 * 1024 + 0 * 1024 + 0 * 128 + 0 * 1;
        
        // DECOMPOSE WITH CHAIN
        {
            // DECOMPOSE WITHIN THREADS 0
            {
                uint zeta = 1048576;
                ushort lo_index_prefix = 0;
                ushort hi_index_prefix = 4;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
                {
                    ushort hi_index = hi_index_prefix + 1;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 1;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
                {
                    ushort hi_index = hi_index_prefix + 2;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 2;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
                {
                    ushort hi_index = hi_index_prefix + 3;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 3;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                uint zeta = 1024;
                ushort lo_index_prefix = 0;
                ushort hi_index_prefix = 2;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
                {
                    ushort hi_index = hi_index_prefix + 1;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 1;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                uint zeta = 1073741824;
                ushort lo_index_prefix = 4;
                ushort hi_index_prefix = 6;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
                {
                    ushort hi_index = hi_index_prefix + 1;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 1;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                uint zeta = 32;
                ushort lo_index_prefix = 0;
                ushort hi_index_prefix = 1;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                uint zeta = 33554432;
                ushort lo_index_prefix = 2;
                ushort hi_index_prefix = 3;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                uint zeta = 32768;
                ushort lo_index_prefix = 4;
                ushort hi_index_prefix = 5;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                uint zeta = 4144559881;
                ushort lo_index_prefix = 6;
                ushort hi_index_prefix = 7;
                {
                    ushort hi_index = hi_index_prefix + 0;
                    uint mult = mul(array[hi_index], zeta);
                    ushort lo_index = lo_index_prefix + 0;
                    array[hi_index] = sub(array[lo_index], mult);
                    array[lo_index] = add(array[lo_index], mult);
                }
            }
            {
                ushort log_power_S_to_j = 1 * 3;
                ushort log_Dj = (3 + 7) - log_power_S_to_j;
                ushort r = tau >> log_Dj;
                ushort Tj = 128 >> log_power_S_to_j;
                ushort log_Tj = 7 - log_power_S_to_j;
                ushort i = (tau >> log_Tj) & (8 - 1);
                ushort t = tau & (Tj - 1);
                
                if (5 < log_Dj)
                // TRANSPOSE ACROSS WARPS
                {
                    ushort Dj = 1 << log_Dj;
                    {
                        ushort index = g * 1024 + (r * 8 + 0) * Dj + i * Tj + t;
                        shared[index] = array[0];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 1) * Dj + i * Tj + t;
                        shared[index] = array[1];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 2) * Dj + i * Tj + t;
                        shared[index] = array[2];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 3) * Dj + i * Tj + t;
                        shared[index] = array[3];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 4) * Dj + i * Tj + t;
                        shared[index] = array[4];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 5) * Dj + i * Tj + t;
                        shared[index] = array[5];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 6) * Dj + i * Tj + t;
                        shared[index] = array[6];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 7) * Dj + i * Tj + t;
                        shared[index] = array[7];
                    }
                    threadgroup_barrier(metal::mem_flags::mem_threadgroup);
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 0 * Tj + t;
                        array[0] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 1 * Tj + t;
                        array[1] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 2 * Tj + t;
                        array[2] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 3 * Tj + t;
                        array[3] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 4 * Tj + t;
                        array[4] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 5 * Tj + t;
                        array[5] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 6 * Tj + t;
                        array[6] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 7 * Tj + t;
                        array[7] = shared[index];
                    }
                }
                else
                // TRANSPOSE ACROSS THREADS
                {
                    {
                        ushort index = 0 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 0 * Tj);
                    }
                    {
                        ushort index = 1 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 1 * Tj);
                    }
                    {
                        ushort index = 2 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 2 * Tj);
                    }
                    {
                        ushort index = 3 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 3 * Tj);
                    }
                    {
                        ushort index = 4 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 4 * Tj);
                    }
                    {
                        ushort index = 5 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 5 * Tj);
                    }
                    {
                        ushort index = 6 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 6 * Tj);
                    }
                    {
                        ushort index = 7 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 7 * Tj);
                    }
                }
                
                // DECOMPOSE WITHIN THREADS j > 0
                ushort r_new = r * 8 + i;
                {
                    ushort component_index = r_new * (1 << 0) + 0;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 0)) - 1 + component_index];
                    ushort lo_index_prefix = 0;
                    ushort hi_index_prefix = 4;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 1;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 1;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 2;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 2;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 3;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 3;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
                {
                    ushort component_index = r_new * (1 << 1) + 0;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 1)) - 1 + component_index];
                    ushort lo_index_prefix = 0;
                    ushort hi_index_prefix = 2;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 1;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 1;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
                {
                    ushort component_index = r_new * (1 << 1) + 1;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 1)) - 1 + component_index];
                    ushort lo_index_prefix = 4;
                    ushort hi_index_prefix = 6;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 1;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 1;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
                {
                    ushort component_index = r_new * (1 << 2) + 0;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 2)) - 1 + component_index];
                    ushort lo_index_prefix = 0;
                    ushort hi_index_prefix = 1;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
                {
                    ushort component_index = r_new * (1 << 2) + 1;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 2)) - 1 + component_index];
                    ushort lo_index_prefix = 2;
                    ushort hi_index_prefix = 3;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
                {
                    ushort component_index = r_new * (1 << 2) + 2;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 2)) - 1 + component_index];
                    ushort lo_index_prefix = 4;
                    ushort hi_index_prefix = 5;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
                {
                    ushort component_index = r_new * (1 << 2) + 3;
                    uint zeta = zeta_arrays[(1 << (1 * 3 + 2)) - 1 + component_index];
                    ushort lo_index_prefix = 6;
                    ushort hi_index_prefix = 7;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
            }
            {
                ushort log_power_S_to_j = 2 * 3;
                ushort log_Dj = (3 + 7) - log_power_S_to_j;
                ushort r = tau >> log_Dj;
                ushort Tj = 128 >> log_power_S_to_j;
                ushort log_Tj = 7 - log_power_S_to_j;
                ushort i = (tau >> log_Tj) & (8 - 1);
                ushort t = tau & (Tj - 1);
                
                if (5 < log_Dj)
                // TRANSPOSE ACROSS WARPS
                {
                    ushort Dj = 1 << log_Dj;
                    {
                        ushort index = g * 1024 + (r * 8 + 0) * Dj + i * Tj + t;
                        shared[index] = array[0];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 1) * Dj + i * Tj + t;
                        shared[index] = array[1];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 2) * Dj + i * Tj + t;
                        shared[index] = array[2];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 3) * Dj + i * Tj + t;
                        shared[index] = array[3];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 4) * Dj + i * Tj + t;
                        shared[index] = array[4];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 5) * Dj + i * Tj + t;
                        shared[index] = array[5];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 6) * Dj + i * Tj + t;
                        shared[index] = array[6];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + 7) * Dj + i * Tj + t;
                        shared[index] = array[7];
                    }
                    threadgroup_barrier(metal::mem_flags::mem_threadgroup);
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 0 * Tj + t;
                        array[0] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 1 * Tj + t;
                        array[1] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 2 * Tj + t;
                        array[2] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 3 * Tj + t;
                        array[3] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 4 * Tj + t;
                        array[4] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 5 * Tj + t;
                        array[5] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 6 * Tj + t;
                        array[6] = shared[index];
                    }
                    {
                        ushort index = g * 1024 + (r * 8 + i) * Dj + 7 * Tj + t;
                        array[7] = shared[index];
                    }
                }
                else
                // TRANSPOSE ACROSS THREADS
                {
                    {
                        ushort index = 0 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 0 * Tj);
                    }
                    {
                        ushort index = 1 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 1 * Tj);
                    }
                    {
                        ushort index = 2 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 2 * Tj);
                    }
                    {
                        ushort index = 3 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 3 * Tj);
                    }
                    {
                        ushort index = 4 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 4 * Tj);
                    }
                    {
                        ushort index = 5 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 5 * Tj);
                    }
                    {
                        ushort index = 6 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 6 * Tj);
                    }
                    {
                        ushort index = 7 ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], 7 * Tj);
                    }
                }
                
                // DECOMPOSE WITHIN THREADS j > 0
                ushort r_new = r * 8 + i;
                {
                    ushort component_index = r_new * (1 << 0) + 0;
                    uint zeta = zeta_arrays[(1 << (2 * 3 + 0)) - 1 + component_index];
                    ushort lo_index_prefix = 0;
                    ushort hi_index_prefix = 4;
                    {
                        ushort hi_index = hi_index_prefix + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 1;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 1;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 2;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 2;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                    {
                        ushort hi_index = hi_index_prefix + 3;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = lo_index_prefix + 3;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
            }
        }
        
        // COLLECT COMPONENTS INTO THREADS
        {
            ushort t = tau & (2 - 1);
            {
                ushort other = t ^ 1;
                ushort scaled_other = other * (8 / 2);
                {
                    ushort index = scaled_other + 0;
                    array[index] = metal::simd_shuffle_xor(array[index], 1);
                }
                {
                    ushort index = scaled_other + 1;
                    array[index] = metal::simd_shuffle_xor(array[index], 1);
                }
                {
                    ushort index = scaled_other + 2;
                    array[index] = metal::simd_shuffle_xor(array[index], 1);
                }
                {
                    ushort index = scaled_other + 3;
                    array[index] = metal::simd_shuffle_xor(array[index], 1);
                }
            }
        }
        
        // KARATSUBA MULTIPLICATION
        uint middle[15];
        uint minors[8] = {3614796953, 1208427060, 1889015752, 3198863462, 3614796953, 1208427060, 1889015752, 3198863462};
        acc[0] = add(acc[0], array[0]);
        acc[2] = add(acc[2], array[1]);
        acc[4] = add(acc[4], array[2]);
        acc[6] = add(acc[6], array[3]);
        acc[1] = add(acc[1], array[4]);
        acc[3] = add(acc[3], array[5]);
        acc[5] = add(acc[5], array[6]);
        acc[7] = add(acc[7], array[7]);
        {
            ushort acc_offset = 0 * (2 * 8 / (1 << 1));
            // Theta = 8
            middle[(8 - 2) + 0] = add(acc[acc_offset + 0], acc[acc_offset + 8 / 2 + 0]);
            middle[(8 - 2) + 8 / 2 + 0] = add(minors[0 + 0], minors[0 + 8 / 2 + 0]);
            middle[(8 - 2) + 1] = add(acc[acc_offset + 1], acc[acc_offset + 8 / 2 + 1]);
            middle[(8 - 2) + 8 / 2 + 1] = add(minors[0 + 1], minors[0 + 8 / 2 + 1]);
            middle[(8 - 2) + 2] = add(acc[acc_offset + 2], acc[acc_offset + 8 / 2 + 2]);
            middle[(8 - 2) + 8 / 2 + 2] = add(minors[0 + 2], minors[0 + 8 / 2 + 2]);
            middle[(8 - 2) + 3] = add(acc[acc_offset + 3], acc[acc_offset + 8 / 2 + 3]);
            middle[(8 - 2) + 8 / 2 + 3] = add(minors[0 + 3], minors[0 + 8 / 2 + 3]);
            {
                // Theta = 4
                middle[(4 - 2) + 0] = add(acc[acc_offset + 0], acc[acc_offset + 4 / 2 + 0]);
                middle[(4 - 2) + 4 / 2 + 0] = add(minors[0 + 0], minors[0 + 4 / 2 + 0]);
                middle[(4 - 2) + 1] = add(acc[acc_offset + 1], acc[acc_offset + 4 / 2 + 1]);
                middle[(4 - 2) + 4 / 2 + 1] = add(minors[0 + 1], minors[0 + 4 / 2 + 1]);
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(acc[acc_offset + 0], acc[acc_offset + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(minors[0 + 0], minors[0 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        acc[acc_offset] = mul(acc[acc_offset], minors[0]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        acc[acc_offset + 2 / 2] = mul(acc[acc_offset + 2 / 2], minors[0 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], acc[acc_offset + 2 / 2 - 1]);
                    minors[0 + 2 / 2 - 1] = acc[acc_offset + 2 - 1];
                    acc[acc_offset + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], minors[0 + 2 / 2 - 1]);
                }
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(middle[(4 - 2) + 0], middle[(4 - 2) + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(middle[(4 - 2) + 4 / 2 + 0], middle[(4 - 2) + 4 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        middle[(4 - 2)] = mul(middle[(4 - 2)], middle[(4 - 2) + 4 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(4 - 2) + 2 / 2] = mul(middle[(4 - 2) + 2 / 2], middle[(4 - 2) + 4 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(4 - 2) + 2 / 2 - 1]);
                    middle[(4 - 2) + 4 / 2 + 2 / 2 - 1] = middle[(4 - 2) + 2 - 1];
                    middle[(4 - 2) + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(4 - 2) + 4 / 2 + 2 / 2 - 1]);
                }
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(acc[acc_offset + 4 / 2 + 0], acc[acc_offset + 4 / 2 + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(minors[0 + 4 / 2 + 0], minors[0 + 4 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        acc[acc_offset + 4 / 2] = mul(acc[acc_offset + 4 / 2], minors[0 + 4 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        acc[acc_offset + 4 / 2 + 2 / 2] = mul(acc[acc_offset + 4 / 2 + 2 / 2], minors[0 + 4 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], acc[acc_offset + 4 / 2 + 2 / 2 - 1]);
                    minors[0 + 4 / 2 + 2 / 2 - 1] = acc[acc_offset + 4 / 2 + 2 - 1];
                    acc[acc_offset + 4 / 2 + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], minors[0 + 4 / 2 + 2 / 2 - 1]);
                }
                middle[(4 - 2) + 4 / 2 - 1] = sub(middle[(4 - 2) + 4 / 2 - 1], acc[acc_offset + 4 / 2 - 1]);
                middle[(4 - 2) + 0] = sub(middle[(4 - 2) + 0], acc[acc_offset + 0]);
                middle[(4 - 2) + 4 / 2 + 0] = sub(middle[(4 - 2) + 4 / 2 + 0], minors[0 + 4 / 2 + 0]);
                minors[0 + 4 / 2 - 1] = acc[acc_offset + 4 - 1];
                acc[acc_offset + 4 - 1] = sub(middle[(4 - 2) + 4 / 2 - 1], minors[0 + 4 / 2 - 1]);
                minors[0 + 0] = sub(acc[acc_offset + 4 / 2 + 0], minors[0 + 0]);
                acc[acc_offset + 4 / 2 + 0] = sub(middle[(4 - 2) + 0], minors[0 + 0]);
                minors[0 + 0] = add(middle[(4 - 2) + 4 / 2 + 0], minors[0 + 0]);
            }
            {
                // Theta = 4
                middle[(4 - 2) + 0] = add(middle[(8 - 2) + 0], middle[(8 - 2) + 4 / 2 + 0]);
                middle[(4 - 2) + 4 / 2 + 0] = add(middle[(8 - 2) + 8 / 2 + 0], middle[(8 - 2) + 8 / 2 + 4 / 2 + 0]);
                middle[(4 - 2) + 1] = add(middle[(8 - 2) + 1], middle[(8 - 2) + 4 / 2 + 1]);
                middle[(4 - 2) + 4 / 2 + 1] = add(middle[(8 - 2) + 8 / 2 + 1], middle[(8 - 2) + 8 / 2 + 4 / 2 + 1]);
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(middle[(8 - 2) + 0], middle[(8 - 2) + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(middle[(8 - 2) + 8 / 2 + 0], middle[(8 - 2) + 8 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        middle[(8 - 2)] = mul(middle[(8 - 2)], middle[(8 - 2) + 8 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(8 - 2) + 2 / 2] = mul(middle[(8 - 2) + 2 / 2], middle[(8 - 2) + 8 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(8 - 2) + 2 / 2 - 1]);
                    middle[(8 - 2) + 8 / 2 + 2 / 2 - 1] = middle[(8 - 2) + 2 - 1];
                    middle[(8 - 2) + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(8 - 2) + 8 / 2 + 2 / 2 - 1]);
                }
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(middle[(4 - 2) + 0], middle[(4 - 2) + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(middle[(4 - 2) + 4 / 2 + 0], middle[(4 - 2) + 4 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        middle[(4 - 2)] = mul(middle[(4 - 2)], middle[(4 - 2) + 4 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(4 - 2) + 2 / 2] = mul(middle[(4 - 2) + 2 / 2], middle[(4 - 2) + 4 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(4 - 2) + 2 / 2 - 1]);
                    middle[(4 - 2) + 4 / 2 + 2 / 2 - 1] = middle[(4 - 2) + 2 - 1];
                    middle[(4 - 2) + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(4 - 2) + 4 / 2 + 2 / 2 - 1]);
                }
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(middle[(8 - 2) + 4 / 2 + 0], middle[(8 - 2) + 4 / 2 + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(middle[(8 - 2) + 8 / 2 + 4 / 2 + 0], middle[(8 - 2) + 8 / 2 + 4 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        middle[(8 - 2) + 4 / 2] = mul(middle[(8 - 2) + 4 / 2], middle[(8 - 2) + 8 / 2 + 4 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(8 - 2) + 4 / 2 + 2 / 2] = mul(middle[(8 - 2) + 4 / 2 + 2 / 2], middle[(8 - 2) + 8 / 2 + 4 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(8 - 2) + 4 / 2 + 2 / 2 - 1]);
                    middle[(8 - 2) + 8 / 2 + 4 / 2 + 2 / 2 - 1] = middle[(8 - 2) + 4 / 2 + 2 - 1];
                    middle[(8 - 2) + 4 / 2 + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(8 - 2) + 8 / 2 + 4 / 2 + 2 / 2 - 1]);
                }
                middle[(4 - 2) + 4 / 2 - 1] = sub(middle[(4 - 2) + 4 / 2 - 1], middle[(8 - 2) + 4 / 2 - 1]);
                middle[(4 - 2) + 0] = sub(middle[(4 - 2) + 0], middle[(8 - 2) + 0]);
                middle[(4 - 2) + 4 / 2 + 0] = sub(middle[(4 - 2) + 4 / 2 + 0], middle[(8 - 2) + 8 / 2 + 4 / 2 + 0]);
                middle[(8 - 2) + 8 / 2 + 4 / 2 - 1] = middle[(8 - 2) + 4 - 1];
                middle[(8 - 2) + 4 - 1] = sub(middle[(4 - 2) + 4 / 2 - 1], middle[(8 - 2) + 8 / 2 + 4 / 2 - 1]);
                middle[(8 - 2) + 8 / 2 + 0] = sub(middle[(8 - 2) + 4 / 2 + 0], middle[(8 - 2) + 8 / 2 + 0]);
                middle[(8 - 2) + 4 / 2 + 0] = sub(middle[(4 - 2) + 0], middle[(8 - 2) + 8 / 2 + 0]);
                middle[(8 - 2) + 8 / 2 + 0] = add(middle[(4 - 2) + 4 / 2 + 0], middle[(8 - 2) + 8 / 2 + 0]);
            }
            {
                // Theta = 4
                middle[(4 - 2) + 0] = add(acc[acc_offset + 8 / 2 + 0], acc[acc_offset + 8 / 2 + 4 / 2 + 0]);
                middle[(4 - 2) + 4 / 2 + 0] = add(minors[0 + 8 / 2 + 0], minors[0 + 8 / 2 + 4 / 2 + 0]);
                middle[(4 - 2) + 1] = add(acc[acc_offset + 8 / 2 + 1], acc[acc_offset + 8 / 2 + 4 / 2 + 1]);
                middle[(4 - 2) + 4 / 2 + 1] = add(minors[0 + 8 / 2 + 1], minors[0 + 8 / 2 + 4 / 2 + 1]);
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(acc[acc_offset + 8 / 2 + 0], acc[acc_offset + 8 / 2 + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(minors[0 + 8 / 2 + 0], minors[0 + 8 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        acc[acc_offset + 8 / 2] = mul(acc[acc_offset + 8 / 2], minors[0 + 8 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        acc[acc_offset + 8 / 2 + 2 / 2] = mul(acc[acc_offset + 8 / 2 + 2 / 2], minors[0 + 8 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], acc[acc_offset + 8 / 2 + 2 / 2 - 1]);
                    minors[0 + 8 / 2 + 2 / 2 - 1] = acc[acc_offset + 8 / 2 + 2 - 1];
                    acc[acc_offset + 8 / 2 + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], minors[0 + 8 / 2 + 2 / 2 - 1]);
                }
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(middle[(4 - 2) + 0], middle[(4 - 2) + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(middle[(4 - 2) + 4 / 2 + 0], middle[(4 - 2) + 4 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        middle[(4 - 2)] = mul(middle[(4 - 2)], middle[(4 - 2) + 4 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(4 - 2) + 2 / 2] = mul(middle[(4 - 2) + 2 / 2], middle[(4 - 2) + 4 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(4 - 2) + 2 / 2 - 1]);
                    middle[(4 - 2) + 4 / 2 + 2 / 2 - 1] = middle[(4 - 2) + 2 - 1];
                    middle[(4 - 2) + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], middle[(4 - 2) + 4 / 2 + 2 / 2 - 1]);
                }
                {
                    // Theta = 2
                    middle[(2 - 2) + 0] = add(acc[acc_offset + 8 / 2 + 4 / 2 + 0], acc[acc_offset + 8 / 2 + 4 / 2 + 2 / 2 + 0]);
                    middle[(2 - 2) + 2 / 2 + 0] = add(minors[0 + 8 / 2 + 4 / 2 + 0], minors[0 + 8 / 2 + 4 / 2 + 2 / 2 + 0]);
                    {
                        // Theta = 1
                        acc[acc_offset + 8 / 2 + 4 / 2] = mul(acc[acc_offset + 8 / 2 + 4 / 2], minors[0 + 8 / 2 + 4 / 2]);
                    }
                    {
                        // Theta = 1
                        middle[(2 - 2)] = mul(middle[(2 - 2)], middle[(2 - 2) + 2 / 2]);
                    }
                    {
                        // Theta = 1
                        acc[acc_offset + 8 / 2 + 4 / 2 + 2 / 2] = mul(acc[acc_offset + 8 / 2 + 4 / 2 + 2 / 2], minors[0 + 8 / 2 + 4 / 2 + 2 / 2]);
                    }
                    middle[(2 - 2) + 2 / 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], acc[acc_offset + 8 / 2 + 4 / 2 + 2 / 2 - 1]);
                    minors[0 + 8 / 2 + 4 / 2 + 2 / 2 - 1] = acc[acc_offset + 8 / 2 + 4 / 2 + 2 - 1];
                    acc[acc_offset + 8 / 2 + 4 / 2 + 2 - 1] = sub(middle[(2 - 2) + 2 / 2 - 1], minors[0 + 8 / 2 + 4 / 2 + 2 / 2 - 1]);
                }
                middle[(4 - 2) + 4 / 2 - 1] = sub(middle[(4 - 2) + 4 / 2 - 1], acc[acc_offset + 8 / 2 + 4 / 2 - 1]);
                middle[(4 - 2) + 0] = sub(middle[(4 - 2) + 0], acc[acc_offset + 8 / 2 + 0]);
                middle[(4 - 2) + 4 / 2 + 0] = sub(middle[(4 - 2) + 4 / 2 + 0], minors[0 + 8 / 2 + 4 / 2 + 0]);
                minors[0 + 8 / 2 + 4 / 2 - 1] = acc[acc_offset + 8 / 2 + 4 - 1];
                acc[acc_offset + 8 / 2 + 4 - 1] = sub(middle[(4 - 2) + 4 / 2 - 1], minors[0 + 8 / 2 + 4 / 2 - 1]);
                minors[0 + 8 / 2 + 0] = sub(acc[acc_offset + 8 / 2 + 4 / 2 + 0], minors[0 + 8 / 2 + 0]);
                acc[acc_offset + 8 / 2 + 4 / 2 + 0] = sub(middle[(4 - 2) + 0], minors[0 + 8 / 2 + 0]);
                minors[0 + 8 / 2 + 0] = add(middle[(4 - 2) + 4 / 2 + 0], minors[0 + 8 / 2 + 0]);
            }
            middle[(8 - 2) + 8 / 2 - 1] = sub(middle[(8 - 2) + 8 / 2 - 1], acc[acc_offset + 8 / 2 - 1]);
            middle[(8 - 2) + 0] = sub(middle[(8 - 2) + 0], acc[acc_offset + 0]);
            middle[(8 - 2) + 8 / 2 + 0] = sub(middle[(8 - 2) + 8 / 2 + 0], minors[0 + 8 / 2 + 0]);
            middle[(8 - 2) + 1] = sub(middle[(8 - 2) + 1], acc[acc_offset + 1]);
            middle[(8 - 2) + 8 / 2 + 1] = sub(middle[(8 - 2) + 8 / 2 + 1], minors[0 + 8 / 2 + 1]);
            middle[(8 - 2) + 2] = sub(middle[(8 - 2) + 2], acc[acc_offset + 2]);
            middle[(8 - 2) + 8 / 2 + 2] = sub(middle[(8 - 2) + 8 / 2 + 2], minors[0 + 8 / 2 + 2]);
            minors[0 + 8 / 2 - 1] = acc[acc_offset + 8 - 1];
            acc[acc_offset + 8 - 1] = sub(middle[(8 - 2) + 8 / 2 - 1], minors[0 + 8 / 2 - 1]);
            minors[0 + 0] = sub(acc[acc_offset + 8 / 2 + 0], minors[0 + 0]);
            minors[0 + 1] = sub(acc[acc_offset + 8 / 2 + 1], minors[0 + 1]);
            minors[0 + 2] = sub(acc[acc_offset + 8 / 2 + 2], minors[0 + 2]);
            acc[acc_offset + 8 / 2 + 0] = sub(middle[(8 - 2) + 0], minors[0 + 0]);
            minors[0 + 0] = add(middle[(8 - 2) + 8 / 2 + 0], minors[0 + 0]);
            acc[acc_offset + 8 / 2 + 1] = sub(middle[(8 - 2) + 1], minors[0 + 1]);
            minors[0 + 1] = add(middle[(8 - 2) + 8 / 2 + 1], minors[0 + 1]);
            acc[acc_offset + 8 / 2 + 2] = sub(middle[(8 - 2) + 2], minors[0 + 2]);
            minors[0 + 2] = add(middle[(8 - 2) + 8 / 2 + 2], minors[0 + 2]);
            ushort component_index = tau * ((1 << 1) / 2) + 0;
            uint zeta = zeta_arrays[(1 << (2 * 3 + 1 - 1)) - 1 + (component_index >> 1)];
            zeta = component_index & 1 ? sub(0, zeta) : zeta;
            {
                uint mult = mul(minors[0 + 0], zeta);
                acc[acc_offset + 0] = add(acc[acc_offset + 0], mult);
            }
            {
                uint mult = mul(minors[0 + 1], zeta);
                acc[acc_offset + 1] = add(acc[acc_offset + 1], mult);
            }
            {
                uint mult = mul(minors[0 + 2], zeta);
                acc[acc_offset + 2] = add(acc[acc_offset + 2], mult);
            }
            {
                uint mult = mul(minors[0 + 3], zeta);
                acc[acc_offset + 3] = add(acc[acc_offset + 3], mult);
            }
            {
                uint mult = mul(minors[0 + 4], zeta);
                acc[acc_offset + 4] = add(acc[acc_offset + 4], mult);
            }
            {
                uint mult = mul(minors[0 + 5], zeta);
                acc[acc_offset + 5] = add(acc[acc_offset + 5], mult);
            }
            {
                uint mult = mul(minors[0 + 6], zeta);
                acc[acc_offset + 6] = add(acc[acc_offset + 6], mult);
            }
        }
    }
    
    // WRITE OUTPUT
    ushort gamma = t_local >> (5 - 3);
    ushort delta = t_local & (32 / 8 - 1);
    uint shared_write_index_prefix = (e * 1024 + 0 * 1024 + g * 1024 + 0 * 128 + 0 * 1) + w * 32 * 8 + gamma * 32 + delta * 8;
    {
        ushort index = (gamma + 0) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 1) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 2) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 3) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 4) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 5) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 6) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
    {
        ushort index = (gamma + 7) & (8 - 1);
        output[shared_write_index_prefix + index] = acc[index];
    }
}