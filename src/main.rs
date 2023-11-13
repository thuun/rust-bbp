fn main() {
    println!("Rust Bailey-Borwein-Plouffe\n");
    println!("Running compute_pi_base10.");
    println!("Results limited to 15-decimal places.\nThis is the maximum available on a 64-bit floating point precision value.");
    for n in 1..16u16 {
        let result: f64 = compute_pi_base10(n);
        let width: usize = usize::from(n);
        println!("Computing to {:>2} decimals: {:.width$}", n, result)
    }
    let mut output: String = String::from("Spigot to 64 decimals: 3.");
    for n in 1..64u32 {
        output.push_str(&format!("{:X}", compute_spigot(n)));
    }
    println!("{}", output);
    println!("Spigot at 100th decimal: {:X}", compute_spigot(100));
    println!("Spigot at 1,000th decimal: {:X}", compute_spigot(1000));
    println!("Spigot at 4,000th decimal: {:X}", compute_spigot(4000));
}

fn compute_pi_base10(precision: u16) -> f64 {
    let mut pi: f64 = 0.0;
    for ki32 in 0..precision + 1 {
        let k: f64 = f64::from(ki32);
        pi = pi
            + (f64::powf(16.0, k * -1.0)
                * (4.0 / (8.0 * k + 1.0)
                    - 2.0 / (8.0 * k + 4.0)
                    - 1.0 / (8.0 * k + 5.0)
                    - 1.0 / (8.0 * k + 6.0)));
    }
    pi
}

// Rob Cobb
// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u32, mut exp: u32, modulus: u32) -> u32 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn sigma(n: u32, j: u32) -> f64 {
    let mut s: f64 = 0.0;
    let mut denom = j;
    for k in 0..n + 1 {
        let r = mod_pow(16, n - k, denom);
        s = fract_mod(s + f64::from(r) / f64::from(denom));
        denom += 8u32;
    }
    let mut num = 1.0 / 16.0;
    //replace EPSILON term with eps abstract later. Requires f64::next_up which is experimental currently.
    while num / f64::from(denom) > f64::EPSILON {
        s += num / f64::from(denom);
        num /= 16.0;
        denom += 8u32;
    }
    s.fract()
}

fn fract_mod(v: f64) -> f64 {
    if v < 0.0 {
        return 1.0 + v.fract() % 1.0;
    }
    v.fract() % 1.0
}

fn f64_to_u8(value: f64) -> u8 {
    let clamped_value = value.round().max(u8::MIN as f64).min(u8::MAX as f64);
    clamped_value as u8
}

fn compute_spigot(digit: u32) -> u8 {
    let r = fract_mod(
        4.0 * sigma(digit - 1, 1) //multiplier correct, sigma result wrong
            - (2.0 * sigma(digit - 1, 4))
            - sigma(digit - 1, 5)
            - sigma(digit - 1, 6),
    );
    f64_to_u8((16.0 * r).floor())
}
