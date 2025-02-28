pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let day = 365;
    let mut probability = 1.0;
    for i in 0..n {
        probability *= (day - i) as f64 / day as f64;
    }
    1.0 - probability
}
