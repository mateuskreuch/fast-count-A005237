use std::time::SystemTime;

//----------------------------------------------------------------------------//

fn the_naive_one(mut k: usize) -> usize {
   // Although A005237 tests forwards, that is, if σ(N) = σ(N + 1), it is more
   // efficient to test backwards. To correct for this, we must shift the range
   // from [1, k] to [2, k + 1]
   k += 1;

   let mut count = 0;
   let mut last_amount_of_factors = 1;
   
   for n in 2..=k {
      let amount_of_factors = count_factors(n);
      
      if amount_of_factors == last_amount_of_factors {
         count += 1;
      }

      last_amount_of_factors = amount_of_factors;
   }

   count
}

fn count_factors(n: usize) -> usize {
   let mut count = 2;

   // We can use a trick here; all the divisors after the square root of N will
   // be mirrored, so we just count them twice. For example, 16: 2x8, 4x4, 8x2
   for i in 2..=(n as f64).sqrt() as usize {
      if n % i == 0 {
         if i != n / i { count += 2; }
         else          { count += 1; }
      }       
   }

   count
}

//----------------------------------------------------------------------------//

/*
To speed up the naive solution, we use prime factorization. That is, σ(N) can
be discovered using the exponents of the primes that compose N. If a number is
given by the following prime factorization:

   x = P1^y1 * P2^y2 ...* Pn^yn

Then the amount of divisors is given by:

   σ(x) = (y1 + 1) * (y2 + 1) ...* (yn + 1)

With this, we simply need an array to keep the amount of factors of every number
from 1 to K. At each prime number P we iterate through its multiples M, find the
exponent of P that composes M, and apply the formula. The order in which we do
this doesn't matter, since multiplication is commutative.

Finding the primes is trivial: since we start at the smallest number and go up,
once we reach a number we'll have gone through all of its divisors. So, if the
number still only has one divisor, it's a prime.
*/
fn the_fast_one(mut k: usize) -> usize {
   k += 1;

   let mut count = 0;
   let mut factors = vec!(1; k + 1);

   for n in 2..=k {
      if factors[n] == 1 {
         for i in (n..=k).step_by(n) {
            factors[i] *= find_exponent(i, n) + 1;
         }
      }

      if factors[n] == factors[n - 1] {
         count += 1;
      }
   }

   count
}

fn find_exponent(mut n: usize, factor: usize) -> usize {
   let mut exponent = 0;

   while n % factor == 0 {
      n /= factor;
      exponent += 1;
   }

   exponent
}

//----------------------------------------------------------------------------//

/*
There's only one bottleneck remaining: finding the exponent. There's a trick 
here too, because the sequence of exponents is given by the recurrence:

                 <-------- n - 1 times -------->
   ε(n, f + 1) = ε(n, f) ⧺ ε(n, f) ...⧺ ε(n, f) ⧺ (ε(n, f) + 1)
   ε(n, 0) = 1

Where ⧺ means concatenation. For example:

ε(2, _) = 1, 12, 1213, 12131214, 1213121412131215...
ε(3, _) = 1, 112, 112112113, 112112113112112113112112114...
ε(5, _) = 1, 11112, 1111211112111121111211113...

That is, for N = 2, the first four exponents are 1, 2, 1 and 3:

2 = 2^1
4 = 2^2
6 = 2^1 (+ 3^1)
8 = 2^3

It is important to notice that this pattern cannot be stored in number form in
a number system with a finite base, as it will break once it reaches that base.

Regardless, since this is concatenation it's very easily parallelizable.
The current code does not go that far, but the structure is there.
*/
fn the_faster_one(mut k: usize) -> usize {
   k += 1;

   let mut count = 0;
   let mut factors = vec!(1; k + 1);

   for n in 2..=k {
      if factors[n] == 1 {
         let biggest_exponent = k.checked_ilog(n).unwrap();

         for i in 1..=biggest_exponent {
            let step = n.pow(i);

            // Imagine K = 81, then from 3..K in steps of 3 the exponents are:
            // 112112113112112113112112114
            // The exponents can be decomposed individually:
            // 11 11 11 11 11 11 11 11 11
            //   2  2     2  2     2  2
            //         3        3
            //                           4
            // What can be observed then, is that they all follow the same
            // pattern: in steps of the next exponent, map to the right N - 1
            // times in steps of current exponent
            for j in (step..=k).step_by(n * step) {
               let until = j + (n - 1)*step;

               for k in (j..until.min(k + 1)).step_by(step) {
                  factors[k] *= i + 1;
               }
            }
         }
      }

      if factors[n] == factors[n - 1] {
         count += 1;
      }
   }

   count
}

//----------------------------------------------------------------------------//

fn main() {
   // check correctness
   for n in 2..=1000 {
      let a = the_naive_one(n);
      let b = the_fast_one(n);
      let c = the_faster_one(n);

      if !(a == b && b == c) {
         println!("algorithms dont match: {0} {1} {2} at {3}", a, b, c, n);
      }
   }

   let x = 1000000;
   let t = SystemTime::now();

   println!("{0}", the_naive_one(x));
   println!("the naive one took {0}ms", t.elapsed().unwrap().as_millis());

   let t = SystemTime::now();

   println!("{0}", the_fast_one(x));
   println!("the fast one took {0}ms", t.elapsed().unwrap().as_millis());

   let t = SystemTime::now();

   println!("{0}", the_faster_one(x));
   println!("the faster one took {0}ms", t.elapsed().unwrap().as_millis());
}
