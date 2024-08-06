// use hal::*;
use num_modular::ModularCoreOps;

mod reference;

mod native;

mod shaders;

mod params;
use params::*;

// #[cfg(test)]
mod test;

const SHADER: &'static str = "unrolled";

// #[no_mangle]
// #[allow(arithmetic_overflow)]
// pub extern "C" fn rust_addition() -> i32 {
//     2
// }

fn main() {
    test::test();
}
