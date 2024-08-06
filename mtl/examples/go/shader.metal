
#include <metal_stdlib>
// #include "utility.h"

using namespace metal;

// [[visible]] int calculate(int x, int y) {
//     return x * y;
// }

// int utility(int gid) {
//     return 2*gid;
// }

kernel void go(
    texture2d<int, access::read_write> src,
    device int *dest,
    uint gid [[thread_position_in_grid]])
{
    // int4 x = int4(4,3,2,1);
    // uint2 y = uint2(0,0);
    // src.write(x, y, 0);
    dest[0] = 108;
    // threadgroup_barrier(mem_flags::mem_texture);
    // int4 a = src.read(y,0);
    // threadgroup_barrier(mem_flags::mem_texture);
}

// xcrun -sdk macosx metal shader.metal -c -o shader.air
// xcrun -sdk macosx metal shader.metal -o shader.metallib
