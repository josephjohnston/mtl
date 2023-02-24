#include <metal_stdlib>
// #include <metal_uniform>
#include "fp.h"

using namespace metal;



// struct Foo {
//     int b;
// };
// uint2 y = uint2(0,0);
// src.write(short4(17,17,17,17), y, 0);
// threadgroup_barrier(mem_flags::mem_texture);
// short4 a = src.read(y,0);

kernel void again(
    texture2d<short, access::read_write> src,
    device Fp *g_idata [[buffer(0)]],
    device Fp *g_odata [[buffer(1)]],
    constant uint &N [[buffer(2)]],
    constant uint &P [[buffer(3)]],
    uint gid [[thread_position_in_grid]],
    uint lid [[thread_position_in_threadgroup]],
    uint bid [[threadgroup_position_in_grid]],
    uint dim [[threads_per_threadgroup]]
) {
    // g_idata[0] = g_idata[1]*(2 << 20)+g_idata[2];
    for(uint i = 0; i < N/2; i++) {
        g_idata[i] =  g_idata[i]*(1 << 20) + g_idata[i+N/2];
        // g[0] = 2^20 g[0] + g[0+16]
        // g[1] = 2^20 + 17
    }
}



// kernel void reduce_neighborhood(
//     device int *g_idata,
//     device int *g_odata,
//     constant uint &n,
//     imageblock<Foo> img_blk,
//     texture2d<short, access::read_write> src,
//     uint gid [[thread_position_in_grid]],
//     uint lid [[thread_position_in_threadgroup]],
//     uint bid [[threadgroup_position_in_grid]],
//     uint dim [[threads_per_threadgroup]]
// ) {
//     // ushort2 z = ushort2(0,0);
//     // threadgroup_imageblock Foo *f = img_blk.data(z);
//     // f->b = 19;
//     // g_idata[0] = f->b;

// }

// xcrun -sdk macosx metal shader.metal -c -o shader.metallib
// xcrun -sdk macosx metal shader.metal -o shader_macos.metallib
// xcrun -sdk iosx metal shader.metal -o shader_ios.metallib
// xcrun -sdk iphoneos metal shader.metal -o shader_ios.metallib


// xcrun -sdk macosx metal -gline-tables-only -frecord-sources shader.metal -c -o shader.air
// xcrun -sdk macosx metal shader.metal -c -o shader.air
// xcrun -sdk macosx metallib shader.air -o shader.metallib

// https://vksegfault.github.io/posts/gentle-intro-gpu-inner-workings/



// kernel void eltwise_add_fp(
//     device Fp* out,
//     const device Fp* in1,
//     const device Fp* in2,
//     uint gid [[thread_position_in_grid]]
// ) {
//     out[gid] = in1[gid] + in2[gid];
// }

// kernel void eltwise_mul_factor_fp(
//     device Fp* io,
//     const device Fp& factor,
//     uint gid [[thread_position_in_grid]]
// ) {
//     io[gid] = io[gid] * factor;
// }

// kernel void eltwise_sum_fp4(
//     device Fp*,
//     const device Fp4* in,
//     device uint& count,
//     device uint& to_add,
//     uint gid [[thread_position_in_grid]]
// ) {
//     Fp4 total;
//     for(size_t i = 0; i < to_add; i++) {
//         total += in[gid + i*count];
//     }
//     for(uint i = 0; i< 4; i++) {
//         out[gid + i*count] = total.elems[i];
//     }
// }

