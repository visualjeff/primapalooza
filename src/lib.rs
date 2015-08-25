#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(test)]
//extern crate rand;
extern crate test;
extern crate num;
use test::Bencher;

#[test]
fn is_it_a_prime_number() {
    assert!(is_prime(5));
    assert!(is_prime_why_not(5));
}

#[test]
fn should_not_be_a_prime_number() {
    assert_eq!(false, is_prime(4));
    assert_eq!(false, is_prime_why_not(4));
}

#[test]
fn prime_factorization_test(){
    let mut vec = Vec::new();
    vec.push(2);
    vec.push(2);
    vec.push(2);
    vec.push(3);
    vec.push(7);
    assert_eq!(vec, prime_factorization(168));
    vec = Vec::new();
    vec.push(2);
    vec.push(3);
    vec.push(17);
    assert_eq!(vec, prime_factorization(102));
}

#[test]
fn get_next_prime_number_test() {
    assert_eq!(79, get_next_prime_number(73));
}

#[test]
fn get_twin_primes_test() {
    assert_eq!((5, 7), get_twin_primes(5));
}

#[test]
fn greatest_common_factor_test() {
    assert_eq!(6, greatest_common_factor(18,24));
}

#[test]
fn least_common_multiple_test() {
    assert_eq!(90, least_common_multiple(18,30));
}

#[test]
fn number_of_primes_test() {
    assert_eq!(489, number_of_primes(1,3500));
}

#[test]
fn number_of_factors_test() {
    assert_eq!(6, number_of_factors(28));
    assert_eq!(12, number_of_factors(72));
    assert_eq!(14, number_of_factors(8128));
}

#[test]
fn mersenne_prime_test() {
    assert_eq!(31, mersenne_prime(5));
    assert_eq!(1125899906842623, mersenne_prime(50));
}

#[test]
fn perfect_number_test() {
    //assert_eq!(false, perfect_number(1));
    assert!(perfect_number(2));
    assert!(perfect_number(3));
    assert_eq!(false, perfect_number(4));
    assert!(perfect_number(5));
    assert_eq!(false, perfect_number(6));
    assert!(perfect_number(7));
    assert_eq!(false, perfect_number(8));
    assert_eq!(false, perfect_number(9));
    assert_eq!(false, perfect_number(10));
    assert_eq!(false, perfect_number(11));
    assert_eq!(false, perfect_number(12));
    assert!(perfect_number(13));
    assert!(perfect_number(31));
    assert_eq!(false, perfect_number(41));
    //assert!(perfect_number(262144));
}

#[test]
fn get_nth_prime_test(){
    assert_eq!(853, get_nth_prime(29));
}

#[test]
fn generate_primes_test() {
    let result = generate_primes(3500);
    assert_eq!(489, result.len());
}

#[test]
fn is_lucky_number_test() {
    assert!(is_lucky_number(2));
    assert!(is_lucky_number(3));
    assert!(is_lucky_number(5));
    assert!(is_lucky_number(11));
    assert_eq!(false, is_lucky_number(13));
    assert!(is_lucky_number(17));
    assert!(is_lucky_number(41));
}

#[test]
fn is_triangular_number_test() {
    assert!(is_triangular_number(3));
    assert!(is_triangular_number(7));
    assert!(is_triangular_number(13));
    assert!(is_triangular_number(31));
}

//#[bench]
//fn bench_is_prime(b: &mut Bencher) {
//    b.iter(|| is_prime(rand::random::<i64>()));
//}

/// Tests for prime numbers.
///
/// # Examples
///
/// ```
/// use primapalooza::is_prime;
/// 
/// println!("{}", primapalooza::is_prime(5));
/// 
pub fn is_prime(x: usize) -> bool {
     if x < 2 {
         panic!("requires a positive integer greater then 1");
     }
     let mut start: usize = 2;
     while start <= (x as f64).sqrt() as usize {
         if x % start < 1 {
             return false;
         }
         start += 1;
     }
     return x > 1;
}


//#[bench]
//fn bench_is_prime_why_not(b: &mut Bencher) {
//      b.iter(|| 
//             for n in (0..10) {
//                test::black_box(is_prime_why_not(n));
//             });
//}

/// Tests for prime numbers and state why its not a prime number.
///
/// # Examples
///
/// ```
/// use primapalooza::is_prime_why_not;
/// 
/// println!("{}", primapalooza::is_prime_why_not(9));
/// 
pub fn is_prime_why_not(x: u64) -> bool {
    if x < 2 {
        panic!("requires a positive integer greater then 1");
    }
    let mut start: u64 = 2;
    let mut return_value: bool = true;
    while start <= (x as f64).sqrt() as u64 {
        let m = (x as u64) % start; 
        if m < 1 {
            println!("{} is divisible by {}.  {} * {} = {}", x, start, start, ((x as u64) / start), x);
            return_value = false;
        }
        start += 1;
    }
    if return_value == true { 
        return_value = x > 1;
    }
    return return_value;
}

//#[bench]
//fn bench_get_next_prime_number(b: &mut Bencher) {
//    b.iter(|| 
//           for n in (1..10) {
//               test::black_box(get_next_prime_number(n));
//           });
//}

/// Returns the next available prime number.
///
/// # Examples
///
/// ```
/// use primapalooza::get_next_prime_number;
/// 
/// println!("{}", primapalooza::get_next_prime_number(9));
/// 
pub fn get_next_prime_number(n:usize) -> usize {
    if n < 2 {
        panic!("requires a positive integer greater then 1");
    }
    let mut x: usize = n;
    x += 1;
    while !is_prime(x) {
        x += 1;
    }
    return x;
}

/// Returns the twin primes.  A pair of prime numbers that differ by 2.
///
/// # Examples
///
/// ```
/// use primapalooza::get_twin_primes;
/// 
/// println!("{:?}", primapalooza::get_twin_primes(5));
/// 
pub fn get_twin_primes(n: usize) -> (usize, usize){
    if !is_prime(n) {
        panic!("prime number required as input");
    }
    return (n, get_next_prime_number(n));
}

//#[bench]
//fn bench_prime_factorization(b: &mut Bencher) {
//    b.iter(||
//           for n in (1..10) {
//                test::black_box(prime_factorization(n));
//           });
//}

/// Perform a prime factorization.
///
/// # Examples
///
/// ```
/// use primapalooza::prime_factorization;
/// 
/// println!("{:?}", primapalooza::prime_factorization(168));
/// 
pub fn prime_factorization(n:usize) -> Vec<usize> {
    if n < 2 {
        panic!("requires a positive integer greater then 1");
    }
    let mut x:usize = n;
    let mut return_value = Vec::new();
    let mut start:usize = 2;
    while !is_prime(x) {
        let m = x % start;
        if m < 1 {
            return_value.push(start);
            x = x / start;
        } else {
            start = get_next_prime_number(start);                                                                  }
    }
    return_value.push(x);
    return return_value;
}


/// Perform a greatest common factor(gcf) calculation.
///
/// # Examples
///
/// ```
/// use primapalooza::greatest_common_factor;
/// 
/// println!("{}", primapalooza::greatest_common_factor(18, 24));
/// 
pub fn greatest_common_factor(x:usize, y:usize) -> usize {
    if x < 2 || y < 2 {
        panic!("requires a positive integers greater then 1");
    }
    let mut vec_x = prime_factorization(x);
    let mut vec_y = prime_factorization(y);
    vec_x.dedup();
    vec_y.dedup();
    let mut m:Vec<usize> = Vec::new();
    for p in vec_x.iter() {
        for q in vec_y.iter() {
            if p == q {
                m.push(*p);
            }
        }
    }
    let mut return_value = 1;
    for k in &m {
        return_value = return_value * k;    
    }
    return return_value;
}


/// Perform a least common multiple(lcm) calculation.
///
/// # Examples
///
/// ```
/// use primapalooza::least_common_multiple;
/// 
/// println!("{}", primapalooza::least_common_multiple(18, 24));
/// 
pub fn least_common_multiple(x:usize, y:usize) -> usize {
    if x < 2 || y < 2 {
        panic!("requires a positive integers greater then 1");
    }
    let gcf = greatest_common_factor(x, y);
    if x % gcf < 1 {
        (x/gcf) * y
    } else {
        (y/gcf) * x
    }
}

/// Number of prime numbers within a range.
///
/// # Examples
///
/// ```
/// use primapalooza::number_of_primes;
/// 
/// println!("{}", primapalooza::number_of_primes(1, 3500));
/// 
pub fn number_of_primes(x: usize, y: usize) -> usize {
    if x < 1 {
        panic!("requires a positive integers");
    }
    if y < 2 {
        panic!("requires a positive integer greater than 1");
    }
    let mut a = x;
    if a == 1 {
        a += 1;
    }
    let mut z = 0;
    for n in a..y {
        if is_prime(n) {
            z += 1
        }
    }
    return z;
}

//#[bench]
//fn bench_number_of_factors(b: &mut Bencher) {
//    b.iter(|| 
//           for n in (2..10000) {
//               test::black_box(number_of_factors(n));
//           });
//}

/// The number of factors in a number.
///
/// # Examples
///
/// ```
/// use primapalooza::number_of_factors;
/// 
/// println!("{}", primapalooza::number_of_factors(72));
/// 
pub fn number_of_factors(x: usize) -> usize {
    if x < 2 {
        panic!("requires a positive integer greater then 1");
    }
    let mut f = Vec::new();
    let pf = prime_factorization(x);
    let mut count = 1;
    let mut previous_pf = 0;
    for z in pf {
        if z != previous_pf {
            if previous_pf != 0 {
                f.push(count);
            }
            count = 1;
        } else {
            count += 1;
        }
        previous_pf = z;
    }
    f.push(count);

    let mut nof = 1;
    for i in f {
        nof = nof * (i + 1)
    }
    return nof;
}


/// Generate a mersenne prime from a prime number.
///
/// # Examples
///
/// ```
/// use primapalooza::mersenne_prime;
/// 
/// let results = primapalooza::mersenne_prime(5);
pub fn mersenne_prime(n: usize) -> usize {
    if n < 2 {
        panic!("requires a positive integer greater then 1");
    }
    let two:usize = 2;
    let result = num::pow(two, n as usize) - 1;
    return result;
}


/// Is a number a perfect number.  The sum of its factors is the number itself.
///
/// # Examples
///
/// ```
/// use primapalooza::perfect_number;
/// 
/// println!("{}", primapalooza::perfect_number(5));
pub fn perfect_number(n:usize) -> bool {
    return is_prime(mersenne_prime(n as usize));
}


/// Get the nth prime using Euler's. Up to 41st prime.
///
/// # Examples
///
/// ```
/// use primapalooza::get_nth_prime;
/// 
/// println!("{}", primapalooza::get_nth_prime(5));
pub fn get_nth_prime(n:usize) -> usize {
    if n > 41 {
        panic!("n needs to be less than 41");
    }
    return (n.pow(2) - n) + 41;
}

/// Generate a collection of prime numbers up to a particular limit.
///
/// # Examples
///
/// ```
/// use primapalooza::generate_primes;
/// 
/// let results = primapalooza::generate_primes(1000);
/// for x in results.iter() {
///     println!("{}", x);
/// }
pub fn generate_primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        panic!("requires a positive integer greater then 1");
    }
    let sqrtlim:usize = (limit as f64).sqrt() as usize;
    let mut pp:usize = 2; 
    let mut ss = vec![pp];
    let mut ep = vec![pp];
    pp += 1;
    let mut rss = vec![ss[0]];
    let mut tp = vec![pp];
    let mut i = 0;
    let ssr = rss[i] * tp[0];
    rss.push(ssr);
    let mut xp = vec![];
    pp += ss[0];
    let mut npp: usize = pp;
    tp.push(npp);
    while npp < limit {
        i += 1;
        while npp < (rss[i] + 1) {
            for n in ss.iter() {
                npp = pp + n;
                if npp > limit {
                    break;
                }
                if npp <= (rss[i] + 1) {
                    pp = npp;
                }
                let sqrtnpp = (npp as f64).sqrt();
                let mut test = true;
                for q in tp.iter() {
                    if sqrtnpp < (*q as f64) { 
                        break;
                    } else if npp % q == 0 {
                        test = false;
                        break;
                    }
                }
                if test {
                    if npp <= sqrtlim { 
                        tp.push(npp);
                    } else { 
                        xp.push(npp);
                    }
                }
            }
            if npp > limit { 
                break;
            }
        }
        if npp > limit { 
            break;
        }
        let mut lrpp = pp;
        let mut nss = vec![];
        while pp < (rss[i] + 1) * 2 - 1 {
            for n in ss.iter() {
                npp = pp + n;
                    if npp > limit { 
                        break;
                    }
                    let sqrtnpp = (npp as f64).sqrt();
                    let mut test = true;
                    for q in tp.iter() {
                        if sqrtnpp < (*q as f64) { 
                            break; 
                        } else if npp % q == 0 {
                            test = false;
                            break;
                        }
                    }
                    if test {
                        if npp <= sqrtlim { 
                            tp.push(npp);
                        } else { 
                            xp.push(npp);
                        }
                    }
                    if npp % tp[0] != 0 {
                        nss.push(npp - lrpp);
                        lrpp = npp;
                    }
                    pp = npp;            
                    if npp % tp[0] != 0 {
                        nss.push(npp - lrpp);
                        lrpp = npp;
                    }
                if npp > limit { 
                    break;
                }
            }
            if npp > limit { 
                break;
            }
        }
        ss = nss;             
        ep.push(tp[0]);             
        tp.remove(0);
        let ssr = rss[i] * tp[0];
        rss.push(ssr);
        npp = lrpp;
    }
    ep.reverse();
    for a in ep.iter() {
        tp.insert(0, *a);
    }
    tp.reverse();
    for a in tp.iter() {
        xp.insert(0, *a);
    }
    xp.dedup();
    return xp;
}


/// According to Euler is your prime number a lucky number?
///
/// # Examples
///
/// ```
/// use primapalooza::is_lucky_number;
/// 
/// println!("{}", primapalooza::is_lucky_number(5));
pub fn is_lucky_number(p:usize) -> bool {
    if !is_prime(p) {
        panic!("prime number required as input");
    }
    let mut n:usize = 1;
    let mut result = true;
    while (n < p) && (result == true) {
        result = is_prime((n.pow(2) - n) + p);
        n += 1;
    }
    return result;
}

/// Is_this_number a triangular number?
///
/// # Examples
///
/// ```
/// use primapalooza::is_triangular_number;
/// 
/// println!("{}", primapalooza::is_triangular_number(5));
pub fn is_triangular_number(n: usize) -> bool {
    perfect_number(n)
}
