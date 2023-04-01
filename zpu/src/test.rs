use super::*;

use rand::Rng;

#[test]
fn random() {
    params::check();
    // major input
    let mut rng = rand::thread_rng();
    let input_len = E * F * G * D;
    let mut major_input: Vec<u8> = vec![0; input_len];
    for i in 0..input_len {
        major_input[i] = if major_input.len() <= (1 << 11) {
            NUMS[i]
        } else {
            rng.gen::<u8>()
        };
    }

    // minor input
    // let mult_val = 2091658123;
    // let add_val = 1523138830;
    // let mut state = 1;
    let component_size = T_J * S / (1 << K);
    let mut minor_input: Vec<u32> = vec![0; component_size];
    // for i in 0..component_size {
    //     state = add_val.addm(mult_val.mulm(state, &P), &P);
    //     minor_input[i] = state;
    // }

    minor_input[0] = 3614796953;
    minor_input[1] = 1208427060;
    minor_input[2] = 1889015752;
    minor_input[3] = 3198863462;
    minor_input[4] = 3614796953;
    minor_input[5] = 1208427060;
    minor_input[6] = 1889015752;
    minor_input[7] = 3198863462;

    let gpu_coefs = naive::go(&major_input, 22);
    let cpu_coefs = reference::go(&major_input, &minor_input);
    for i in 0..cpu_coefs.len() {
        assert_eq!(cpu_coefs[i], gpu_coefs[i]);
    }
    println!("\nGPU CONSISTENT");
}
