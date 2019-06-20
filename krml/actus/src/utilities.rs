use super::*;

// 4.1 Schedule
pub fn schedule(
    s: Time,
    t: Time,
    cycle: Option<Cycle>,
    end_of_month_convention: Option<EndOfMonthConvention>,
    business_day_convention: Option<BusinessDayConvention>,
) -> Vec<UncheckedTime> {
    let mut vec: Vec<UncheckedTime> = Vec::new();

    if s >= t || (s == Time(None) && t == Time(None)) {
        return vec;
    }
    let mut s = s.0.unwrap();
    if t == Time(None) {
        vec.push(s);
        return vec;
    }
    let mut t = t.0.unwrap();
    if cycle == None {
        vec.push(s);
        vec.push(t);
        return vec;
    }

    let cycle = cycle.unwrap();
    let end_of_month_convention = end_of_month_convention.unwrap_or(EndOfMonthConvention::SD);
    // let business_day_convention = business_day_convention.unwrap_or(BusinessDayConvention::NULL);

    match cycle {
        Cycle::Days(int, stub) => vec.push(t),
        Cycle::Months(int, stub) => {
            vec.push(s);
            let mut x: UncheckedTime;
            while s < t {
                s.year += (s.month as u16 + int) / 12;
                s.month = (s.month - 1 + int as i8) % 12 + 1;
                x = end_of_month_shift(s, end_of_month_convention);
                vec.push(x);
            }
            if x > t && stub {
                vec.pop();
            }
        }
        Cycle::Years(int, stub) => {
            vec.push(s);
            let mut x: UncheckedTime;
            while s < t {
                s.year += int;
                x = end_of_month_shift(s, end_of_month_convention);
                vec.push(x);
            }
            if x > t && stub {
                vec.pop();
            }
        }
    }
    vec
}

// 4.3 End of Month Shift Convention
pub fn end_of_month_shift(
    mut date: UncheckedTime,
    end_of_month_convention: EndOfMonthConvention,
) -> UncheckedTime {
    let days_in_month = Time::days_in_month(date.year, date.month);
    match end_of_month_convention {
        EndOfMonthConvention::EOM => date.day = days_in_month,
        EndOfMonthConvention::SD => {
            if date.day > days_in_month {
                date.day = days_in_month
            }
        }
    }
    date
}

// 4.4 Business Day Shift Convention
// pub fn business_day_shift(
//     mut date: UncheckedTime,
//     business_day_convention: BusinessDayConvention,
// ) -> UncheckedTime {
//     match business_day_convention {
//         BusinessDayConvention::NULL => {}
//         BusinessDayConvention::SCF => {}
//         BusinessDayConvention::SCMF => {}
//         BusinessDayConvention::CSF => {}
//         BusinessDayConvention::CSMF => {}
//         BusinessDayConvention::SCP => {}
//         BusinessDayConvention::SCMP => {}
//         BusinessDayConvention::CSP => {}
//         BusinessDayConvention::CSMP => {}
//     }
//     date
// }

// 4.5 Business Day Calendar
// pub fn business_day(date: UncheckedTime, calendar: Calendar) -> bool {
//     match calendar {
//         Calendar::NC => true,
//         Calendar::MTF => {
//             let weekday = Time::day_of_week(date.year, date.month, date.day);
//             if weekday == 6 || weekday == 7 {
//                 false
//             } else {
//                 true
//             }
//         }
//     }
// }

// 4.6 Year Fraction Convention (https://en.wikipedia.org/wiki/Day_count_convention).
pub fn year_fraction(s: Time, t: Time, day_cont_convention: DayCountConvention) -> Real {
    if s == Time(None) || t == Time(None) || s < t {
        return Real(None);
    }
    match day_cont_convention {
        DayCountConvention::_AAISDA => {
            let mut year_1 = s.0.unwrap().year;
            let mut month_1 = s.0.unwrap().month;
            let day_1 = s.0.unwrap().day;
            let year_2 = t.0.unwrap().year;
            let mut month_2 = t.0.unwrap().month;
            let day_2 = t.0.unwrap().day;

            let mut diff_leap: i64 = 0;
            let mut diff_normal: i64 = 0;

            let flag_1 = Time::is_leap_year(year_1);
            let flag_2 = Time::is_leap_year(year_2);

            if flag_1 {
                diff_leap -= day_1 as i64;
            } else {
                diff_normal -= day_1 as i64;
            }
            if flag_2 {
                diff_leap += day_2 as i64;
            } else {
                diff_normal += day_2 as i64;
            }

            let mut x: i64 = 0;
            while month_1 != 0 {
                x += Time::days_in_month(year_1, month_1) as i64;
                month_1 -= 1;
            }
            if flag_1 {
                diff_leap -= x;
            } else {
                diff_normal -= x;
            }
            x = 0;
            while month_2 != 0 {
                x += Time::days_in_month(year_2, month_2) as i64;
                month_2 -= 1;
            }
            if flag_2 {
                diff_leap += x;
            } else {
                diff_normal += x;
            }

            while year_1 < year_2 {
                if Time::is_leap_year(year_1) {
                    diff_leap += 366;
                } else {
                    diff_normal += 365;
                }
                year_1 += 1;
            }

            Real::from(diff_normal) / Real::from(365) + Real::from(diff_leap) / Real::from(366)
        }
        DayCountConvention::_A360 => {
            let mut year_1 = s.0.unwrap().year;
            let mut month_1 = s.0.unwrap().month;
            let day_1 = s.0.unwrap().day;
            let year_2 = t.0.unwrap().year;
            let mut month_2 = t.0.unwrap().month;
            let day_2 = t.0.unwrap().day;

            let mut diff: i64 = 0;

            diff += (day_2 - day_1) as i64;

            while month_1 != 0 {
                diff -= Time::days_in_month(year_1, month_1) as i64;
                month_1 -= 1;
            }
            while month_2 != 0 {
                diff += Time::days_in_month(year_2, month_2) as i64;
                month_2 -= 1;
            }

            while year_1 < year_2 {
                diff += 365 + (Time::is_leap_year(year_1) as i64);
                year_1 += 1;
            }

            Real::from(diff) / Real::from(360)
        }
        DayCountConvention::_A365 => {
            let mut year_1 = s.0.unwrap().year;
            let mut month_1 = s.0.unwrap().month;
            let day_1 = s.0.unwrap().day;
            let year_2 = t.0.unwrap().year;
            let mut month_2 = t.0.unwrap().month;
            let day_2 = t.0.unwrap().day;

            let mut diff: i64 = 0;

            diff += (day_2 - day_1) as i64;

            while month_1 != 0 {
                diff -= Time::days_in_month(year_1, month_1) as i64;
                month_1 -= 1;
            }
            while month_2 != 0 {
                diff += Time::days_in_month(year_2, month_2) as i64;
                month_2 -= 1;
            }

            while year_1 < year_2 {
                diff += 365 + (Time::is_leap_year(year_1) as i64);
                year_1 += 1;
            }

            Real::from(diff) / Real::from(365)
        }
        DayCountConvention::_30E360 => {
            let year_1 = Real::from(s.0.unwrap().year as i64);
            let month_1 = Real::from(s.0.unwrap().month as i64);
            let mut day_1 = Real::from(s.0.unwrap().day as i64);
            let year_2 = Real::from(t.0.unwrap().year as i64);
            let month_2 = Real::from(t.0.unwrap().month as i64);
            let mut day_2 = Real::from(t.0.unwrap().day as i64);

            if day_1 == Real::from(31) {
                day_1 = Real::from(30);
            }
            if day_2 == Real::from(31) {
                day_2 = Real::from(30);
            }

            (Real::from(360) * (year_2 - year_1)
                + Real::from(30) * (month_2 - month_1)
                + (day_2 - day_1))
                / Real::from(360)
        }
        DayCountConvention::_30360 => {
            let year_1 = Real::from(s.0.unwrap().year as i64);
            let month_1 = Real::from(s.0.unwrap().month as i64);
            let day_1 = Real::from(s.0.unwrap().day as i64);
            let year_2 = Real::from(t.0.unwrap().year as i64);
            let month_2 = Real::from(t.0.unwrap().month as i64);
            let day_2 = Real::from(t.0.unwrap().day as i64);

            (Real::from(360) * (year_2 - year_1)
                + Real::from(30) * (month_2 - month_1)
                + (day_2 - day_1))
                / Real::from(360)
        }
        // DayCountConvention::_30E360ISDA => Real::from(0),
        // DayCountConvention::_BUS252 => Real::from(0),
    }
}

/// 4.7 Contract Role Sign Convention
pub fn contract_role_sign(contract_role: ContractRole) -> Real {
    match contract_role {
        ContractRole::RPA => Real::from(1),
        ContractRole::RPL => Real::from(-1),
        ContractRole::CLO => Real::from(1),
        ContractRole::CNO => Real::from(1),
        ContractRole::COL => Real::from(1),
        ContractRole::LG => Real::from(1),
        ContractRole::ST => Real::from(-1),
        ContractRole::BUY => Real::from(1),
        ContractRole::SEL => Real::from(-1),
        ContractRole::RFL => Real::from(1),
        ContractRole::PFL => Real::from(-1),
        ContractRole::RF => Real::from(1),
        ContractRole::PF => Real::from(-1),
    }
}

// 4.8 Contract Default Convention
pub fn contract_default(contract_status: ContractStatus) -> Real {
    match contract_status {
        ContractStatus::PF => Real::from(1),
        ContractStatus::DL => Real::from(1),
        ContractStatus::DQ => Real::from(1),
        ContractStatus::DF => Real::from(0),
    }
}
