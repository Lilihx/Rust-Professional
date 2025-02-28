// 男员工 60 -> 63   从2025-1开始，4个月延迟1个月
// 女员工 55 ->  58 同上
// 女员工 50 -> 55  2个月延迟一个月

struct Date {
    year: i32,
    month: i32,
}

impl Date {
    pub fn new(s: &str) -> Self {
        let mut d = Date {
            year: 2025,
            month: 1,
        };
        let mut sp = s.split("-");
        if let Some(year) = sp.next() {
            d.year = year.parse().unwrap();
        } else {
            panic!("input is invalid")
        }
        if let Some(year) = sp.next() {
            d.month = year.trim_start_matches(|x| x == '0').parse().unwrap();
        } else {
            panic!("input is invalid")
        }
        d
    }

    pub fn format(&self) -> String {
        if self.month < 10 {
            format!("{}-0{}", self.year, self.month)
        } else {
            format!("{}-{}", self.year, self.month)
        }
    }

    pub fn add(&mut self, m: i32) {
        self.year += m / 12;
        self.month += m % 12;
        self.year += self.month / 13;
        self.month -= self.month / 13 * 12;
    }

    pub fn dur_month(&self, d: &Date) -> i32 {
        (d.year - self.year) * 12 + d.month - self.month
    }
}

struct Policy {
    effect_time: Date,
    retire_age_before: i32,
    retire_age_after: i32,
    delay_speed: i32,
}

impl Policy {
    pub fn new(s: &str) -> Self {
        let mut p = Policy {
            effect_time: Date::new("2025-01"),
            retire_age_before: 50,
            retire_age_after: 55,
            delay_speed: 2,
        };
        if s.contains("男职工") {
            p.retire_age_before = 60;
            p.retire_age_after = 63;
            p.delay_speed = 4;
        } else if s.contains("55周岁女职工") {
            p.retire_age_before = 55;
            p.retire_age_after = 58;
            p.delay_speed = 4;
        } else {
        }
        p
    }

    pub fn cal_retirement(&self, birth: &mut Date) -> String {
        // 到2025 有几个月
        let dur_2025 = birth.dur_month(&self.effect_time);
        println!("{:?}", dur_2025);
        let month_remain_max = self.retire_age_after * 12 - dur_2025;
        let month_remain = self.retire_age_before * 12 - dur_2025;
        if month_remain < 0 {
            // 直接退休，不用延期
            birth.add(self.retire_age_before * 12);
            return format!("{},{},{}", birth.format(), self.retire_age_before, 0);
        }
        birth.add(dur_2025);
        let mut delay_month = month_remain / self.delay_speed;
        let mut retire = month_remain; // retire 表示过了25年，还差多少月。
        loop {
            if delay_month == 0 {
                break;
            }
            retire = (retire + delay_month).min(month_remain_max);
            if retire >= month_remain_max {
                break;
            }
            delay_month = retire / self.delay_speed - (retire - month_remain);
        }
        retire = retire.max(1);
        birth.add(retire);
        if (retire - month_remain) % 12 == 0 {
            format!(
                "{},{},{}",
                birth.format(),
                self.retire_age_before + (retire - month_remain) / 12,
                retire - month_remain
            )
        } else {
            format!(
                "{},{:.2},{}",
                birth.format(),
                self.retire_age_before as f32 + (retire - month_remain) as f32 / 12.0,
                retire - month_remain
            )
        }
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let policy = Policy::new(tp);
    let mut birth = Date::new(time);
    policy.cal_retirement(&mut birth)
}
