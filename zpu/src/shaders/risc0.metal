
#include <metal_stdlib>
#include "fp.h"

using namespace metal;

kernel void K1_1f(
    device uchar *input,
    device Fp *output,
    constant uint &N,
    uint gid [[thread_position_in_grid]])
{
    Fp lo = Fp(input[gid]);
    Fp hi = Fp(input[gid + N / 2]);
    Fp mult = hi.mul2_0();
    output[gid] = lo.sub(mult);
    output[gid + N / 2] = hi.add(mult);
}

kernel void K8_con_1f(
    device uchar *input,
    device Fp *output,
    constant uint &N,
    uint gid [[thread_position_in_grid]])
{
    gid *= 8;
    Fp lo1 = Fp(uint(input[gid]));
    Fp hi1 = Fp(uint(input[gid + N / 2]));
    Fp lo2 = Fp(uint(input[gid + 1]));
    Fp hi2 = Fp(uint(input[gid + 1 + N / 2]));
    Fp lo3 = Fp(uint(input[gid + 2]));
    Fp hi3 = Fp(uint(input[gid + 2 + N / 2]));
    Fp lo4 = Fp(uint(input[gid + 3]));
    Fp hi4 = Fp(uint(input[gid + 3 + N / 2]));
    Fp lo5 = Fp(uint(input[gid + 4]));
    Fp hi5 = Fp(uint(input[gid + 4 + N / 2]));
    Fp lo6 = Fp(uint(input[gid + 5]));
    Fp hi6 = Fp(uint(input[gid + 5 + N / 2]));
    Fp lo7 = Fp(uint(input[gid + 6]));
    Fp hi7 = Fp(uint(input[gid + 6 + N / 2]));
    Fp lo8 = Fp(uint(input[gid + 7]));
    Fp hi8 = Fp(uint(input[gid + 7 + N / 2]));

    Fp mult1 = hi1.mul2_0();
    Fp mult2 = hi2.mul2_0();
    Fp mult3 = hi3.mul2_0();
    Fp mult4 = hi4.mul2_0();
    Fp mult5 = hi5.mul2_0();
    Fp mult6 = hi6.mul2_0();
    Fp mult7 = hi7.mul2_0();
    Fp mult8 = hi8.mul2_0();

    output[gid] = lo1.sub(mult1);
    output[gid + N / 2] = hi1.add(mult1);
    output[gid + 1] = lo2.sub(mult2);
    output[gid + 1 + N / 2] = hi2.add(mult2);
    output[gid + 2] = lo3.sub(mult3);
    output[gid + 2 + N / 2] = hi3.add(mult3);
    output[gid + 3] = lo4.sub(mult4);
    output[gid + 3 + N / 2] = hi4.add(mult4);
    output[gid + 4] = lo5.sub(mult5);
    output[gid + 4 + N / 2] = hi5.add(mult5);
    output[gid + 5] = lo6.sub(mult6);
    output[gid + 5 + N / 2] = hi6.add(mult6);
    output[gid + 6] = lo7.sub(mult7);
    output[gid + 6 + N / 2] = hi7.add(mult7);
    output[gid + 7] = lo8.sub(mult8);
    output[gid + 7 + N / 2] = hi8.add(mult8);
}

kernel void K8_off_1f(
    device uchar *input,
    device Fp *output,
    constant uint &N,
    uint gid [[thread_position_in_grid]])
{
    // gid *= 8;
    Fp lo1 = Fp(uint(input[gid]));                      // 0, 1, 2
    Fp hi1 = Fp(uint(input[gid + N / 2]));              // 64, 65, 66
    Fp lo2 = Fp(uint(input[gid + 1 * N / 16]));         // 32, 33, 34
    Fp hi2 = Fp(uint(input[gid + 1 * N / 16 + N / 2])); // 96, 97, 98
    Fp lo3 = Fp(uint(input[gid + 2 * N / 16]));
    Fp hi3 = Fp(uint(input[gid + 2 * N / 16 + N / 2]));
    Fp lo4 = Fp(uint(input[gid + 3 * N / 16]));
    Fp hi4 = Fp(uint(input[gid + 3 * N / 16 + N / 2]));
    Fp lo5 = Fp(uint(input[gid + 4 * N / 16]));
    Fp hi5 = Fp(uint(input[gid + 4 * N / 16 + N / 2]));
    Fp lo6 = Fp(uint(input[gid + 5 * N / 16]));
    Fp hi6 = Fp(uint(input[gid + 5 * N / 16 + N / 2]));
    Fp lo7 = Fp(uint(input[gid + 6 * N / 16]));
    Fp hi7 = Fp(uint(input[gid + 6 * N / 16 + N / 2]));
    Fp lo8 = Fp(uint(input[gid + 7 * N / 16]));
    Fp hi8 = Fp(uint(input[gid + 7 * N / 16 + N / 2]));

    Fp mult1 = hi1.mul2_0();
    Fp mult2 = hi2.mul2_0();
    Fp mult3 = hi3.mul2_0();
    Fp mult4 = hi4.mul2_0();
    Fp mult5 = hi5.mul2_0();
    Fp mult6 = hi6.mul2_0();
    Fp mult7 = hi7.mul2_0();
    Fp mult8 = hi8.mul2_0();

    output[gid] = lo1.sub(mult1);
    output[gid + N / 2] = hi1.add(mult1);
    output[gid + 1 * N / 16] = lo2.sub(mult2);
    output[gid + 1 * N / 16 + N / 2] = hi2.add(mult2);
    output[gid + 2 * N / 16] = lo3.sub(mult3);
    output[gid + 2 * N / 16 + N / 2] = hi3.add(mult3);
    output[gid + 3 * N / 16] = lo4.sub(mult4);
    output[gid + 3 * N / 16 + N / 2] = hi4.add(mult4);
    output[gid + 4 * N / 16] = lo5.sub(mult5);
    output[gid + 4 * N / 16 + N / 2] = hi5.add(mult5);
    output[gid + 5 * N / 16] = lo6.sub(mult6);
    output[gid + 5 * N / 16 + N / 2] = hi6.add(mult6);
    output[gid + 6 * N / 16] = lo7.sub(mult7);
    output[gid + 6 * N / 16 + N / 2] = hi7.add(mult7);
    output[gid + 7 * N / 16] = lo8.sub(mult8);
    output[gid + 7 * N / 16 + N / 2] = hi8.add(mult8);
}

kernel void K8_off_1f_v2(
    device uchar *input,
    device Fp *output,
    constant uint &N,
    uint gid [[thread_position_in_grid]])
{
    Fp hi1 = Fp(uint(input[gid + N / 2]));
    Fp mult1 = hi1.mul2_0();
    Fp hi2 = Fp(uint(input[gid + 1 * N / 16 + N / 2]));
    Fp mult2 = hi2.mul2_0();
    Fp hi3 = Fp(uint(input[gid + 2 * N / 16 + N / 2]));
    Fp mult3 = hi3.mul2_0();
    Fp hi4 = Fp(uint(input[gid + 3 * N / 16 + N / 2]));
    Fp mult4 = hi4.mul2_0();
    Fp hi5 = Fp(uint(input[gid + 4 * N / 16 + N / 2]));
    Fp mult5 = hi5.mul2_0();
    Fp hi6 = Fp(uint(input[gid + 5 * N / 16 + N / 2]));
    Fp mult6 = hi6.mul2_0();
    Fp hi7 = Fp(uint(input[gid + 6 * N / 16 + N / 2]));
    Fp mult7 = hi7.mul2_0();
    Fp hi8 = Fp(uint(input[gid + 7 * N / 16 + N / 2]));
    Fp mult8 = hi8.mul2_0();

    Fp lo1 = Fp(uint(input[gid]));
    Fp lo2 = Fp(uint(input[gid + 1 * N / 16]));
    Fp lo3 = Fp(uint(input[gid + 2 * N / 16]));
    Fp lo4 = Fp(uint(input[gid + 3 * N / 16]));
    Fp lo5 = Fp(uint(input[gid + 4 * N / 16]));
    Fp lo6 = Fp(uint(input[gid + 5 * N / 16]));
    Fp lo7 = Fp(uint(input[gid + 6 * N / 16]));
    Fp lo8 = Fp(uint(input[gid + 7 * N / 16]));

    output[gid] = lo1.sub(mult1);
    output[gid + N / 2] = hi1.add(mult1);
    output[gid + 1 * N / 16] = lo2.sub(mult2);
    output[gid + 1 * N / 16 + N / 2] = hi2.add(mult2);
    output[gid + 2 * N / 16] = lo3.sub(mult3);
    output[gid + 2 * N / 16 + N / 2] = hi3.add(mult3);
    output[gid + 3 * N / 16] = lo4.sub(mult4);
    output[gid + 3 * N / 16 + N / 2] = hi4.add(mult4);
    output[gid + 4 * N / 16] = lo5.sub(mult5);
    output[gid + 4 * N / 16 + N / 2] = hi5.add(mult5);
    output[gid + 5 * N / 16] = lo6.sub(mult6);
    output[gid + 5 * N / 16 + N / 2] = hi6.add(mult6);
    output[gid + 6 * N / 16] = lo7.sub(mult7);
    output[gid + 6 * N / 16 + N / 2] = hi7.add(mult7);
    output[gid + 7 * N / 16] = lo8.sub(mult8);
    output[gid + 7 * N / 16 + N / 2] = hi8.add(mult8);
}

kernel void K1_2f(
    device uchar *input,
    device Fp *output,
    constant uint &N,
    uint gid [[thread_position_in_grid]])
{
    Fp lo = Fp(input[gid]);              // 0, 1, 2
    Fp hi = Fp(input[gid + N / 4]);      // 16,17,18
    Fp lo2 = Fp(input[gid + 2 * N / 4]); // 32,33,34
    Fp hi2 = Fp(input[gid + 3 * N / 4]); // 48,49,50
    Fp mult = hi.mul2_0();
    Fp mult2 = hi.mul2_0();
    output[gid] = lo.sub(mult);
    output[gid + N / 4] = hi.add(mult);
    output[gid + 2 * N / 4] = lo2.sub(mult2);
    output[gid + 3 * N / 4] = hi2.add(mult2);
}

template <uint N, typename T, ushort U>
void load(
    device vec<T, U> *input,
    device uint *output,
    uint gid)
{
    vec<T, U> input_hi_vals = input[gid + N / (2 * U)];
    vec<T, U> input_lo_vals = input[gid];
    // vec<uint, U> lo_vals;
    // for (uchar i = 0; i < U; i++)
    // {
    //     // Fp hi = Fp(input_hi_vals[i]);
    //     // Fp lo = Fp(input_lo_vals[i]);
    //     // Fp mult = hi.mul2_0();
    //     // hi_vals[i] = uint(input_hi_vals[i]);
    //     // uint(lo.add(mult));
    //     lo_vals[i] = uint(input_lo_vals[i]);
    //     // uint(lo.sub(mult));
    // }

    for (uchar i = 0; i < U; i++)
    {
        output[U * gid + N / 2 + i] = input_hi_vals[i];
        output[U * gid + i] = input_lo_vals[i];
    }

    // Fp mult0 = hi0.mul2_0();
    // Fp mult1 = hi1.mul2_0();

    // output[gid + N / 2] = hi0.add(mult0);
    // output[gid] = lo0.sub(mult0);
}

// template <uint N>
// T = uchar, ushort
// template <uint N, typename T, ushort V, T W>
// thread vec<T, V> Kappa(
//     device T *input,
//     // device uint *output,
//     uint id)
// {
//     T hi_vals[V];
//     T lo_vals[V];
//     // for (ushort i = 0; i < V; i++)
//     // {
//     //     // hi_vals[i] = input[N / 2 + id + i * V];
//     //     // lo_vals[i] = input[id + i * V];
//     //     // T mult = hi_vals[i] * W;
//     //     // hi_vals[i] = lo_vals[i] - hi_vals[i];
//     //     // lo_vals[i] = lo_vals[i] + hi_vals[i];
//     //     lo_vals[i] = i;
//     // }
//     return lo_vals;
// }

template <uint D, uint S>
void Kappa(
    device uchar *input,
    device uint *output,
    uint g,
    uint t)
{
    uint vals[S];
    // READING INPUT
    for (ushort s = 0; s < S; s++)
    {
        vals[s] = input[g * D + s * (D / S) + t];
    }
    // READING CONSTANTS
    // REDUCING IN THREAD
    for (ushort k = 0; k < ushort(log2(half(S))); k++)
    {
        for (uint s = 0; s < S / (1 << (k + 1)); s++)
        {
            for (ushort i = 0; i < (1 << k); i++)
            {
                uint hi_index = (2 * i + 1) * (S / (1 << (k + 1))) + s;
                uint lo_index = (2 * i) * (S / (1 << (k + 1))) + s;
                uint mult = vals[hi_index] * (1 << 20);
                vals[hi_index] = vals[lo_index] + mult;
                vals[lo_index] = vals[lo_index] - mult;
            }
        }
    }
    // REDUCING ACROSS THREADS
    // MULTIPLYING
    // WRITING
    for (ushort s = 0; s < S; s++)
    {
        output[g * D + s * (D / S) + t] = vals[s];
    }
}

// constant const uint S = 1 << 6;
kernel void go(
    device uchar *input,
    device uint *output,
    // constant uint &D,
    uint g [[threadgroup_position_in_grid]],
    uint t [[thread_position_in_threadgroup]])
{
    Kappa<1 << 14, 1 << 6>(input, output, g, t);
    // // load<1 << 29, uint, 1 << 2>(input, output, gid);
    // // thread vec<uint, 1> a = Kappa<64, uint, 1, 1 << 20>(input, gid);

    // // uint hi_vals[V];
    // // uint lo_vals[V];
    // uint vals[S];
    // // READING INPUT
    // for (ushort s = 0; s < S; s++)
    // {
    //     // hi_vals[i] = input[D / 2 + gid + i * stride];
    //     // lo_vals[i] = input[gid + i * stride];
    //     vals[s] = input[g * D + s * (D / S) + t];
    // }
    // // READING CONSTANTS
    // // REDUCING IN THREAD
    // for (ushort k = 0; k < ushort(log2(half(S))); k++)
    // {
    //     for (uint s = 0; s < S / (1 << (k + 1)); s++)
    //     {
    //         for (ushort i = 0; i < (1 << k); i++)
    //         {
    //             uint hi_index = (2 * i + 1) * (S / (1 << (k + 1))) + s;
    //             uint lo_index = (2 * i) * (S / (1 << (k + 1))) + s;
    //             uint mult = vals[hi_index] * (1 << 20);
    //             vals[hi_index] = vals[lo_index] + mult;
    //             vals[lo_index] = vals[lo_index] - mult;
    //         }
    //     }
    // }
    // // REDUCING ACROSS THREADS
    // // MULTIPLYING
    // // WRITING
    // for (ushort s = 0; s < S; s++)
    // {
    //     // output[N / 2 + gid + i * stride] = hi_vals[i];
    //     // output[gid + i * stride] = lo_vals[i];
    //     output[g * D + s * (D / S) + t] = vals[s];
    // }
    // // now we have the top level done. each thread will now continue on its own.
    // // luckily this is just arithmetic. we're done loading the inputs.
}

// shared[tid] = input[bid * dim + tid];
// threadgroup_barrier(mem_flags::mem_threadgroup);
// for (short i = bid / 2; i > 0; i >>= 2)
// {
//     if (tid < i)
//     {
//         shared[tid] += shared[tid + i];
//     }
//     threadgroup_barrier(mem_flags::mem_threadgroup);
// }
// if (tid == 0)
// {
//     output[bid] = shared[0];
// }

// kernel void zkshift(
//     device int *input,
//     device int *output,
//     threadgroup int *shared [[threadgroup(0)]],
//     uint tid [[thread_position_in_threadgroup]],
//     uint bid [[threadgroup_position_in_grid]],
//     uint dim [[threads_per_threadgroup]])
// {
//     uint gid = bid * dim + tid;
//     shared[tid] = input[gid];
//     threadgroup_barrier(mem_flags::mem_threadgroup);
//     for (uint s = 1; s < dim; s *= 2)
//     {
//         int index = tid * 2 * s;
//         if (index < bid)
//         {
//             shared[tid] += shared[tid + s];
//         }
//         threadgroup_barrier(mem_flags::mem_threadgroup);
//     }
//     if (tid == 0)
//     {
//         output[bid] = shared[0];
//     }
// }

// // 1
// if (lid % 2 == 0)
// {
//     idata[lid] += idata[lid + 1];
// }
// threadgroup_barrier(mem_flags::mem_threadgroup);
// if (lid % 4 == 0)
// {
//     idata[lid] += idata[lid + 2];
// }
// threadgroup_barrier(mem_flags::mem_threadgroup);
// if (lid % 8 == 0)
// {
//     idata[lid] += idata[lid + 4];
// }
// threadgroup_barrier(mem_flags::mem_threadgroup);
// if (lid % 16 == 0)
// {
//     idata[lid] += idata[lid + 8];
// }
// // 2
// int index = 2 * lid;
// if(index < bid) {
//     idata[index] += idata[index + 1];
// }
// threadgroup_barrier(mem_flags::mem_threadgroup);

// if idx > n return
// for(int stride = 1; stride < dim; stride *= 2) {
//     if((lid % (2 * stride)) == 0) {
//         idata[lid] += idata[lid + stride];
//     }
//     threadgroup_barrier(mem_flags::mem_none);
// }
// if (lid == 0) {
//     g_odata[threadgroup_in_grid] = idata[0];
// }
// if(lid == 0) {
//     g_odata[bid] = idata[0];
// }
