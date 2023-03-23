
#include "arithmetic.h"

kernel void go(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    uint e [[threadgroup_position_in_grid]],
    ushort w_global [[simdgroup_index_in_threadgroup]],
    ushort t_local [[thread_index_in_simdgroup]])
{
    uint array[4];
    ushort w = w_global & ((1 << 1) - 1);
    ushort tau = w * 32 + t_local;
    ushort g = w_global / 2;
    uint global_read_index_prefix = e * 256 + 0 * 256 + g * 256 + 0 * 64 + tau * 1 + 0 * 1;
    
    for (ushort f = 0; f < 1; f++)
    {
        
        // READ INPUT
        for (ushort s = 0; s < 4; s++)
        {
            for (ushort u = 0; u < 1; u++)
            {
                uint global_read_index = global_read_index_prefix + 0 * 256 + 0 * 256 + 0 * 256 + s * 64 + 0 * 1 + u * 1;
                array[s * 1 + u] = uint(input[global_read_index]);
            }
        }
        global_read_index_prefix += 0 * 256 + 1 * 256 + 0 * 256 + 0 * 64 + 0 * 1 + 0 * 1;
        
        // DECOMPOSE WITH CHAIN
        ushort Dj = 256;
        ushort Tj = 64;
        ushort r = 0;
        ushort i = 0;
        ushort t = tau;
        
        // DECOMPOSE WITHIN THREADS 0
        for (ushort k = 0; k < 2; k++)
        {
            ushort s_bound = 4 / (1 << (k + 1));
            for (ushort i = 0; i < (1 << k); i++)
            {
                uint zeta = zetas((1 << k) - 1 + i);
                ushort lo_index_prefix = (2 * i) * s_bound;
                ushort hi_index_prefix = lo_index_prefix + s_bound;
                for (ushort s = 0; s < s_bound; s++)
                {
                    {
                        ushort hi_index = (hi_index_prefix + s) * 1 + 0;
                        uint mult = mul(array[hi_index], zeta);
                        ushort lo_index = (lo_index_prefix + s) * 1 + 0;
                        array[hi_index] = sub(array[lo_index], mult);
                        array[lo_index] = add(array[lo_index], mult);
                    }
                }
            }
        }
        for (ushort j = 1; j <= 2; j++)
        {
            Dj /= 4;
            Tj /= 4;
            i = (tau / Tj) & (4 - 1);
            t = tau & (Tj - 1);
            
            if (32 < Dj)
            // TRANSPOSE ACROSS WARPS
            {
                for (ushort s = 0; s < 4; s++)
                {
                    ushort index = g * 256 + (r * 4 + s) * Dj + i * Tj + t;
                    shared[index] = array[s];
                }
                threadgroup_barrier(metal::mem_flags::mem_threadgroup);
                for (ushort s = 0; s < 4; s++)
                {
                    ushort index = g * 256 + (r * 4 + i) * Dj + s * Tj + t;
                    array[s] = shared[index];
                }
            }
            else
            // TRANSPOSE ACROSS THREADS
            {
                for (ushort s = 0; s < 4; s++)
                {
                    ushort mask = s * Tj;
                    ushort index = s ^ i;
                    array[index] = metal::simd_shuffle_xor(array[index], mask);
                }
            }
            
            // DECOMPOSE WITHIN THREADS j
            ushort r_new = r * 4 + i;
            ushort k_bound = j < 2 ? 2 : 2;
            for (ushort k = 0; k < k_bound; k++)
            {
                ushort s_bound = 4 / (1 << (k + 1));
                for (ushort i_new = 0; i_new < (1 << k); i_new++)
                {
                    ushort component_index = r_new * (1 << k) + i_new;
                    uint zeta = zetas((1 << (j * 2 + k)) - 1 + component_index);
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
        
        // COLLECT IRREDUCIBLES INTO THREADS
        uint tmp[4] = {0};
        for (ushort mask = 0; mask < 4; mask++)
        {
            for (ushort v = 0; v < 4 / 4; v++)
            {
                ushort other = t ^ mask;
                ushort index = other * (4 / 4) + v;
                tmp[index] = metal::simd_shuffle_xor(array[index], mask);
            }
        }
        for (ushort mask = 0; mask < Tj; mask++)
        {
            for (ushort v = 0; v < 4 / Tj; v++)
            {
                ushort other = t ^ mask;
                ushort new_index = v * Tj + other;
                ushort old_index = other * (4 / Tj) + v;
                array[old_index] = tmp[new_index];
            }
        }
        ushort u = 0;
        uint middle[4] = {0};
        uint minors[4] = {1, 0, 0, 0};
        ushort component_index = (r * Tj + t) * ((1 << 2) / Tj) + u;
        ushort zeta_index = (1 << ((2 - 1) * 2 + 2 + 1)) - 1 + component_index / 2;
        uint zeta = zetas(zeta_index);
        if (component_index % 2) { zeta = sub(0, zeta); }
        // Theta = 4
        middle[(4 - 2) + 0] = array[0 + 4 / 2 + 0];
        middle[(4 - 2) + 1] = array[0 + 4 / 2 + 1];
        {
            // Theta = 2
            middle[(2 - 2) + 0] = array[0 + 2 / 2 + 0];
            {
                // Theta = 1
                array[0] = mul(array[0], minors[0]);
                // Theta = 1
                middle[(2 - 2)] = mul(middle[(2 - 2)], minors[0 + 2 / 2]);
            }
            array[0 + 2 / 2 + 0] = array[0 + 2 / 2 + 0] + array[0 + 0];
            minors[0 + 2 / 2 + 0] = minors[0 + 2 / 2 + 0] + minors[0 + 0];
            // Theta = 1
            array[0 + 2 / 2] = mul(array[0 + 2 / 2], minors[0 + 2 / 2]);
            array[0 + 2 / 2 + 0] = sub(array[0 + 2 / 2 + 0], middle[(2 - 2) + 0]);
            array[0 + 2 / 2 + 0] = sub(array[0 + 2 / 2 + 0], array[0 + 0]);
            array[0 + 0] = add(array[0 + 0], mul(middle[(2 - 2) + 0], zeta));
            // Theta = 2
            middle[(2 - 2) + 0] = middle[(4 - 2) + 2 / 2 + 0];
            {
                // Theta = 1
                middle[(4 - 2)] = mul(middle[(4 - 2)], minors[0 + 4 / 2]);
                // Theta = 1
                middle[(2 - 2)] = mul(middle[(2 - 2)], minors[0 + 4 / 2 + 2 / 2]);
            }
            middle[(4 - 2) + 2 / 2 + 0] = middle[(4 - 2) + 2 / 2 + 0] + middle[(4 - 2) + 0];
            minors[0 + 4 / 2 + 2 / 2 + 0] = minors[0 + 4 / 2 + 2 / 2 + 0] + minors[0 + 4 / 2 + 0];
            // Theta = 1
            middle[(4 - 2) + 2 / 2] = mul(middle[(4 - 2) + 2 / 2], minors[0 + 4 / 2 + 2 / 2]);
            middle[(4 - 2) + 2 / 2 + 0] = sub(middle[(4 - 2) + 2 / 2 + 0], middle[(2 - 2) + 0]);
            middle[(4 - 2) + 2 / 2 + 0] = sub(middle[(4 - 2) + 2 / 2 + 0], middle[(4 - 2) + 0]);
            middle[(4 - 2) + 0] = add(middle[(4 - 2) + 0], mul(middle[(2 - 2) + 0], zeta));
        }
        array[0 + 4 / 2 + 0] = array[0 + 4 / 2 + 0] + array[0 + 0];
        minors[0 + 4 / 2 + 0] = minors[0 + 4 / 2 + 0] + minors[0 + 0];
        array[0 + 4 / 2 + 1] = array[0 + 4 / 2 + 1] + array[0 + 1];
        minors[0 + 4 / 2 + 1] = minors[0 + 4 / 2 + 1] + minors[0 + 1];
        // Theta = 2
        middle[(2 - 2) + 0] = array[0 + 4 / 2 + 2 / 2 + 0];
        {
            // Theta = 1
            array[0 + 4 / 2] = mul(array[0 + 4 / 2], minors[0 + 4 / 2]);
            // Theta = 1
            middle[(2 - 2)] = mul(middle[(2 - 2)], minors[0 + 4 / 2 + 2 / 2]);
        }
        array[0 + 4 / 2 + 2 / 2 + 0] = array[0 + 4 / 2 + 2 / 2 + 0] + array[0 + 4 / 2 + 0];
        minors[0 + 4 / 2 + 2 / 2 + 0] = minors[0 + 4 / 2 + 2 / 2 + 0] + minors[0 + 4 / 2 + 0];
        // Theta = 1
        array[0 + 4 / 2 + 2 / 2] = mul(array[0 + 4 / 2 + 2 / 2], minors[0 + 4 / 2 + 2 / 2]);
        array[0 + 4 / 2 + 2 / 2 + 0] = sub(array[0 + 4 / 2 + 2 / 2 + 0], middle[(2 - 2) + 0]);
        array[0 + 4 / 2 + 2 / 2 + 0] = sub(array[0 + 4 / 2 + 2 / 2 + 0], array[0 + 4 / 2 + 0]);
        array[0 + 4 / 2 + 0] = add(array[0 + 4 / 2 + 0], mul(middle[(2 - 2) + 0], zeta));
        array[0 + 4 / 2 + 0] = sub(array[0 + 4 / 2 + 0], middle[(4 - 2) + 0]);
        array[0 + 4 / 2 + 0] = sub(array[0 + 4 / 2 + 0], array[0 + 0]);
        array[0 + 4 / 2 + 1] = sub(array[0 + 4 / 2 + 1], middle[(4 - 2) + 1]);
        array[0 + 4 / 2 + 1] = sub(array[0 + 4 / 2 + 1], array[0 + 1]);
        array[0 + 0] = add(array[0 + 0], mul(middle[(4 - 2) + 0], zeta));
        array[0 + 1] = add(array[0 + 1], mul(middle[(4 - 2) + 1], zeta));
        
        // WRITE OUTPUT
        uint global_write_index_prefix = e * 256 + 0 * 256 + g * 256 + 0 * 64 + 0 * 1 + 0 * 1;
        ushort tau_lower = tau & (Tj - 1);
        for (ushort s = 0; s < 4; s++)
        {
            uint index = (r * Tj + tau_lower) * 4 + s;
            output[index] = array[s];
        }
    }
}