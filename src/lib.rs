#[test]
fn is_it_a_prime_number() {
    assert_eq!(true, is_prime(5));
}
#[test]
fn should_not_be_a_prime_number() {
    assert_eq!(false, is_prime(4));
}

#[test]
fn one_is_not_a_prime_number() {
    assert_eq!(false, is_prime(1));
}


/// Tests for prime numbers.
///
/// # Examples
///
/// ```
/// use prime::is_prime;
/// 
/// println!("{}", prime::is_prime(5));
/// 
pub fn is_prime(x: i32) -> bool {
    let mut start: i64 = 2;
    while start <= (x as f64).sqrt() as i64 {
        if (x as i64) % start < 1 {
            return false;
        }
        start += 1;
    }
    return x > 1;
}

