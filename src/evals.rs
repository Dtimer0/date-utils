use crate::types::*;
use chrono::{DateTime as chronoDateTime, Timelike};
use chrono::prelude::Local;
use chrono::Datelike;


#[macro_export]
macro_rules! allShare { // macro checks if every element has the same of one or more fields and returns them
    ($($date:expr), *) => {
        todo!();
    }
}

#[macro_export]
macro_rules! implevalfns {
    ($struct:ident) => {
        impl crate::types::datekind for $struct {
            fn isLeapYear(&self) -> bool {
                if (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0{
                    return true;
                }
                false
            }
            fn weekday(&self) -> Result<String, std::io::Error> {
                let weekdays: Vec<&str> = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];                                                                 
                return Ok(weekdays[self.weekday_as_int().expect("Error converting date to week number") as usize].to_string());
            }
            fn weekday_as_int(&self) -> Result<i8, std::io::Error> {
                let last_two_digits_year: i32 = self.year.to_string().as_str().chars().take(2).map(|x| x.to_string()).collect::<String>().parse().unwrap();
                let first_two_digits_year: i32 = self.year as i32 % 100;      
            return Ok(((self.day + 
                ((13*(if self.month == 1 || self.month == 2{
                self.month + 10
                } else {
                self.month - 2
                }
                ) - 1 ) / 5 ) + first_two_digits_year as i8 + (first_two_digits_year as i8 / 4) + (last_two_digits_year as i8 / 4) - 2*last_two_digits_year as i8) % 7) as i8);
            }
            fn sharesDay(&self, date2: &$struct) -> bool {
                if self.day == date2.day {
                    return true;
                }
                false
            }
            fn sharesYear(&self, date2: &$struct) -> bool {
                if self.year == date2.year {
                    return true;
                }
                false
            }
            fn sharesMonth(&self, date2: &$struct) -> bool {
                if self.month == date2.month {
                    return true;
                }
                false
            }


        }
    }
}
impl crate::types::Date {
    fn snapshot_date() -> crate::types::Date {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::Date {
            day: local.day() as i8,
            month: local.month() as i8,
            year: local.year() as i16,
            
        }
    }
    fn DateShares(&self, compare_type: &str, datetime2: &Self) -> Result<bool, &str> {
        match compare_type {
            "second" => if self.day == datetime2.day{ Ok(true) } else { Ok(false) },
            "minute" => if self.month == datetime2.month { Ok(true) } else { Ok(false) },
            "hour" => if self.year == datetime2.year { Ok(true) } else { Ok(false) },
            &_ => return Err("Invalid compare type")
        }
    }
}
impl crate::types::DateTime {
    fn snapshot_datetime() -> crate::types::DateTime {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::DateTime {
            second: local.second() as i8,
            minute: local.minute() as i8,
            hour: local.hour() as i8,
            day: local.day() as i8,
            month: local.month() as i8,
            year: local.year() as i16,
            
        }

    }
    fn sharesSecond(&self, datetime2: DateTime) -> bool{
        if self.second == datetime2.second {
            return true;
        }
        false
    }
    fn sharesMinute(&self, datetime2: DateTime) -> bool{
        if self.minute == datetime2.minute {
            return true;
        }
        false
    }
    fn sharesHour(&self, datetime2: DateTime) -> bool{
        if self.hour == datetime2.hour {
            return true;
        }
        false
    }
    fn DateTimeShares(&self, compare_type: &str, datetime2: &Self) -> Result<bool, &str> {
        match compare_type {
            "second" => if self.second == datetime2.second { Ok(true) } else { Ok(false) },
            "minute" => if self.minute == datetime2.minute { Ok(true) } else { Ok(false) },
            "hour" => if self.hour == datetime2.hour { Ok(true) } else { Ok(false) },
            "day" => if self.day == datetime2.day { Ok(true) } else { Ok(false) },
            "month" => if self.month == datetime2.month { Ok(true) } else { Ok(false) },
            "year" => if self.year == datetime2.year { Ok(true) } else { Ok(false) },
            &_ => {return Err("Invalid compare type");}
        }
    }
}
implevalfns!(Date);
implevalfns!(DateTime);