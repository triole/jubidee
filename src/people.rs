use chrono;
use chrono::offset::{Local, TimeZone};
use chrono::prelude::*;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Data {
    now: chrono::DateTime<chrono::Local>,
    pub people: Vec<Person>,
}

impl Data {
    pub fn init() -> Self {
        let d = Data {
            now: Local::now(),
            people: Vec::new(),
        };
        return d;
    }

    pub fn add(&mut self, name: &str, dob: &str) {
        let dob_date = self.parse_date_string(dob);
        let birthday_done_this_year = self.birthday_done_this_year(&dob_date);
        let next_birthday = self.next_birthday(&dob_date, birthday_done_this_year);
        let p = Person {
            name: name.to_string(),
            dob: dob_date,
            age_duration: self.date_delta(&self.now, &dob_date),
            age_in_years: self.age_in_years(&dob_date, birthday_done_this_year),
            next_birthday: next_birthday,
            nb_done_this_year: birthday_done_this_year,
            duration_to_nb: self.date_delta(&next_birthday, &self.now),
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
                 {nbdur:>width_nbdur$}d\
                 ",
                name = p.name,
                width_name = 25,
                age_in_years = p.age_in_years,
                width_age_in_years = 4,
                age_in_days = p.age_duration.num_days(),
                width_age_in_days = 6,
                nbdow = p.next_birthday.format("  %a %d.%m.%y"),
                nbdur = p.duration_to_nb.num_days(),
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

    fn birthday_done_this_year(&self, dob: &chrono::DateTime<chrono::Local>) -> bool {
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
        birthday_done_this_year: bool,
    ) -> chrono::DateTime<chrono::Local> {
        let next_birthday: chrono::DateTime<chrono::Local>;
        if birthday_done_this_year == false {
            next_birthday = dob.with_year(self.now.year()).unwrap();
        } else {
            next_birthday = dob.with_year(self.now.year() + 1).unwrap();
        }
        return next_birthday;
    }

    fn age_in_years(
        &self,
        dob: &chrono::DateTime<chrono::Local>,
        birthday_done_this_year: bool,
    ) -> i32 {
        let mut age_in_years: i32 = self.now.year() - dob.year();
        if birthday_done_this_year == true {
            age_in_years += 1;
        }
        return age_in_years;
    }
}

#[derive(Debug, Eq)]
pub struct Person {
    name: String,
    dob: chrono::DateTime<chrono::Local>,
    age_duration: chrono::Duration,
    age_in_years: i32,
    next_birthday: chrono::DateTime<chrono::Local>,
    nb_done_this_year: bool,
    duration_to_nb: chrono::Duration,
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
