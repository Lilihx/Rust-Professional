pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut res: u32 = 0;
    let mut f_n_2: u32 = 0;
    let mut f_n_1: u32 = 1;
    let mut f_n: u32 = 1;
    while f_n < threshold {
        if f_n % 2 != 0 {
            res += f_n;
        }
        f_n = f_n_2 + f_n_1;
        f_n_2 = f_n_1;
        f_n_1 = f_n;
    }
    res
}
