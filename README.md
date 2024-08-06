
No documentation for now.
I wrote `mtl` to learn about Metal and Rust, and build bindings on `objc2` for safest interop between Rust and Objective-C.
I began on a hardware abstraction layer `hal` to simplify usage of `mtl`.
In `zpu` I use `mtl` directly.

Once in this directory, try
` cargo run --bin zpu --release`
and see output as below.
The figures on the top are relevant parameters.
Pseudorandom number generation of an input occurs, and then encoding of data for the GPU, and the GPU returns a vector of which every 2^24'th element is displayed.
Printing the `end times` (of cpu and gpu) is a hack to avoid crashing.
The timestamp is the number of milliseconds taken.
Current parameters represent the equivalent of committing 2^29 field elements, each element a byte, though the modulus is large enough each element could also be two bytes, in which case that's 1GB worth of field elements committed in 100ms on my M2 air.
The bottleneck is multiplication of 256-bit field elements (a degree-8 extension of a 32-bit prime), and the extra arithmetic and overflow checks needed for reduction on this 32-bit prime (`2^32-2^24+2^16-2^8+1`).
Switching to the prime `2^16+1` (to be done), the first cost will completely disappear while the second will be minimal with a single subtraction and overflow check.

```
S: 8, T: 128, J: 2, K: 1, T_J: 2
DECOMPS: 7
LOG_DEG: 3
PRNG DONE
ENCODING DONE

GPU COEFS:
0: 1512679166
16777216: 79423001
33554432: 3826312280
50331648: 1496890540
67108864: 2378816035
83886080: 953629831
100663296: 504138149
117440512: 2418787688
134217728: 2157201342
150994944: 1566083781
167772160: 4150078029
184549376: 3361336455
201326592: 1864857051
218103808: 2169960419
234881024: 3001931358
251658240: 2699190436
268435456: 3262133215
285212672: 3137904268
301989888: 3669827662
318767104: 2148411304
335544320: 724383635
352321536: 2278303218
369098752: 4182880130
385875968: 1914513434
402653184: 1142463988
419430400: 141179634
436207616: 1326809710
452984832: 2005445116
469762048: 3180495256
486539264: 3933829751
503316480: 3269346552
520093696: 329396339

end times: 41951904290625, 41951904290625
timespan: [100]
```