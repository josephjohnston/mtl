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

fn generate_constants() -> Vec<Output> {
    let mut rng = rand::thread_rng();
    let constants_len = D * E * F * G;
    let mut constants: Vec<Output> = vec![0; constants_len];
    for i in 0..constants_len {
        constants[i] = rng.gen::<Output>();
    }
    constants
}

#[test]
fn random() {
    params::check();
    let major_input = generate_major_input();
    let constants = generate_constants();
    println!("\nPRNG DONE!");

    let gpu_coefs = naive::go(&major_input, &constants, 0);
    let cpu_coefs = reference::go(&major_input, &constants);
    for i in 0..cpu_coefs.len() {
        assert_eq!(cpu_coefs[i], gpu_coefs[i].addm(0, &P));
    }
    println!("\nGPU CONSISTENT");
}
