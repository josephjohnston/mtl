
#include "fp.h"

kernel void go(
    device uchar2 *input,
    device uint2 *output,
    ushort t [[thread_index_in_simdgroup]],
    ushort w [[simdgroup_index_in_threadgroup]],
    ushort b [[threadgroup_position_in_grid]])
{
    uint2 array[8];
    uint in_global_index_prefix = b * 8192 + w * 256 + t * 1;
    array[0] = uint2(input[in_global_index_prefix + 0]);
    array[1] = uint2(input[in_global_index_prefix + 32]);
    array[2] = uint2(input[in_global_index_prefix + 64]);
    array[3] = uint2(input[in_global_index_prefix + 96]);
    array[4] = uint2(input[in_global_index_prefix + 128]);
    array[5] = uint2(input[in_global_index_prefix + 160]);
    array[6] = uint2(input[in_global_index_prefix + 192]);
    array[7] = uint2(input[in_global_index_prefix + 224]);
    {
        uint mult = mul(array[8],1048576);
        array[8] = sub(array[0],mult);
        array[0] = add(array[0],mult);
    }
    {
        uint mult = mul(array[9],1048576);
        array[9] = sub(array[1],mult);
        array[1] = add(array[1],mult);
    }
    {
        uint mult = mul(array[10],1048576);
        array[10] = sub(array[2],mult);
        array[2] = add(array[2],mult);
    }
    {
        uint mult = mul(array[11],1048576);
        array[11] = sub(array[3],mult);
        array[3] = add(array[3],mult);
    }
    {
        uint mult = mul(array[12],1048576);
        array[12] = sub(array[4],mult);
        array[4] = add(array[4],mult);
    }
    {
        uint mult = mul(array[13],1048576);
        array[13] = sub(array[5],mult);
        array[5] = add(array[5],mult);
    }
    {
        uint mult = mul(array[14],1048576);
        array[14] = sub(array[6],mult);
        array[6] = add(array[6],mult);
    }
    {
        uint mult = mul(array[15],1048576);
        array[15] = sub(array[7],mult);
        array[7] = add(array[7],mult);
    }
    {
        uint mult = mul(array[4],1024);
        array[4] = sub(array[0],mult);
        array[0] = add(array[0],mult);
    }
    {
        uint mult = mul(array[5],1024);
        array[5] = sub(array[1],mult);
        array[1] = add(array[1],mult);
    }
    {
        uint mult = mul(array[6],1024);
        array[6] = sub(array[2],mult);
        array[2] = add(array[2],mult);
    }
    {
        uint mult = mul(array[7],1024);
        array[7] = sub(array[3],mult);
        array[3] = add(array[3],mult);
    }
    {
        uint mult = mul(array[12],1073741824);
        array[12] = sub(array[8],mult);
        array[8] = add(array[8],mult);
    }
    {
        uint mult = mul(array[13],1073741824);
        array[13] = sub(array[9],mult);
        array[9] = add(array[9],mult);
    }
    {
        uint mult = mul(array[14],1073741824);
        array[14] = sub(array[10],mult);
        array[10] = add(array[10],mult);
    }
    {
        uint mult = mul(array[15],1073741824);
        array[15] = sub(array[11],mult);
        array[11] = add(array[11],mult);
    }
    {
        uint mult = mul(array[2],32);
        array[2] = sub(array[0],mult);
        array[0] = add(array[0],mult);
    }
    {
        uint mult = mul(array[3],32);
        array[3] = sub(array[1],mult);
        array[1] = add(array[1],mult);
    }
    {
        uint mult = mul(array[6],33554432);
        array[6] = sub(array[4],mult);
        array[4] = add(array[4],mult);
    }
    {
        uint mult = mul(array[7],33554432);
        array[7] = sub(array[5],mult);
        array[5] = add(array[5],mult);
    }
    {
        uint mult = mul(array[10],32768);
        array[10] = sub(array[8],mult);
        array[8] = add(array[8],mult);
    }
    {
        uint mult = mul(array[11],32768);
        array[11] = sub(array[9],mult);
        array[9] = add(array[9],mult);
    }
    {
        uint mult = mul(array[14],4144559881);
        array[14] = sub(array[12],mult);
        array[12] = add(array[12],mult);
    }
    {
        uint mult = mul(array[15],4144559881);
        array[15] = sub(array[13],mult);
        array[13] = add(array[13],mult);
    }
    uint out_global_index_prefix = b * 8192 + w * 256 + t * 1;
    output[out_global_index_prefix + 0] = array[0];
    output[out_global_index_prefix + 32] = array[1];
    output[out_global_index_prefix + 64] = array[2];
    output[out_global_index_prefix + 96] = array[3];
    output[out_global_index_prefix + 128] = array[4];
    output[out_global_index_prefix + 160] = array[5];
    output[out_global_index_prefix + 192] = array[6];
    output[out_global_index_prefix + 224] = array[7];
}