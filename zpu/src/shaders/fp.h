
#include <metal_stdlib>

// class Fp
// {
// public:
//     static constant int P = 4278255361;

// private:
//     uint val;
//     static constexpr int mul(int a, int b)
//     {
//         return int((long(a) * long(b)) % P);
//         // // 2^40 = -1,
//         // // take upper 64-40=16 bits and subtract from lower 32.
//         // // write as low + mid*2^32 + high*2^40
//         // uint low = uint(c);
//         // uint mid = (c << (64 - 32)) >> 32;
//         // uint high = c >> 40;
//     }
//     static constexpr int add(int a, int b)
//     {
//         return int((long(a) + long(b)) % P);
//     }
//     // static constexpr uint sub(uint a, uint b)
//     // {
//     //     return (ulong(a) - ulong(b)) % P;
//     // }

// public:
//     constexpr Fp() : val(0) {}
//     constexpr Fp(uint v) : val(v) {}
//     constexpr uint asUInt() const { return val; }

//     constexpr void operator=(uint rhs) { val = rhs; }
//     constexpr void operator=(uint rhs) device { val = rhs; }
//     Fp operator+=(Fp rhs)
//     {
//         val = add(val, rhs.val);
//         return *this;
//     }
//     Fp operator+=(Fp rhs) device
//     {
//         val = add(val, rhs.val);
//         return *this;
//     }
//     constexpr Fp operator+(Fp rhs) const { return Fp(add(val, rhs.val)); }
//     constexpr Fp operator+(Fp rhs) const device { return Fp(add(val, rhs.val)); }
//     // constexpr Fp operator-(Fp rhs) const { return Fp(sub(val, rhs.val)); }
//     // // constexpr Fp operator-() const { return Fp(sub(0,val)); }
//     constexpr Fp operator*(Fp rhs) const { return Fp(mul(val, rhs.val)); }
//     constexpr Fp operator*(Fp rhs) const device { return Fp(mul(val, rhs.val)); }
//     // constexpr Fp operator[](int index) const { return }

//     // constexpr bool operator==(Fp rhs) const { return val == rhs.val; }
//     // constexpr bool operator!=(Fp rhs) const { return val != rhs.val; }
//     // constexpr bool operator!=(Fp rhs) device const { return val != rhs.val; }
// };

static const constant uint P = 4278255361;

uint mul(uint a, uint b)
{
    return uint((ulong(a) * ulong(b)) % P);
}

uint add(uint a, uint b)
{
    return uint((ulong(a) + ulong(b)) % P);
}

uint sub(uint a, uint b)
{
    return uint((ulong(a) - ulong(b)) % P);
}

// template <class UType, ushort U>
// UType mul_scalar(UType a, uint b)
// {
//     UType result;
//     for (ushort u = 0; u < U; u++)
//     {
//         result[u] = uint((ulong(a[u]) * ulong(b)) % P);
//     }
//     return result;
// }

// template <class UType, ushort U>
// UType mul_vector(UType a, UType b)
// {
//     UType result;
//     for (ushort u = 0; u < U; u++)
//     {
//         result[u] = uint((ulong(a[u]) * ulong(b[u])) % P);
//     }
//     return result;
// }

// template <class UType, ushort U>
// UType add_scalar(UType a, uint b)
// {
//     UType result;
//     for (ushort u = 0; u < U; u++)
//     {
//         result[u] = uint((ulong(a[u]) + ulong(b)) % P);
//     }
//     return result;
// }

// template <class UType, ushort U>
// UType add_vector(UType a, UType b)
// {
//     UType result;
//     for (ushort u = 0; u < U; u++)
//     {
//         result[u] = uint((ulong(a[u]) + ulong(b[u])) % P);
//     }
//     return result;
// }

// template <class UType, ushort U>
// UType sub_scalar(UType a, uint b)
// {
//     UType result;
//     for (ushort u = 0; u < U; u++)
//     {
//         result[u] = uint((ulong(a[u]) + P - ulong(b)) % P);
//     }
//     return result;
// }

// template <class UType, ushort U>
// UType sub_vector(UType a, UType b)
// {
//     UType result;
//     for (ushort u = 0; u < U; u++)
//     {
//         result[u] = uint((ulong(a[u]) + P - ulong(b[u])) % P);
//     }
//     return result;
// }

class Fp
{
public:
    static const constant uint P = 4278255361;
    // static const constant uint R1_0 = P - 1;
    // static constant uint R2_0 = 1 << 20;
    // static constant uint R2_1 = P - (1 << 20);

private:
    uint val;
    static constexpr uint mul(uint a, uint b)
    {
        return uint((ulong(a) * ulong(b)));
        // return (ulong(a) * ulong(b));
        // // 2^40 = -1,
        // // take upper 64-40=16 bits and subtract from lower 32.
        // // write as low + mid*2^32 + high*2^40
        // uint low = uint(c);
        // uint mid = (c << (64 - 32)) >> 32;
        // uint high = c >> 40;
        // return a * b;
    }
    static constexpr uint add(uint a, uint b)
    {
        return (ulong(a) + ulong(b));
        // return (ulong(a) + ulong(b));
        // return a + b;
    }
    static constexpr uint sub(uint a, uint b)
    {
        return (ulong(a) - ulong(b) + P);
        // return (ulong(a) - ulong(b) + P);
        // return a - b;
    }

public:
    constexpr Fp() : val(0) {}
    constexpr Fp(uint v) : val(v) {}
    constexpr uint asUInt() const { return val; }

    constexpr Fp add(Fp rhs) const
    {
        return Fp(add(val, rhs.val));
    }
    constexpr Fp sub(Fp rhs) const
    {
        return Fp(sub(val, rhs.val));
    }

    Fp mul2_0() const
    {
        return Fp(mul(val, 1 << 20));
    }

    operator uint()
    {
        return val;
    }

    // constexpr void operator=(uint rhs) { val = rhs; }
    // // constexpr void operator=(uint rhs) device { val = rhs; }
    // // Fp operator+=(Fp rhs)
    // // {
    // //     val = add(val, rhs.val);
    // //     return *this;
    // // }
    // // Fp operator+=(Fp rhs) device
    // // {
    // //     val = add(val, rhs.val);
    // //     return *this;
    // // }
    // // Fp operator-=(Fp rhs)
    // // {
    // //     val = sub(val, rhs.val);
    // //     return *this;
    // // }
    // // Fp operator-=(Fp rhs) device
    // // {
    // //     val = sub(val, rhs.val);
    // //     return *this;
    // // }
    // // +
    // // constexpr Fp operator+(Fp rhs) const thread { return Fp(add(val, rhs.val)); }
    // constexpr Fp operator+(Fp rhs) const { return Fp(add(val, rhs.val)); }
    // // constexpr Fp operator+(Fp rhs) const device { return Fp(add(val, rhs.val)); }
    // // -
    // constexpr Fp operator-(Fp rhs) const threadgroup { return Fp(sub(val, rhs.val)); }
    // // constexpr Fp operator-() const { return Fp(sub(0, val)); }
    // // *
    // // constexpr Fp operator*(Fp rhs) const thread { return Fp(mul(val, rhs.val)); }
    // constexpr Fp operator*(Fp rhs) const { return Fp(mul(val, rhs.val)); }
    // // constexpr Fp operator*(Fp rhs) const device { return Fp(mul(val, rhs.val)); }
    // // constexpr Fp operator[](int index) const { return }

    // // constexpr bool operator==(Fp rhs) const { return val == rhs.val; }
    // // constexpr bool operator!=(Fp rhs) const { return val != rhs.val; }
    // // constexpr bool operator!=(Fp rhs) device const { return val != rhs.val; }
};

// class Fp {
// public:
//   /// The value of P, the modulus of Fp.
//   static constant uint32_t P = 15 * (uint32_t(1) << 27) + 1;
//   static constant uint32_t M = 0x88000001;
//   static constant uint32_t R2 = 1172168163;

// private:
//   // The actual value, always < P.
//   uint32_t val;

//   // We make 'impls' of the core ops which all the other uses call.  This is done to allow for
//   // tweaking of the implementation later, for example switching to montgomery representation or
//   // doing inline assembly or some crazy CUDA stuff.

//   // Add two numbers
//   static constexpr uint32_t add(uint32_t a, uint32_t b) {
//     uint32_t r = a + b;
//     return (r >= P ? r - P : r);
//   }

// //   // Subtract two numbers
// //   static constexpr uint32_t sub(uint32_t a, uint32_t b) {
// //     uint32_t r = a - b;
// //     return (r > P ? r + P : r);
// //   }

//   // Multiply two numbers
//   static constexpr uint32_t mul(uint32_t a, uint32_t b) {
//     uint64_t o64 = uint64_t(a) * uint64_t(b);
//     uint32_t low = -uint32_t(o64);
//     uint32_t red = M * low;
//     o64 += uint64_t(red) * uint64_t(P);
//     uint32_t ret = o64 >> 32;
//     return (ret >= P ? ret - P : ret);
//   }

// //   // Encode / Decode
//   static constexpr uint32_t encode(uint32_t a) { return mul(R2, a); }

// //   static constexpr uint32_t decode(uint32_t a) { return mul(1, a); }

//   // A private constructor that take the 'interal' form.
//   constexpr Fp(uint32_t val, bool ignore) : val(val) {}

// public:
//   /// Default constructor, sets value to 0.
//   constexpr Fp() : val(0) {}

// //   /// Construct an FP from a uint32_t, wrap if needed
// //   constexpr Fp(uint32_t val) : val(encode(val)) {}

// //   /// Convert to a uint32_t
// //   constexpr uint32_t asUInt32() const { return decode(val); }

// //   constexpr uint32_t asUInt32() device const { return decode(val); }

// //   /// Return the raw underlying word
// //   constexpr uint32_t asRaw() const { return val; }

// //   /// Get the largest value, basically P - 1.
// //   static constexpr Fp maxVal() { return P - 1; }

// //   /// Get an 'invalid' Fp value
// //   static constexpr Fp invalid() { return Fp(0xfffffffful, true); }

//   // Implement all the various overloads
//   constexpr void operator=(uint32_t rhs) { val = encode(rhs); }

// //   constexpr void operator=(uint32_t rhs) device { val = encode(rhs); }

// //   constexpr Fp operator+(Fp rhs) const { return Fp(add(val, rhs.val), true); }

// //   constexpr Fp operator-() const { return Fp(sub(0, val), true); }

// //   constexpr Fp operator-(Fp rhs) const { return Fp(sub(val, rhs.val), true); }

// //   constexpr Fp operator*(Fp rhs) const { return Fp(mul(val, rhs.val), true); }

// //   constexpr Fp operator+(Fp rhs) device const { return Fp(add(val, rhs.val), true); }

// //   constexpr Fp operator-() device const { return Fp(sub(0, val), true); }

// //   constexpr Fp operator-(Fp rhs) device const { return Fp(sub(val, rhs.val), true); }

// //   constexpr Fp operator*(Fp rhs) device const { return Fp(mul(val, rhs.val), true); }

//   constexpr Fp operator+=(Fp rhs) {
//     val = add(val, rhs.val);
//     return *this;
//   }

//   constexpr Fp operator+=(Fp rhs) device {
//     val = add(val, rhs.val);
//     return *this;
//   }

// //   constexpr Fp operator-=(Fp rhs) {
// //     val = sub(val, rhs.val);
// //     return *this;
// //   }

// //   constexpr Fp operator*=(Fp rhs) {
// //     val = mul(val, rhs.val);
// //     return *this;
// //   }

// //   constexpr bool operator==(Fp rhs) const { return val == rhs.val; }

// //   constexpr bool operator!=(Fp rhs) const { return val != rhs.val; }

// //   constexpr bool operator<(Fp rhs) const { return decode(val) < decode(rhs.val); }

// //   constexpr bool operator<=(Fp rhs) const { return decode(val) <= decode(rhs.val); }

// //   constexpr bool operator>(Fp rhs) const { return decode(val) > decode(rhs.val); }

// //   constexpr bool operator>=(Fp rhs) const { return decode(val) >= decode(rhs.val); }

// //   constexpr bool operator==(Fp rhs) device const { return val == rhs.val; }

// //   constexpr bool operator!=(Fp rhs) device const { return val != rhs.val; }

// //   constexpr bool operator<(Fp rhs) device const { return decode(val) < decode(rhs.val); }

// //   constexpr bool operator<=(Fp rhs) device const { return decode(val) <= decode(rhs.val); }

// //   constexpr bool operator>(Fp rhs) device const { return decode(val) > decode(rhs.val); }

// //   constexpr bool operator>=(Fp rhs) device const { return decode(val) >= decode(rhs.val); }

// //   // Post-inc/dec
// //   constexpr Fp operator++(int) {
// //     Fp r = *this;
// //     val = add(val, encode(1));
// //     return r;
// //   }

// //   constexpr Fp operator--(int) {
// //     Fp r = *this;
// //     val = sub(val, encode(1));
// //     return r;
// //   }

// //   // Pre-inc/dec
// //   constexpr Fp operator++() {
// //     val = add(val, encode(1));
// //     return *this;
// //   }

// //   constexpr Fp operator--() {
// //     val = sub(val, encode(1));
// //     return *this;
// //   }
// };

// // /// Raise an value to a power
// // constexpr inline Fp pow(Fp x, size_t n) {
// //   Fp tot = 1;
// //   while (n != 0) {
// //     if (n % 2 == 1) {
// //       tot *= x;
// //     }
// //     n = n / 2;
// //     x *= x;
// //   }
// //   return tot;
// // }

// // /// Compute the multiplicative inverse of x, or `1/x` in finite field terms.  Since `x^(P-1) == 1
// // /// (mod P)` for any x != 0 (as a consequence of Fermat's little therorm), it follows that `x *
// // /// x^(P-2) == 1 (mod P)` for x != 0.  That is, `x^(P-2)` is the multiplicative inverse of x.
// // /// Computed this way, the 'inverse' of zero comes out as zero, which is convient in many cases, so
// // /// we leave it.
// // constexpr inline Fp inv(Fp x) {
// //   return pow(x, Fp::P - 2);
// // }

uint get_zeta(uint k, uint i)
{
    uint prims_2[] = {
        1048576,
    };
    uint prims_3[] = {
        1024,
        1073741824,
    };
    uint prims_4[] = {
        32,
        33554432,
        32768,
        4144559881,
    };
    uint prims_5[] = {
        16707839,
        4261539330,
        4274061053,
        4274061061,
        534650848,
        534912992,
        4144037761,
        4144037505,
    };
    uint prims_6[] = {
        364914777,
        2274230434,
        1464515241,
        1441048032,
        3120762142,
        45032751,
        196321259,
        947271947,
        4184386525,
        3065742370,
        919248094,
        2278185239,
        171586511,
        531848519,
        1274452609,
        3982137898,
    };
    uint prims_7[] = {
        1297642494,
        3327754660,
        2526751946,
        2129504484,
        3020261559,
        3810020456,
        3847465774,
        3970313073,
        2689265513,
        2766251085,
        1388567172,
        3842193303,
        2954927500,
        3786866165,
        3159035588,
        2626659467,
        2504948723,
        1488462141,
        3151931493,
        2392531113,
        3830654479,
        1816320888,
        1128492723,
        3708275820,
        3081630698,
        3779079003,
        2234052728,
        1762621666,
        3037601520,
        786573619,
        1139377988,
        4065946328,
    };
    uint prims_8[] = {
        493853244,
        2630285104,
        871589258,
        2389324427,
        2968537725,
        2882271469,
        2221324090,
        3728040527,
        1792685315,
        4069366704,
        338212691,
        10783282,
        2529644974,
        2406181663,
        2011959971,
        3933190337,
        1122335902,
        562582394,
        1578728461,
        1480102279,
        889615164,
        2589549385,
        302463957,
        819753580,
        1816837538,
        2239014032,
        599443123,
        394504728,
        1081892113,
        2521481523,
        210614787,
        2069158492,
        2122591668,
        3775120428,
        1819528072,
        180144644,
        1486373247,
        1670676750,
        529152400,
        3265551773,
        206566095,
        3541041349,
        1934204752,
        1889168591,
        3720435503,
        1998977010,
        2331859679,
        2078683782,
        1026655235,
        2800282348,
        1058032482,
        1155858166,
        3909251897,
        2761418424,
        4043927916,
        1373075368,
        2126899470,
        276736331,
        1013149118,
        3965176830,
        1805271112,
        1462002270,
        3979203491,
        3886952625,
    };
    if (k == 2)
    {
        return prims_2[i];
    }
    else if (k == 3)
    {
        return prims_3[i];
    }
    else if (k == 4)
    {
        return prims_4[i];
    }
    else if (k == 5)
    {
        return prims_5[i];
    }
    else if (k == 6)
    {
        return prims_6[i];
    }
    else if (k == 7)
    {
        return prims_7[i];
    }
    else if (k == 8)
    {
        return prims_8[i];
    }
    else
    {
        return 0;
    }
}

// uint seeds[] = {

// };