
#include <metal_stdlib>

class Fp
{
public:
    // static
private:
    static constexpr uint mult(uint a, uint b)
    {
        unsigned long c = ulong(a) * ulong(b);
        // 2^40 = -1,
        // take upper 64-40=16 bits and subtract from lower 32.
        // write as low + mid*2^32 + high*2^40
        uint loww = uint(c);
        uint mid = (c << (64 - 32)) >> 32;
        uint
                    c0 +
                c1 * 2 ^
            32 + c2 * 2 ^ 40 c0 - c2

                                          c1 *
                                      (2 ^ 24 - 2 ^ 16 + 2 ^ 8 - 1) = 2(c1 << 16) -
    }
}