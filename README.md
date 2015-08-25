# primapalooza
Name says it all.  Handy prime number functions implemented in Rust.

Add primapalooza dependency to your project Cargo.toml file:
```json
[dependencies]
primapalooza = "0.3.0"
```

Add primapalooza to your project's source code:
```rust
extern crate primapalooza;

use primapalooza::is_prime;
use primapalooza:generate_primes;
```

Methods signatures:
```rust
fn is_prime(usize) -> bool
fn is_prime_why_not(usize) -> bool
fn perfect_number(usize) -> bool
fn get_next_prime_number(usize) -> usize
fn get_twin_primes(usize) -> (usize, usize)
fn prime_factorization(usize) -> Vec<usize>
fn greatest_common_factor(usize, usize) -> usize
fn least_common_multiple(usize, usize) -> usize
fn number_of_primes(usize, usize) -> usize
fn number_of_factors(usize) -> usize
fn mersenne_prime(usize) -> usize
fn get_nth_prime(usize) -> usize
fn generate_primes(usize) -> Vec<usize>
fn is_lucky_number(usize) -> bool
fn is_triangular_number(usize) -> bool
```
