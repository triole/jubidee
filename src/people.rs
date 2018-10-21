use chrono;
use chrono::offset::{Local, TimeZone};
use chrono::prelude::*;
use std::cmp::Ordering;

use number_magic;
use util;

#[derive(Debug)]
pub struct Data {
    now: chrono::DateTime<chrono::Local>,
    pub people: Vec<Person>,
}

impl Data {
    pub fn init(s: &str) -> Self {
        let d = Data {
            now: util::parse_date_string(s),
            people: Vec::new(),
        };
        return d;
    }

    pub fn add(&mut self, name: &str, dob: &str) {
        let day_of_birth = self.parse_date_string(dob);
        let birthday_over_this_year = self.birthday_over_this_year(&day_of_birth);
        let age_duration = self.date_delta(&self.now, &day_of_birth);
        let next_birthday = self.next_birthday(&day_of_birth, birthday_over_this_year);
        let next_birthday_duration = self.date_delta(&next_birthday, &self.now);
        let p = Person {
            name: name.to_string(),
            age: Duration {
                date: day_of_birth,
                in_duration: age_duration,
                in_days: age_duration.num_days() as u32,
                in_years: self.age_in_years(&day_of_birth, birthday_over_this_year) as u32,
            },
            next_birthday: Duration {
                date: next_birthday,
                in_duration: next_birthday_duration,
                in_days: (next_birthday_duration.num_days() as u32) + 1,
                in_years: birthday_over_this_year as u32,
            },
        };
        self.people.push(p);
    }

    pub fn conky_output(&self) {
        for p in &self.people {
            // println!("{number:>width$}", number=1, width=6);
            println!(
                "\
                 {name:<width_name$}\
                 {age_in_years:>width_age_in_years$}\
                 {age_in_days:>width_age_in_days$}\
                 {nbdow}\
                 {nbdur:>width_nbdur$}\
                 ",
                name = p.name,
                width_name = 25,
                age_in_years = p.age.in_years,
                width_age_in_years = 4,
                age_in_days = p.age.in_days,
                width_age_in_days = 6,
                nbdow = p.next_birthday.date.format("  %a %d.%m.%y"),
                nbdur = p.next_birthday.in_days,
                width_nbdur = 5,
            );
        }
    }

    fn parse_date_string(&self, s: &str) -> chrono::DateTime<chrono::Local> {
        let t = s.to_owned() + " 00:00:00";
        return Local.datetime_from_str(&t, "%Y%m%d %H:%M:%S").unwrap();
    }

    fn date_delta(
        &self,
        d1: &chrono::DateTime<chrono::Local>,
        d2: &chrono::DateTime<chrono::Local>,
    ) -> chrono::Duration {
        return d1.signed_duration_since(d2.to_owned());
    }

    fn birthday_over_this_year(&self, dob: &chrono::DateTime<chrono::Local>) -> bool {
        let mut b = false;
        let current_year = self.now.year();
        let next_birthday = dob.with_year(current_year).unwrap();
        if next_birthday < self.now {
            b = true;
        }
        return b;
    }

    fn next_birthday(
        &self,
        dob: &chrono::DateTime<chrono::Local>,
        birthday_over_this_year: bool,
    ) -> chrono::DateTime<chrono::Local> {
        let next_birthday: chrono::DateTime<chrono::Local>;
        if birthday_over_this_year == false {
            next_birthday = dob.with_year(self.now.year()).unwrap();
        } else {
            next_birthday = dob.with_year(self.now.year() + 1).unwrap();
        }
        return next_birthday;
    }

    fn age_in_years(
        &self,
        dob: &chrono::DateTime<chrono::Local>,
        birthday_over_this_year: bool,
    ) -> i32 {
        let mut age_in_years: i32 = self.now.year() - dob.year();
        if birthday_over_this_year == true {
            age_in_years += 1;
        }
        return age_in_years;
    }
}

#[derive(Debug, Eq)]
pub struct Person {
    name: String,
    age: Duration,
    next_birthday: Duration,
}

// --- make the struct sortable by next birthday
impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.next_birthday.cmp(&other.next_birthday)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.next_birthday == other.next_birthday
    }
}

// same for age
#[derive(Debug, Eq)]
pub struct Duration {
    date: chrono::DateTime<chrono::Local>,
    in_duration: chrono::Duration,
    in_years: u32,
    in_days: u32,
}

impl Ord for Duration {
    fn cmp(&self, other: &Duration) -> Ordering {
        self.in_days.cmp(&other.in_days)
    }
}
impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Duration) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        self.in_days == other.in_days
    }
}

#[test]
fn test_name() {
    // TODO: make more and better tests
    let mut data = Data::init("20180101");
    data.add("Foo", "20180101");
    assert_eq!(data.people[0].age.in_years, 0);
    assert_eq!(data.people[0].age.in_days, 0);
}
