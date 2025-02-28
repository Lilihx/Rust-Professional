fn gen_prime(limit: u128) -> Vec<u128> {
    let mut primes: Vec<u128> = Vec::new();
    let mut is_prime: Vec<bool> = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=limit {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = i * i;
            while j <= limit {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    primes
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut number = number;
    let mut max_prime = 0;
    let primes = gen_prime(10_000_000);
    for &prime in &primes {
        while number % prime == 0 {
            number /= prime;
            max_prime = prime;
        }
    }
    // 求factor
    let mut factor = *primes.last().unwrap();
    if (number as f64).sqrt() as u128 > *primes.last().unwrap() as u128 {
        factor = primes.last().unwrap().pow(2) as u128;
    }
    while factor * factor <= number {
        while number % factor == 0 {
            number /= factor;
            max_prime = factor;
        }
        while number % (factor + 2) == 0 {
            max_prime = factor + 2;
            number /= factor + 2;
        }
        factor += 6;
    }
    number.max(max_prime)
}
