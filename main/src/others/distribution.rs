use std::ops::Range;
use textplots::{Chart, Plot, Shape};

use rand::{seq::SliceRandom, *};

const N: usize = (1 << 10);
const WEIGHT: usize = N;
const LOAD: usize = (1 << 8);

fn main() {
    let mut rng = rand::thread_rng();

    // let mut a = Element { coefs: [0; N] };
    // a.sample_uni(&mut rng);

    let mut x = Element { coefs: [0; N] };
    for i in 0..N {
        x.coefs[i] = rng.gen_range(0..29);
    }
    println!("x std: {}", x.std());

    let mut c = Element { coefs: [0; N] };
    // let mut c_length = 0;
    // let mut c_std: f32 = 0.0;
    for i in 0..N {
        let v: i32 = rng.gen_range(0..256);
        // c_length += v;
        // c_std += v.pow(2) as f32;
        c.coefs[i] = v;
    }
    // // println!("c length: {c_length}");
    // // c_std = c_std.sqrt() / (N as f32).sqrt();
    // // println!("c std: {}", c_std);

    // let mut c_sum_std = 0.0;
    // let count = 1 << 20;
    // for i in 0..count {
    //     // randomize challenge
    //     // c.coefs.shuffle(&mut rng);
    //     let mut c_sum = 0;
    //     for j in 0..N {
    //         c.coefs[j] = rng.gen_range(0..20);
    //         c.coefs[j] *= rng.gen_range(0..2) * 2 - 1;
    //         c_sum += c.coefs[j];
    //     }
    //     c_sum_std += c_sum.pow(2) as f32;
    // }
    // let real_c_sum_std = c_sum_std.sqrt() / (count as f32).sqrt();
    // println!("sum c std: {real_c_sum_std}");
    // println!("l2: {}", c.l2());

    // INEQUALITY
    const COUNT: usize = 1 << 12;
    let mut points_lo = [(0.0, 0.0); COUNT + 1];
    let mut points_hi = [(0.0, 0.0); COUNT + 1];
    let mut points_sum = [(0.0, 0.0); COUNT + 1];
    let mut dot_std = 0.0;
    let mut c_sum_std = 0.0;
    for i in 0..COUNT {
        // randomize the challenge
        // c.coefs.shuffle(&mut rng);
        let mut c_sum = 0;
        for j in 0..N {
            c.coefs[j] = rng.gen_range(0..2);
            c.coefs[j] *= rng.gen_range(0..2) * 2 - 1;
            c_sum += c.coefs[j];
        }
        c_sum_std += c_sum.pow(2) as f32;
        // take dot product with x
        let mut sum: i32 = 0;
        for i in 0..N {
            sum += x.coefs[i] * c.coefs[i];
        }
        // calculate std of output
        let b = 6;
        let data1 = ((sum << 32 - b) as u32 >> 32 - b) as i32;
        let data2 = sum >> b;
        let calc = data1 + data2 * (2 as i32).pow(b);
        // println!("lo: {data1:b}:{data1}, hi: {data2:b}:{data2}, sum: {sum:b}:{sum}, calc: {calc:b}:{calc}",);
        assert_eq!(calc, sum);
        points_lo[i] = ((i + 1) as f32, data1 as f32);
        points_hi[i] = ((i + 1) as f32, data2 as f32);
        points_sum[i] = ((i + 1) as f32, calc as f32);
        dot_std += sum.pow(2) as f32;
    }
    let real_dot_std = dot_std.sqrt() / (COUNT as f32).sqrt();
    let real_c_sum_std = c_sum_std.sqrt() / (COUNT as f32).sqrt();
    println!("sum std: {}", real_dot_std);
    println!("c sum std: {}", real_c_sum_std);
    println!(
        "proposition: (sum_std) = (x std)(c_sum_std): {}",
        x.std() * c.l2()
    );

    // println!("{:?}", points);
    // let mut c = Element { coefs: [0; N] };
    // for _ in 0..(1 << 4) {
    //     c.sample_chall(&mut rng);
    //     let prod = a.mult_chall(&c);
    // }

    // // let mut points = [(0.0, 0.0); N + 1];
    // // println!("a: {}", a.l2());
    // let mut avg: f32 = 0.0;

    // println!("var: {}", gsum / count);

    Chart::new(300, 60, 0.0, COUNT as f32)
        .lineplot(&Shape::Points(&points_lo))
        .display();
    Chart::new(300, 60, 0.0, COUNT as f32)
        .lineplot(&Shape::Points(&points_hi))
        .display();
    Chart::new(300, 100, 0.0, COUNT as f32)
        .lineplot(&Shape::Points(&points_sum))
        .display();
}

#[derive(Debug)]
struct Element {
    coefs: [i32; N],
}

impl Element {
    fn std(&self) -> f32 {
        self.l2() / (N as f32).sqrt()
    }
    fn l2(&self) -> f32 {
        let mut sum = 0;
        for coef in self.coefs {
            sum += coef.pow(2);
        }
        (sum as f32).sqrt()
    }
    fn frma(&mut self, a: &Self, rot: usize, mult: i32) {
        for i in 0..rot {
            self.coefs[i] -= mult * a.coefs[N - 1 - i];
        }
        for i in rot..N {
            self.coefs[i] += mult * a.coefs[i - rot];
        }
    }
    fn mult_chall(&self, chall: &Self) -> Self {
        let mut prod = Self { coefs: [0; N] };
        for rot in 0..N {
            if chall.coefs[rot] != 0 {
                prod.frma(self, rot, chall.coefs[rot]);
            }
        }
        prod
    }
    fn sample_uni<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        for i in 0..N {
            let load_range: Range<i32> = (1 - (LOAD as i32) / 2)..(LOAD as i32 / 2 + 1);
            let uniform = rng.gen_range(load_range);
            self.coefs[i] = uniform;
        }
    }
    fn sample_chall<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        // for i in 0..N {
        self.coefs = [0; N];
        for i in 0..WEIGHT {
            // let loc = rng.gen_range(0..(N - i));
            let value = rng.gen_range(0..2) * 2 - 1;
            // println!("{}", loc);
            self.coefs[i] = value;
        }
        // }
    }
    // fn sample_chall() -> Self {
    //     let mut elem = Self { coefs: [0; N] };
    //     // let v0: i32 = if rng.gen::<bool>() { 1 } else { -1 };
    //     let bits: [bool; WEIGHT] = rng.gen(0..2);
    //     // coefs = bits.map(|bit| if bit { 1 } else { -1 });
    //     for i in 0..bits.len() {
    //         elem.coefs[i] = if bits[i] { 1 } else { -1 }
    //     }
    //     elem
    // }
}
