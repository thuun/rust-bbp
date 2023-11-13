# Rust BPP
This is an example implementation of the Bailey-Borwein-Plouffe (BPP) formula and specializations, including the *spigot*, to generate the $n$<sup>th</sup> hexadecimal (decimal) value of $\pi$, and thus $\pi$ itself (at least until you run out of computing resources :grin:).

This much more computationally cheap compared to other methods that must include preceding digits, but still is linearithmic (the further the value is, the longer it takes): $O(n\log n)$

The formula discovered in 1995 is:    

$`{\displaystyle \pi =\sum _{k=0}^{\infty }\left[{\frac {1}{16^{k}}}\left({\frac {4}{8k+1}}-{\frac {2}{8k+4}}-{\frac {1}{8k+5}}-{\frac {1}{8k+6}}\right)\right]}`$

Likewise, a spigot algorithim was defined which retrieves the hexadecimal value at decimal position $n$.

$`{\displaystyle \sum _{k=0}^{n}{\frac {16^{n-k}{\bmod {(}}8k+1)}{8k+1}}+\sum _{k=n+1}^{\infty }{\frac {16^{n-k}}{8k+1}}.}`$

$`{\displaystyle 16(4\Sigma _{1}-2\Sigma _{2}-\Sigma _{3}-\Sigma _{4})}`$

These three implementations are demonstrated in this application.

In this example, you can see the BPP formula used to calculate out the floating point value (base 10) in the `compute_pi_base10` function. Likewise, you can see the specific hexidecimal spigot algorithm in the `compute_spigot` function.

⚠️ The modular exponentiation function ultimately limits how far you can "look into the distance". In this exmample, it is limited to the range of unsigned 32-bit integers which using exponentiation doesn't get into the 100's of thousands for bases unfortunately. 
However, for the purposes of this exmample it gets the point across, you could optimize this as you see fit.

## Building & Running
Pretty standard :crab: rust:
```
cargo build
cargo test
cargo run
```