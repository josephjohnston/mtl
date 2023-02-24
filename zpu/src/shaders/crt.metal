#include <metal_stdlib>
using namespace metal;

template <uint D, uint S, uint U>
void gamma(
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
    // DECOMPOSING WITHIN THREADS
    // for (ushort k = 0; k < ushort(log2(half(S))); k++)
    // {
    //     for (ushort i = 0; i < (1 << k); i++)
    //     {
    //         for (ushort s = 0; s < S / (1 << (k + 1)); s++)
    //         {
    //             for (ushort u = 0; u < U; u++)
    //             {
    //                 // uint hi_index = (2 * i + 1) * (S / (1 << (k + 1))) + s;
    //                 // uint lo_index = (2 * i) * (S / (1 << (k + 1))) + s;
    //                 uint hi_index =
    //                     (2 * i + 1) * (S / (1 << (k + 1))) * U + s * U + u;
    //                 uint mult = vals[hi_index] * (1 << 20);
    //                 uint lo_index = (2 * i) * (S / (1 << (k + 1))) * U + s * U + u;
    //                 vals[hi_index] = vals[lo_index] - mult;
    //                 vals[lo_index] = vals[lo_index] + mult;
    //             }
    //         }
    //     }
    // }
    // DECOMPOSING ACROSS THREADS

    // WRITING OUTPUT
    for (ushort s = 0; s < S; s++)
    {
        output[g * D + s * (D / S) + t] = vals[s];
    }
}

kernel void go(
    device uchar *input,
    device uint *output,
    uint g [[threadgroup_position_in_grid]],
    uint t [[thread_position_in_threadgroup]])
{
    gamma<1 << 7, 1 << 3, 1 << 0>(input, output, g, t);
}