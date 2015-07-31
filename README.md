# primapalooza
Name says it all.  Handy prime number functions implemented in Rust.

Add primapalooza dependency to your project Cargo.toml file:
```json
[dependencies]
primapalooza = "0.1.9"
```

Add primapalooza to your project's source code:
```rust
extern crate primapalooza;

use primapalooza::is_prime;
use primapalooza:generate_primes;
```

Methods signatures:
```rust
fn is_prime(i32) -> bool
fn is_prime_why_not(i32) -> bool
fn perfect_number(n:i32) -> bool
fn get_next_prime_number(i32) -> i32
fn get_twin_primes(i32) -> (i32, i32)
fn prime_factorization(i32) -> Vec<i64>
fn greatest_common_factor(i32, i32) -> i32
fn least_common_multiple(i32, i32) -> i32
fn number_of_primes(i32, i32) -> i32
fn number_of_factors(i32) -> i32
fn mersenne_prime(i32) -> i32
fn get_nth_prime(i32) -> i32
fn generate_primes(i64) -> Vec<i64>
```
