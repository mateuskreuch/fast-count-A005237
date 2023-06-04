# Fast count A005237

The integer sequence [A005237](https://oeis.org/A005237) is the sequence of numbers N such that N and N + 1 have the same amount of divisors. This repository is concerned with _counting_ A005237: how many numbers N from 0 to K exist that satisfy σ(N) = σ(N + 1)? (where σ is the [divisor function](https://en.wikipedia.org/wiki/Divisor_function))

This problem is being analyzed because although the naive solution to it is quite simple, making it fast involves some pretty elegant number theory. There are three implemented solutions in Rust; the naive one, for comparison; a fast one, and a faster one.