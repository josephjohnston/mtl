
// #include <iostream>

template <int N>
void my_func(
    int *input)
{
    for (short i = 0; i < N - 1; i++)
    {
        input[i + 1] += input[i];
    }
}

// void go(
//     int *input)
// {
//     my_func<4>(input);
// }

int main()
{
    int a[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    int *input = a;
    my_func<8>(input);
    for (int i = 0; i < 8; i++)
    {
        std::cout << ", ";
        std::cout << a[i];
    }
}