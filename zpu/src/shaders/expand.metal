
template <uint N>
void my_func(
    device uint *input)
{
    for (ushort i = 0; i < N; i++)
    {
        input[i] += input[i + 1];
    }
}

kernel void go(
    device uint *input)
{
    my_func<1 << 4>(input);
}