use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[test]
fn test_first_prime2() {
    assert_eq!(np::nth_with_caching(0), 2);
}

#[test]
fn test_second_prime2() {
    assert_eq!(np::nth_with_caching(1), 3);
}

#[test]
fn test_sixth_prime2() {
    assert_eq!(np::nth_with_caching(5), 13);
}

#[test]
fn test_big_prime2() {
    assert_eq!(np::nth_with_caching(10_000), 104_743);
}

#[test]
fn test_is_prime_2() {
    assert_eq!(np::is_prime(2), true);
}

#[test]
fn test_is_prime_3() {
    assert_eq!(np::is_prime(3), true);
}

#[test]
fn test_is_not_prime_4() {
    assert_eq!(np::is_prime(4), false);
}

#[test]
fn test_is_not_prime_28() {
    assert_eq!(np::is_prime(28), false);
}

#[test]
fn test_is_prime_29() {
    assert_eq!(np::is_prime(29), true);
}


#[test]
fn test_is_not_prime_104_742() {
    assert_eq!(np::is_prime(104_742), false);
}

#[test]
fn test_is_prime_104_743() {
    assert_eq!(np::is_prime(104_743), true);
}

#[test]
fn test_is_prime_2_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(2, &ps), true);
}

#[test]
fn test_is_prime_3_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(3, &ps), true);
}

#[test]
fn test_is_not_prime_4_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(4, &ps), false);
}

#[test]
fn test_is_not_prime_28_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(28, &ps), false);
}

#[test]
fn test_is_prime_29_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(29, &ps), true);
}

#[test]
fn test_is_not_prime_104_742_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(104_742, &ps), false);
}

#[test]
fn test_is_prime_104_743_plus() {
    let ps: Vec<u32> = vec!();
    assert_eq!(np::is_prime_plus(104_743, &ps), true);
}
