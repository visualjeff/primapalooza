# primapalooza
Name says it all.  Handy prime number functions implemented in Rust.

Add primapalooza dependency to your project Cargo.toml file:
```json
[dependencies]
primapalooza = "0.3.2"
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
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
