fn main() {
    println!("Rust Bailey-Borwein-Plouffe");
    println!("Results limited to 48-decimal places.\nThis is the maximum available on a 64-bit floating precision value.");
    for n in 1..49u16 {
        let result: f64 = compute_pi(n);
        let width: usize = usize::from(n);
        println!("Computing to {:>2} decimals: {:.width$}", n, result)
    }
}

fn compute_pi(precision: u16) -> f64 {
    let mut pi: f64 = 0.0;
    for ki32 in 0..precision + 1 {
        let k: f64 = f64::from(ki32);
        pi = pi
            + ((f64::powf(16.0, k * -1.0))
                * (4.0 / (8.0 * k + 1.0)
                    - 2.0 / (8.0 * k + 4.0)
                    - 1.0 / (8.0 * k + 5.0)
                    - 1.0 / (8.0 * k + 6.0)));
    }
    pi
}

//todo implement optimized
//fn compute_pi_optimized(precision: u16) -> f64 {}

// todo implement spigot
// fn compute_spigot(precision: u32) -> u8 {}
