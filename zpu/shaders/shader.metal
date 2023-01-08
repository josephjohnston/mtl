#include <metal_stdlib>

using namespace metal;

struct Foo {
    int b;
};


// kernel void again(
//     texture2d<short, access::read> src,
//     device int *g_idata
// ) {
//     uint2 y = uint2(0,0);
//     src.write(short4(17,17,17,17), y, 0);
//     short4 a = src.read(y,0);
//     // g_idata[0] = a[3];
//     // g_idata[1] = a[1];
//     // g_idata[2] = a[2];
//     // g_idata[3] = a[0];
// }

kernel void reduce_neighborhood(
    device int *g_idata,
    device int *g_odata,
    constant uint &n,
    imageblock<Foo> img_blk,
    texture2d<short, access::read_write> src,
    uint gid [[thread_position_in_grid]],
    uint lid [[thread_position_in_threadgroup]],
    uint bid [[threadgroup_position_in_grid]],
    uint dim [[threads_per_threadgroup]]
) {
    // ushort2 z = ushort2(0,0);
    // threadgroup_imageblock Foo *f = img_blk.data(z);
    // f->b = 17;
    // g_idata[0] = f->b;

    device int *idata = g_idata + dim * bid;
    // 1
    if(lid % 2 == 0) {
        idata[lid] += idata[lid + 1];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if(lid % 4 == 0) {
        idata[lid] += idata[lid + 2];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if(lid % 8 == 0) {
        idata[lid] += idata[lid + 4];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if(lid % 16 == 0) {
        idata[lid] += idata[lid + 8];
    }
    // 2
    int index = 2 * lid;
    if(index < bid) {
        idata[index] += idata[index + 1];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);

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
    if(lid == 0) {
        g_odata[bid] = idata[0];
    }
}

// xcrun -sdk macosx metal shader.metal -c -o shader.metallib
// xcrun -sdk macosx metal shader.metal -o shader_macos.metallib
// xcrun -sdk iosx metal shader.metal -o shader_ios.metallib
// xcrun -sdk iphoneos metal shader.metal -o shader_ios.metallib


// xcrun -sdk macosx metal -gline-tables-only -frecord-sources shader.metal -c -o shader.air
// xcrun -sdk macosx metal shader.metal -c -o shader.air
// xcrun -sdk macosx metallib shader.air -o shader.metallib

// https://vksegfault.github.io/posts/gentle-intro-gpu-inner-workings/




