#[derive(Debug)]
struct date {
    year: u32,
    month: u32,
    day: u32,
}

impl date {
    fn new(s: &str) -> Self {
        let list = s
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if list.len() != 3 {
            panic!("Invalid input format")
        }
        date {
            year: list[0],
            month: list[1],
            day: list[2],
        }
    }

    fn format(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    fn before(&self, d: &date) -> bool {
        self.year < d.year
            || (self.year == d.year && self.month < d.month)
            || (self.year == d.year && self.month == d.month && self.day <= d.day)
    }

    fn after(&self, days: u32) -> date {
        let mut d = date {
            year: self.year,
            month: self.month,
            day: self.day,
        };
        d.day += days;
        let mut month = self.month;
        while d.day > d.days_in_month(month) {
            d.day -= d.days_in_month(month);
            month += 1;
            if month > 12 {
                d.year += 1;
                month = 1;
            }
        }
        d.month = month;
        d
    }

    fn dur_days(&self, d: &date) -> u32 {
        let mut dur = 0;
        for y in self.year + 1..d.year {
            dur += self.days_in_year(y);
        }
        if self.year == d.year
            && (self.month < d.month || (self.month == d.month && self.day <= d.day))
        {
            dur + d.passed_days() - self.passed_days()
        } else {
            dur + self.remain_days() + d.passed_days()
        }
    }

    fn is_leap_year(&self, year: u32) -> bool {
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            true
        } else {
            false
        }
    }

    fn days_in_month(&self, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(self.year) {
                    29
                } else {
                    28
                }
            }
            _ => {
                panic!("Invalid month")
            }
        }
    }

    fn remain_days(&self) -> u32 {
        self.days_in_year(self.year) - self.passed_days()
    }

    fn passed_days(&self) -> u32 {
        let mut days = 0;
        for m in 1..self.month {
            days += self.days_in_month(m);
        }
        days + self.day
    }

    fn days_in_year(&self, year: u32) -> u32 {
        if self.is_leap_year(year) {
            366
        } else {
            365
        }
    }

    // 2025-01-01
    // y = 25, c = 20
    // 25 + 25 / 4 + 20 / 4 - 2 * 20 + 26 * 14 / 10 + 1 - 1

    fn week_day(&self) -> u32 {
        let mut year = self.year;
        let mut m = self.month;
        if self.month <= 2 {
            year -= 1;
            m += 12;
        }
        let y = year as i32 % 100;
        let c = year as i32 / 100;
        let week = (y + y / 4 + c / 4 - 2 * c
            + (26 * {
                match m as i32 {
                    1 | 2 => m as i32 + 12 + 1,
                    _ => m as i32 + 1,
                }
            }) / 10
            + self.day as i32
            - 1)
            % 7;
        match week {
            0 => 7,
            _ => week as u32,
        }
    }

    fn passed_week(&self) -> u32 {
        let w = self.passed_days() / 7 + 1;
        if w > 52 {
            w - 52
        } else {
            w
        }
    }

    fn get_spring_festival(&self) -> u32 {
        let chinese_new_year = vec!["2025-01-29", "2026-02-17"];
        // 判断是当年春节，还是下一年春节
        let idx = self.year as usize - 2025;
        let d = date::new(chinese_new_year[idx]);
        if self.before(&d) {
            self.dur_days(&d)
        } else {
            self.dur_days(&date::new(chinese_new_year[idx + 1]))
        }
    }

    fn next_a(&self) -> u32 {
        let holidy = vec![
            "2025-01-01",
            "2025-01-28",
            "2025-01-29",
            "2025-01-30",
            "2025-01-31",
            "2025-02-03",
            "2025-02-04",
            "2025-04-04",
            "2025-05-01",
            "2025-05-02",
            "2025-05-05",
            "2025-06-02",
            "2025-10-01",
            "2025-10-02",
            "2025-10-03",
            "2025-10-06",
            "2025-10-07",
            "2025-10-08",
            "2026-01-01",
        ];
        let mut cnt = 0;
        loop {
            let now = self.after(cnt + 1);
            if holidy.contains(&now.format().as_str()) {
                cnt += 1;
                continue;
            }
            if now.week_day() > 5 {
                cnt += 1;
                continue;
            }
            return cnt;
        }
    }
}

// 第几周，周几，当年的第几天，当前还剩下多少天，距离过年还有多少天，距离下一次A股开盘多少天
pub fn time_info(time: &str) -> String {
    let d = date::new(time);
    let res = vec![
        d.passed_week().to_string(),
        d.week_day().to_string(),
        d.passed_days().to_string(),
        d.remain_days().to_string(),
        d.get_spring_festival().to_string(),
        d.next_a().to_string(),
    ];
    res.join(",")
}
