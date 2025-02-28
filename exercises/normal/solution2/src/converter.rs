pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (num_str, from_base) = parse_num_str(num_str);
    let num_10 = convert_to_10(&num_str, from_base);
    convert_from_10(num_10, to_base)
}

fn parse_num_str(num_str: &str) -> (String, u32) {
    if let Some(left) = num_str.find('(') {
        if let Some(right) = num_str.find(')') {
            let r = &num_str[0..left];
            let base: u32 = num_str[left + 1..right].parse().unwrap();
            return (String::from(r), base);
        }
    }
    panic!("Invalid input")
}

fn convert_to_10(num_str: &str, from_base: u32) -> i32 {
    let mut num_10: i32 = 0;
    let mut base: i32 = 1;
    for c in num_str.chars().rev().into_iter() {
        if let Some(digit) = c.to_digit(10) {
            num_10 += (digit as i32) * (base as i32);
            base *= from_base as i32;
        }
    }
    num_10
}

fn convert_from_10(num: i32, to_base: u32) -> String {
    let mut s: Vec<i32> = Vec::new();
    let mut num_10 = num;
    while num_10 > 0 {
        let digit = num_10 % (to_base as i32);
        num_10 /= to_base as i32;
        s.push(digit);
    }
    s.reverse();
    let mut res = String::new();
    for d in s {
        res.push(char::from_digit(d as u32, to_base).unwrap());
    }
    res
}
