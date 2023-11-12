# Rust BPP
This is an example implementation of the Bailey-Borwein-Plouffe (BPP) formula and specializations, including the *spigot*, to generate the $n$<sup>th</sup> hexidecimal (decimal) value of $\pi$, and thus $\pi$ itself (at least until you run out of computing resources :grin:).

This much more computationally cheap compared to other methods that must include preceding digits, but still is linearithmic (the further the value is, the longer it takes): $O(n\log n)$

The formula discovered in 1995 is:    

$`{\displaystyle \pi =\sum _{k=0}^{\infty }\left[{\frac {1}{16^{k}}}\left({\frac {4}{8k+1}}-{\frac {2}{8k+4}}-{\frac {1}{8k+5}}-{\frac {1}{8k+6}}\right)\right]}`$

This gave rise to specializations in the formula optimized for computing ($s$, $b$, $m$, & $A$ are `integers`):    

$`{\displaystyle P(s,b,m,A)=\sum _{k=0}^{\infty }\left[{\frac {1}{b^{k}}}\sum _{j=1}^{m}{\frac {a_{j}}{(mk+j)^{s}}}\right]}`$

Likewise, a spigot algorithim was defined which retrieves the hexidecimal value at decimal position $n$.

$`{\displaystyle P(s,b,m,A)=\sum _{k=0}^{\infty }\left[{\frac {1}{b^{k}}}\sum _{j=1}^{m}{\frac {a_{j}}{(mk+j)^{s}}}\right]}`$

These three implementations are demonstrated in this application.

## Building & Running
Pretty standard :crab: rust:
```
cargo build
cargo test
cargo run
```