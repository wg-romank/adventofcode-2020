# Breaking cryptographic handshake or Advent of Code 2020.25

I have been checking out Advent of Code[1] puzzles for a while now.
I love the simplicity of that website: we give you input you owe us output.
No constrains on tools and languages used, as long as you get the right answer you are good.

Puzzles are ogranized in the progression from simplier ones to more difficult.
I have usually started from the beginning - as I have been using this website to learn new languages,
which usually required grasping new paradigm or a concept. Simple tasks get you going fast so when it gets to real deal you are more or less prepared.

This time I did not pick up any new languages but just wanted to improve my Rust skills, so I started at the end, day 25 of 2020th edition [2]. Puzzle gives you the task to reverse-egnineer cryptographic handshake between two entities.

I quickly coded brute-force solution, since I was using language runtime with native performance I thought that might just be enough for the task. But seeing my solution run for 5 minutes and did not finish I decided to take a different approach. Usually solutions for Advent of Code puzzles run within milliseconds, otherwise you are doing something wrong. 

After researching the problem online I found mentions of Baby-step-giant-step algorithm[3] that is finding the solution of equation that looks like so

```
a ^ x  = beta (mod n)
```

Where `x` - unknown exponent. Turns out cryptographic handshake was doing exactly that operation, it used some set value referred as `subject number` and repeatedly multiplied it by itself getting modulus of the result. Unknown exponent is the loop size in question.

Algorithm included modulus exponentiation operation, but that was easy to find on crates.io[4]. Another problem was that at certain point it included exponentiation to negative degree but my computation was running in `u64` and I did not really want to mess with floating-point arithmetics.

Luckily one french dude from 17th century has been looking into cyclic group arithmetics and discovered a thing called now Fermat's little theorem[5], which would greatly help us out.

Baby-step-giant-step requires to compute `a ^ (-mi)` and Fermat tells us that `a ^ n = a (mod n)`, if `n` is prime (which it is, thank you folks behind Advent of Code).

Then goes some juggling with degrees:

```
a ^ (-m)
a ^ (-2m) * a ^ m
a ^ (-2m) * a ^ mn
a ^ (mn - 2m)
a ^ (m * (n - 2))
```

To make above true, all computations must be `(mod n)` obviously. With last obstacle out of our way we have a beautiful solution that works in milliseconds. (That is algorithm and theory behind is beautiful of course, and I am just fleshing them out in Rust code)

```rust
fn baby_step_giant_step(beta: u64, a: u64, n: u64) -> u64 {
    let m = (n as f64).sqrt().ceil() as u64; // modulus is group order

    let lookup = (0..m)
        .map(|j| (mod_exp(a, j, n), j))
        .collect::<HashMap<u64, u64>>();

    let a_m = mod_exp(a, m * (n - 2), n);

    (0..m)
        .fold_while(beta, |gamma, i| {
            if let Some(j) = lookup.get(&gamma) {
                Done((i * m + j) % n)
            } else {
                Continue((gamma * a_m) % n)
            }
        })
        .into_inner()
}
```

References
- [1] https://adventofcode.com/
- [2] https://adventofcode.com/2020/day/25
- [3] https://en.wikipedia.org/wiki/Baby-step_giant-step
- [4] https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
