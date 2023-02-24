pub mod G {
    const EPSILON: u64 = (1 << 32) - (1 << 0);

    pub fn mult(a: u64, b: u64) -> u64 {
        let c = (a as u128) * (b as u128);
        let bottom = c as u64;
        let low = bottom;
        let top = (c >> 64) as u64;
        let mid = top & EPSILON;
        let high = (top >> 32) as u64;
        let prod = (mid << 32) - mid;
        // mid * EPSILON;
        let mut diff = low - high;
        if high > low {
            diff -= EPSILON as u64;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        result
    }
}

// extra bits
// shift left by k, maybe just virtually.

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct H(pub u32);
const EPSILON: u32 = (1 << 24) - (1 << 16) + (1 << 8) - (1 << 0);
impl H {
    // pub fn add(a: u32, b: u32) -> u32 {}
    pub fn prod(&self, b: Self) -> Self {
        let c = (self.0 as u64) * (b.0 as u64);
        let bottom = c as u32;
        // let mid = ((c << (64 - 40)) >> (64 - 8)) as u32;
        let top = (c >> 32) as u32;
        let low = bottom;
        let mid = top & ((1 << 8) - 1);
        // let high = (c >> 40) as u32;
        let high = top >> 8;

        let mut prod = (mid << 16) + mid;
        prod = (prod << 8) - prod;
        // let mut prod = mid << 16;
        // prod += prod >> 16;
        // prod = prod << 8;
        // prod -= prod >> 8;

        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    #[allow(arithmetic_overflow)]
    pub fn sum(&self, b: Self) -> Self {
        let mut sum = self.0 + b.0;
        if sum < b.0 {
            if sum >= 0 - EPSILON {
                sum += 2 * EPSILON;
            } else {
                sum += EPSILON;
            }
        }
        Self(sum)
    }
    #[allow(arithmetic_overflow)]
    pub fn diff(&self, b: Self) -> Self {
        let mut diff = self.0 - b.0;
        if self.0 < b.0 {
            diff -= EPSILON;
            if diff >= 0 - EPSILON {
                diff -= EPSILON;
            }
        }
        Self(diff)
    }

    pub fn prod_5(&self) -> Self {
        let top = self.0 >> (32 - 5);
        let bottom = self.0 << 5;
        let prod = top * EPSILON;
        let mut result = bottom + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    pub fn prod_10(&self) -> Self {
        let bottom = self.0 << 10;
        let top = self.0 >> (32 - 10);
        let low = bottom;
        let mid = top & ((1 << 8) - 1);
        let high = top >> 8;

        let prod = mid * EPSILON;
        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    pub fn prod_15(&self) -> Self {
        let bottom = self.0 << 15;
        let top = self.0 >> (32 - 15);
        let low = bottom;
        let mid = top & ((1 << 8) - 1);
        let high = top >> 8;

        let prod = mid * EPSILON;
        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    pub fn prod_20(&self) -> Self {
        let bottom = self.0 << 20;
        let top = self.0 >> (32 - 20);
        let low = bottom;
        let mid = top & ((1 << 8) - 1);
        let high = top >> 8;

        let prod = mid * EPSILON;
        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    pub fn prod_25(&self) -> Self {
        let bottom = self.0 << 25;
        let top = self.0 >> (32 - 25);
        let low = bottom;
        let mid = top & ((1 << 8) - 1);
        let high = top >> 8;

        let prod = mid * EPSILON;
        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    pub fn prod_30(&self) -> Self {
        let bottom = self.0 << 30;
        let top = self.0 >> (32 - 30);
        let low = bottom;
        let mid = top & ((1 << 8) - 1);
        let high = top >> 8;

        if low as u64 + (1 << 32) * (mid as u64) + (1 << 40) * (high as u64)
            != (self.0 as u64) * (1 << 30)
        {
            println!("{}", self.0);
            println!("{low}, {mid}, {high}");
        };

        let prod = mid * EPSILON;
        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        Self(result)
    }
    // not sure what to do for 35, haven't thought about it
    // 2^{a*40+b}
    // subtract from p if a is odd
    // shift left by b bits (up to 39) and reduce
    // https://cp4space.hatsya.com/2021/09/01/an-efficient-prime-for-number-theoretic-transforms/
    // we always write X*2^40 + Y*Eps + Z
    // Z is 32 bits, Y is 8 bits, X is up to 31/2 bits
    // we want Z-X
    // we want to be able to shift left by 39/40 bits
}

mod I {
    // for n=2^7*3^2, order 3, phi(n)=1536, solution 38.1

    // for later
}
