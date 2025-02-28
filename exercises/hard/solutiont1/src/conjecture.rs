use std::collections::HashMap;

// 一个奇合数 = 1个素数 + 2 * 一个数的平方
pub fn goldbach_conjecture() -> String {
    let mut res: Vec<i32> = Vec::new();
    let mut num = 1;
    let mut prime_cache: HashMap<i32, bool> = HashMap::new();
    loop {
        num += 1;
        if num % 2 == 0 {
            continue;
        }
        let prime = prime_cache.entry(num).or_insert(is_prime(num));
        if *prime {
            continue;
        }
        let mut gold_batch = false;
        let mut tmp = 1;
        loop {
            let tmp_2 = num - 2 * tmp * tmp;
            if tmp_2 <= 0 {
                break;
            }
            let prime = prime_cache.entry(tmp_2).or_insert(is_prime(tmp_2));
            if *prime {
                gold_batch = true;
                break;
            }
            tmp += 1;
        }
        if !gold_batch {
            res.push(num);
        }
        if res.len() == 2 {
            break;
        }
    }
    format!("{},{}", res[0], res[1])
}

// check 一个数是不是素数
pub fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let mut index = 2;
    while index * index <= num {
        if num % index == 0 {
            return false;
        }
        index += 1;
    }
    true
}
