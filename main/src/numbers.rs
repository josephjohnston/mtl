pub mod G {
    //
    const EPSILON: u64 = (1 << 32) - (1 << 0);

    pub fn mult(a: u64, b: u64) -> u64 {
        let c = (a as u128) * (b as u128);
        let low = c as u64;
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

pub mod H {
    const EPSILON: u32 = (1 << 24) - (1 << 16) + (1 << 8) - (1 << 0);

    // pub fn add(a: u32, b: u32) -> u32 {}
    pub fn mult(a: u32, b: u32) -> u32 {
        let c = (a as u64) * (b as u64);
        let low = c as u32;
        // let mid = ((c << (64 - 40)) >> (64 - 8)) as u32;
        let top = (c >> 32) as u32;
        let mid = top & ((1 << 8) - 1);
        // let high = (c >> 40) as u32;
        let high = top >> 8;

        // let mut prod = (mid << 16) + mid;
        // prod = (prod << 8) - prod;
        let mut prod = mid << 16;
        prod += prod >> 16;
        prod = prod << 8;
        prod -= prod >> 8;

        let mut diff = low - high;
        if high > low {
            diff -= EPSILON;
        }
        let mut result = diff + prod;
        if result < prod {
            result += EPSILON;
        }
        result
    }
}

mod I {
    // for n=2^7*3^2, order 3, phi(n)=1536, solution 38.1
}
