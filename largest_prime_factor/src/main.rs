fn main() {
    // For any factor of a number there is a co-factor.
    // This co-factor is the number divided by its co-factor.
    // A fast way to find the largest prime factor is to find the largest
    // factors and check their primality.
    // There are also some other optimizations in here.
    // You could also probably bound the divisor search by the square root of
    // the number, but that would require more type coercions here so I didn't.
    let mut i = 2;
    loop {
        if (600_851_475_143 as u64) % i == 0 && is_prime((600_851_475_143 as u64) / i) {
            println!("{}", (600_851_475_143 as u64) / i);
            break;
        }
        i += 1;
    }
}

fn is_prime(candidate: u64) -> bool {
    for i in 2..candidate/2 {
        if candidate % i == 0 { return false; }
    }
    return true;
}