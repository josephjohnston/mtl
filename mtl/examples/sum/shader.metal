
#include <metal_stdlib>
using namespace metal;

kernel void test(
    device const uint32_t *input [[buffer(0)]],
    device uint32_t *output [[buffer(1)]],
    ushort e [[threadgroup_position_in_grid]],
    ushort i [[simdgroup_index_in_threadgroup]],
    ushort j [[thread_index_in_simdgroup]],
    ushort simdGroupSize [[threads_per_simdgroup]],
    ushort simdGroupCount [[simdgroups_per_threadgroup]])
{
    uint global_id = e * (simdGroupCount * simdGroupSize) + i * simdGroupSize + j;
    output[global_id] = global_id;
}

// xcrun -sdk macosx metal shader.metal -o shader.metallib

// we need to know the length, and then we need to figu