
#include "arithmetic.h"

kernel void go(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    uint e [[threadgroup_position_in_grid]],
    ushort w_global [[simdgroup_index_in_threadgroup]],
    ushort t_local [[thread_index_in_simdgroup]])
{
    uint array[8];
    uint acc[8] = {0};
    ushort g = w_global >> 2;
    ushort w = w_global & (4 - 1);
    ushort tau = w * 32 + t_local;
    uint global_read_index_prefix = e * 2048 + 0 * 2048 + g * 1024 + 0 * 128 + tau * 1;
    
    for (ushort f = 0; f < 1; f++)
    {
        // READ INPUT
        for (ushort s = 0; s < 8; s++)
        {
            array[s] = uint(input[global_read_index_prefix + 0 * 2048 + 0 * 2048 + 0 * 1024 + s * 128 + 0 * 1]);
        }
        global_read_index_prefix += 0 * 2048 + 1 * 2048 + 0 * 1024 + 0 * 128 + 0 * 1;
        
        // DECOMPOSE WITH CHAIN
        {
            // DECOMPOSE WITHIN THREADS 0
            for (ushort k = 0; k < 3; k++)
            {
                ushort s_bound = 1 << (3 - (k + 1));
                for (ushort i = 0; i < (1 << k); i++)
                {
                    uint zeta = zetas((1 << k) - 1 + i);
                    ushort lo_index_prefix = (2 * i) * s_bound;
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
            for (ushort j = 1; j <= 2; j++)
            {
                ushort log_power_S_to_j = j * 3;
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
                    for (ushort s = 0; s < 8; s++)
                    {
                        if (s == i)
                        {
                            continue;
                        }
                        ushort index = g * 1024 + (r * 8 + s) * Dj + i * Tj + t;
                        shared[index] = array[s];
                    }
                    threadgroup_barrier(metal::mem_flags::mem_threadgroup);
                    for (ushort s = 0; s < 8; s++)
                    {
                        if (s == i)
                        {
                            continue;
                        }
                        ushort index = g * 1024 + (r * 8 + i) * Dj + s * Tj + t;
                        array[s] = shared[index];
                    }
                }
                else
                // TRANSPOSE ACROSS THREADS
                {
                    for (ushort s = 0; s < 8; s++)
                    {
                        ushort index = s ^ i;
                        array[index] = metal::simd_shuffle_xor(array[index], s * Tj);
                    }
                }
                
                // DECOMPOSE WITHIN THREADS j > 0
                ushort r_new = r * 8 + i;
                ushort k_bound = j < 2 ? 3 : 1;
                for (ushort k = 0; k < k_bound; k++)
                {
                    ushort s_bound = 1 << (3 - (k + 1));
                    for (ushort i_new = 0; i_new < (1 << k); i_new++)
                    {
                        ushort component_index = r_new * (1 << k) + i_new;
                        uint zeta = zetas((1 << (j * 3 + k)) - 1 + component_index);
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
            }
        }
        
        // COLLECT COMPONENTS INTO THREADS
        {
            ushort t = tau & (2 - 1);
            for (ushort mask = 0; mask < 2; mask++)
            {
                ushort other = t ^ mask;
                ushort scaled_other = other * (8 / 2);
                for (ushort v = 0; v < 8 / 2; v++)
                {
                    ushort index = scaled_other + v;
                    array[index] = metal::simd_shuffle_xor(array[index], mask);
                }
            }
        }
        
        // SCHOOLBOOK MULTIPLICATION
        uint minors[2 * 8 / (1 << 1)] = {3614796953, 1208427060, 1889015752, 3198863462, 3614796953, 1208427060, 1889015752,3198863462};
        for (ushort u = 0; u < (1 << 1) / 2; u++)
        {
            ushort component_index = tau * ((1 << 1) / 2) + u;
            uint zeta = zetas((1 << (2 * 3 + 1 - 1)) - 1 + (component_index >> 1));
            zeta = component_index & 1 ? sub(0, zeta) : zeta;
            for (ushort s1 = 0; s1 < 4; s1++)
            {
                for (ushort t1 = 0; t1 < 2; t1++)
                {
                    ushort coef_index = s1 * 2 + t1;
                    if (coef_index > 0)
                    {
                        ushort new_bottom_index = t1 == 0 ? (u + 1) * (8 / (1 << 1)) - s1 : (2 - t1) * (8 / 2) + (u + 1) * 4 - (s1 + 1);
                        array[new_bottom_index] = mul(array[new_bottom_index], zeta);
                    }
                    for (ushort s2 = 0; s2 < 4; s2++)
                    {
                        for (ushort t2 = 0; t2 < 2; t2++)
                        {
                            ushort array_index = t2 * (8 / 2) + u * 4 + s2;
                            ushort delta = (coef_index + (s2 * 2 + t2)) % (2 * 4);
                            ushort acc_index = u * (2 * 4) + delta;
                            acc[acc_index] = add(acc[acc_index], mul(array[array_index], minors[coef_index]));
                        }
                    }
                }
            }
        }
    }
    
    // WRITE OUTPUT
    ushort gamma = t_local >> (5 - 3);
    ushort delta = t_local & (32 / 8 - 1);
    ushort global_write_index_prefix = (e * 2048 + 0 * 2048 + g * 1024 + 0 * 128 + 0 * 1) + w * 32 * 8 + gamma * 32 + delta * 8;
    for (ushort s = 0; s < 8; s++)
    {
        ushort index = (gamma + s) & (8 - 1);
        output[global_write_index_prefix + index] = acc[index];
    }
}