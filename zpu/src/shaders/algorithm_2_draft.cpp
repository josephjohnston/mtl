
template <ushort S, ushort T_0, ushort T_J, ushort J, ushort K>
void Algorithm_2_draft(
    device uchar *input,
    threadgroup uint *shared,
    device uint *output,
    const uint e,
    const short w_global,
    const ushort t_local,
    constant const Params &params)
{
    const ushort F = params.F;
    const ushort G = params.G;

    ushort D[J + 1];
    ushort T[J + 1];
    ushort L[J + 1];
    for (ushort j = 0; j <= J; j++)
    {
        if (j == 0)
        {
            T[0] = T_0;
            D[0] = S * T[0];
            L[0] = 0;
        }
        else
        {
            D[j] = D[j - 1] / S;
            T[j] = D[j] / S;
            L[j] = L[j - 1] + log(S);
        }
    }
    uint array[S];
    ushort W = T[0] / params.X;
    ushort w = w_global & (W - 1);
    ushort tau = w * params.X + t_local;
    ushort g = w_global / W;
    uint global_read_index_prefix = get_index(e, F, 0, G, g, S, 0, T[0], tau, 1, 0);

    for (ushort f = 0; f < F; f++)
    {

        // READ INPUT
        for (ushort s = 0; s < S; s++)
        {
            uint global_read_index = global_read_index_prefix + get_index(0, F, 0, G, 0, S, s, T[0], 0, 1, 0);
            array[s] = uint(input[global_read_index]);
        }
        global_read_index_prefix += get_index(0, F, 1, G, 0, S, 0, T[0], 0, 1, 0);

        // DECOMPOSE WITH CHAIN
        ushort r;
        ushort i;
        ushort s;
        ushort t;
        for (ushort j = 0; j <= J; j++)
        {
            r = tau / T[j - 1];
            i = (tau / T[j]) & (S - 1);
            s = (tau / T[j]) & (S - 1);
            t = tau & (T[j] - 1);

            // TRANSPOSE ACROSS WARPS
            if (0 < j && params.X < T[j - 1])
            {
                for (ushort i_iter = 0; i_iter < S; i_iter++)
                {
                    if (i_iter == s)
                    {
                        continue;
                    }
                    ushort index = g * D[0] + r * D[j - 1] + i_iter * T[j - 1] + s * T[j] + t;
                    shared[index] = array[i_iter];
                }
                threadgroup_barrier(metal::mem_flags::mem_threadgroup);
                for (ushort s_iter = 0; s_iter < S; s_iter++)
                {
                    if (s_iter == i)
                    {
                        continue;
                    }
                    ushort index = g * D[0] + (r * S + i) * D[j] + s_iter * T[j] + t;
                    array[s_iter] = shared[index];
                }
            }

            // TRANSPOSE ACROSS THREADS
            if (0 < j && T[j - 1] <= params.X)
            {
                for (ushort s_iter = 0; s_iter < S; s_iter++)
                {
                    ushort mask = s_iter << log(T[j]);
                    ushort index = s_iter ^ i;
                    array[index] = metal::simd_shuffle_xor(array[index], mask);
                    // ushort sigma = (r * S[j] + s_iter) * T[j] + t;
                    // array[index] = metal::simd_shuffle(array[index], sigma);
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
                    uint zeta = zetas((1 << (L[j] + k)) - 1 + component_index);
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

        r = tau / T[J];

        // COLLECT IRREDUCIBLES INTO THREADS
        for (ushort mask = 0; mask < T[J]; mask++)
        {
            for (ushort v = 0; v < S / T[J]; v++)
            {
                ushort other = t ^ mask;
                ushort index = other * (S / T[J]) + v;
                array[index] = metal::simd_shuffle_xor(array[index], mask);
            }
        }

        // MULTIPLY IRREDUCIBLES WITHIN THREADS
        uint tmp[S] = {0};
        {
            uint mult_val = 2091658123;
            uint add_val = 1523138830;
            uint state = 1;
            uint minors[T_J * S / (1 << K)] = {1, 0, 0, 0};
            // {3614796953, 1208427060, 1889015752, 3198863462};
            // {1, 0, 0, 0};

            ushort s_bound = S / (1 << K);

            for (ushort u = 0; u < (1 << K) / T[J]; u++)
            {
                uint acc[T_J * S / (1 << K)] = {0};

                ushort component_index = (r * T[J] + t) * ((1 << K) / T[J]) + u;
                ushort zeta_index = (1 << ((J - 1) * log(S) + K + 1)) - 1 + component_index / 2;
                uint zeta = zetas(zeta_index);
                if (component_index % 2)
                {
                    zeta = sub(0, zeta);
                }

                for (ushort s1 = 0; s1 < s_bound; s1++)
                {
                    for (ushort t1 = 0; t1 < T[J]; t1++)
                    {
                        ushort coef_index = s1 * T[J] + t1;
                        if (coef_index > 0)
                        {
                            ushort new_bottom_index = (T[J] - t1) * (S / T[J]) + u * s_bound + (S / (1 << K) - (s1 + 1));
                            array[new_bottom_index] = mul(array[new_bottom_index], zeta);
                        }
                        // state = add(mul(state, mult_val), add_val);

                        for (ushort s2 = 0; s2 < s_bound; s2++)
                        {
                            for (ushort t2 = 0; t2 < T[J]; t2++)
                            {
                                ushort array_index = t2 * (S / T[J]) + u * s_bound + s2;
                                ushort acc_index = t2 * s_bound + s2;
                                ushort off_index = (acc_index + coef_index) % S;
                                acc[off_index] = add(acc[off_index], mul(array[array_index], minors[coef_index]));
                            }
                        }
                    }
                }

                for (ushort s2 = 0; s2 < s_bound; s2++)
                {
                    for (ushort t2 = 0; t2 < T[J]; t2++)
                    {
                        ushort array_index = t2 * (S / T[J]) + u * s_bound + s2;
                        ushort acc_index = t2 * s_bound + s2;
                        array[array_index] = acc[acc_index];
                    }
                }
            }
            for (ushort mask = 0; mask < T[J]; mask++)
            {
                for (ushort v = 0; v < S / T[J]; v++)
                {
                    ushort other = t ^ mask;
                    ushort new_index = v * T[J] + other;
                    ushort old_index = other * (S / T[J]) + v;
                    tmp[old_index] = array[new_index];
                }
            }
        }

        // WRITE OUTPUT
        uint global_write_index_prefix = 0;
        // get_index_2(e, F, f, G, g, S, 0, T[0], 0);
        // ushort r = tau / T[CHAIN_LENGTH - 1];
        ushort tau_lower = tau & (T[J] - 1);
        for (ushort i_iter = 0; i_iter < S; i_iter++)
        {
            // uint index = global_write_index_prefix + r * D[J] + i_iter * T[J] + tau_lower;
            uint index = (r * T[J] + tau_lower) * S + i_iter;
            output[index] = tmp[i_iter];
        }
    }
}