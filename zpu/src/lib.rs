use hal::*;
use num_modular::ModularCoreOps;

mod reference;

mod naive;

mod params;
use params::*;

mod shaders;

#[cfg(test)]
mod test;

#[no_mangle]
#[allow(arithmetic_overflow)]
pub extern "C" fn rust_addition() -> i32 {
    2
}
