# primapalooza
Name says it all.  Handy prime number functions implemented in Rust.

Add primapalooza dependency to your project Cargo.toml file:
```json
[dependencies]
primapalooza = "0.1.6"
```

Add primapalooza to your project's source code:
```rust
extern crate primapalooza;

use primapalooza::is_prime;
use primapalooza:generate_primes;
```

Methods signatures:
```rust
fn is_prime(x: i32) -> bool
fn is_prime_why_not(x: i32) -> bool
fn get_next_prime_number(n:i32) -> i32
fn prime_factorization(n:i32) -> Vec<i64>
fn greatest_common_factor(x:i32, y:i32) -> i32
fn least_common_multiple(x:i32, y:i32) -> i32
fn number_of_primes(x: i32, y: i32) -> i32
fn number_of_factors(n: i32) -> i32
fn generate_primes(limit: i64) -> Vec<i64>
```
