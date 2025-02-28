pub fn dp_rec_mc(amount: u32) -> u32 {
    // 1 2 5 10 20 30 50 100
    // dp[amount] = dp[amount - coins[i]] + 1
    let coins = vec![1, 2, 5, 10, 20, 30, 50, 100];
    let mut dp = vec![100000000 as u32; amount as usize + 1];
    dp[0] = 0;

    for i in 1..(amount + 1) {
        for &coin in &coins {
            if coin > i {
                continue;
            }
            dp[i as usize] = dp[i as usize].min(dp[i as usize - coin as usize] + 1)
        }
    }
    dp[amount as usize]
}
