use num_modular::ModularCoreOps;

const EPSILON: u32 = (1 << 24) - (1 << 16) + (1 << 8) - (1 << 0);
const P: u32 = ((1 << 32) - EPSILON as u64) as u32;
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Fp(pub u32);
impl Fp {
    pub fn add(&self, rhs: Self) -> Self {
        Fp(self.0.addm(rhs.0, &P))
    }
    pub fn sub(&self, rhs: Self) -> Self {
        Fp(self.0.subm(rhs.0, &P))
    }
    pub fn mul(&self, rhs: Self) -> Self {
        Fp(self.0.mulm(rhs.0, &P))
    }
}

// const EPSILON: i32 = (1 << 24) - (1 << 16) + (1 << 8) - (1 << 0);
// const P: i32 = ((1 << 32) - EPSILON as i64) as i32;
// #[derive(Debug, Clone, Copy)]
// #[repr(transparent)]
// pub struct Fp(pub i32);
// impl Fp {
//     pub fn mul(&self, rhs: Self) -> Self {
//         Fp(self.0.mulm(rhs.0, &P))
//     }
// }
