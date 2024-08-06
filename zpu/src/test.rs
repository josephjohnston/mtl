use super::*;

use rand::Rng;

fn generate_major_input() -> Vec<Input> {
    let mut rng = rand::thread_rng();
    let input_len = E * F * G * D;
    let mut major_input: Vec<Input> = vec![0; input_len];
    for i in 0..input_len {
        major_input[i] = if major_input.len() <= (1 << 11) {
            NUMS[i] as Input
        } else {
            rng.gen::<Input>()
        };
    }
    major_input
}

// #[test]
pub fn test() {
    params::check();
    let major_input = generate_major_input();
    println!("PRNG DONE");

    let to_hide = 24;
    let gpu_coefs = native::go(&major_input, to_hide);
    // let cpu_coefs = reference::go(&major_input, to_hide);
    // for i in 0..cpu_coefs.len() {
    //     assert_eq!(cpu_coefs[i], gpu_coefs[i].addm(0, &P));
    // }
}
