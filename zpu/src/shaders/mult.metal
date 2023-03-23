
// MULTIPLYING AND ACCUMULATING WITHIN WARPS
// SCHOOLBOOK
// for (ushort s = 0; s < 2; s++)
// uint s = 0;
// {
//     uint zeta = get_zeta(logS + logT + logU - logOrd - 1 + 2, s / 2);
//     for (ushort u0 = 0; u0 < U; u0++)
//     {
//         if (u0 > 0)
//         {
//             array[s * U + (U - u0)] = mul(array[s * U + (U - u0)], zeta);
//         }
//         state = add(mul(state, mult_val), add_val);
//         for (ushort u1 = 0; u1 < U; u1++)
//         {
//             uint mult = mul(array[(s * U + u1 + (S * U - u0)) % (S * U)], state);
//             acc[s * U + u1] = add(acc[s * U + u1], mult);
//         }
//     }
// }
// s = 1;
// {
//     uint zeta = get_zeta(logS + logT + logU - logOrd - 1 + 2, s / 2);
//     for (ushort u0 = 0; u0 < U; u0++)
//     {
//         if (u0 > 0)
//         {
//             array[s * U + (U - u0)] = mul(array[s * U + (U - u0)], zeta);
//         }
//         state = add(mul(state, mult_val), add_val);
//         for (ushort u1 = 0; u1 < U; u1++)
//         {
//             uint mult = mul(array[(s * U + u1 + (S * U - u0)) % (S * U)], state);
//             acc[s * U + u1] = 2 * acc[s * U + u1];
//         }
//     }
// }

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

// // SUMMING ACROSS WARPS
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
//     threadgroup_barrier(mem_flags::mem_threadgroup);
//     if (group != 0)
//     {
//         return;
//     }
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
