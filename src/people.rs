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
        let day_of_birth = util::parse_date_string(dob);
        let age_duration = self.date_delta(&self.now, &day_of_birth);
        let birthday_over_this_year = self.birthday_over_this_year(&day_of_birth);
        let next_birthday = self.next_birthday(&day_of_birth, birthday_over_this_year);
        let next_birthday_duration = self.date_delta(&next_birthday, &self.now);
        let p = Person {
            name: name.to_string(),
            birthday_over_this_year: birthday_over_this_year,
            age: Duration {
                date: day_of_birth,
                in_duration: age_duration,
                in_days: age_duration.num_days() as u32,
                in_years: self.age_in_years(&day_of_birth, birthday_over_this_year) as u32,
                jubidee: 0,
            },
            next_birthday: Duration {
                date: next_birthday,
                in_duration: next_birthday_duration,
                in_days: (next_birthday_duration.num_days() as u32),
                in_years: self.years_to_next_birthday(birthday_over_this_year),
                jubidee: 0,
            },
            next_jubidee: self.calculate_next_jubidee(day_of_birth, age_duration),
        };
        self.people.push(p);
    }

    fn calculate_next_jubidee(
        &self,
        day_of_birth: chrono::DateTime<chrono::Local>,
        age_duration: chrono::Duration,
    ) -> Duration {
        let next_jubidee_number = number_magic::next_jubidee_number(age_duration.num_days() as u32);
        let next_jubidee_in_days = next_jubidee_number - age_duration.num_days() as u32;
        let next_jubidee_date = self.now + chrono::Duration::days(next_jubidee_in_days as i64);
        let d = Duration {
            date: next_jubidee_date,
            in_duration: self.date_delta(&self.now, &day_of_birth),
            in_days: next_jubidee_in_days,
            in_years: 0,
            jubidee: next_jubidee_number,
        };
        return d;
    }

    pub fn conky_output(&self) {
        for p in &self.people {
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

    fn age_in_years(
        &self,
        dob: &chrono::DateTime<chrono::Local>,
        birthday_over_this_year: i8,
    ) -> i32 {
        let mut age_in_years: i32 = self.now.year() - dob.year();
        if birthday_over_this_year == -1 {
            age_in_years -= 1;
        }
        if age_in_years < 0 {
            age_in_years = 0
        };
        return age_in_years;
    }

    // returns: -1 if not over, 0 if on day, 1 if over
    // three values necessary, for age calculation we need to know if there is a birthday party
    fn birthday_over_this_year(&self, dob: &chrono::DateTime<chrono::Local>) -> i8 {
        let mut b = -1;
        let current_year = self.now.year();
        let next_birthday = dob.with_year(current_year).unwrap();
        if next_birthday == self.now {
            b = 0;
        } else if next_birthday < self.now {
            b = 1;
        }
        return b;
    }

    fn date_delta(
        &self,
        d1: &chrono::DateTime<chrono::Local>,
        d2: &chrono::DateTime<chrono::Local>,
    ) -> chrono::Duration {
        return d1.signed_duration_since(d2.to_owned());
    }

    fn next_birthday(
        &self,
        dob: &chrono::DateTime<chrono::Local>,
        birthday_over_this_year: i8,
    ) -> chrono::DateTime<chrono::Local> {
        let next_birthday: chrono::DateTime<chrono::Local>;
        if birthday_over_this_year <= 0 {
            next_birthday = dob.with_year(self.now.year()).unwrap();
        } else {
            next_birthday = dob.with_year(self.now.year() + 1).unwrap();
        }
        return next_birthday;
    }

    fn years_to_next_birthday(&self, birthday_over_this_year: i8) -> u32 {
        let mut r = 1;
        if birthday_over_this_year < 1 {
            r = 0;
        }
        return r;
    }

    // sorting methods, note these of course require the ord trait, see below
    pub fn sort_by_next_birthday(mut self) {
        &self.people
            .sort_by(|a, b| a.next_birthday.cmp(&b.next_birthday));
    }

    pub fn sort_by_next_jubidee(mut self) {
        &self.people
            .sort_by(|a, b| a.next_jubidee.cmp(&b.next_jubidee));
    }
}

#[derive(Debug, Eq)]
pub struct Person {
    name: String,
    birthday_over_this_year: i8,
    age: Duration,
    pub next_birthday: Duration,
    pub next_jubidee: Duration,
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
    jubidee: u32,
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

// --- testing
#[test]
fn test_age_and_birthday_calculation() {
    assert_person_data(
        "20180101", "P1", "20180101", 0, 0, "20180101", 0, 0, "20180101", 0,
    );
    assert_person_data(
        "20110807", "P2", "19810807", 30, 10957, "20110807", 0, 0, "20110919", 43,
    );
    assert_person_data(
        "20200101", "P3", "20180101", 2, 730, "20200101", 0, 0, "20200121", 20,
    );
    assert_person_data(
        "20200101", "P4", "20180130", 1, 701, "20200130", 0, 29, "20200219", 49,
    );
    assert_person_data(
        "20200201", "P5", "20180130", 2, 732, "20210130", 1, 364, "20200219", 18,
    );
    // assert_person_data(
    //     "20181021", "P6", "19261021", 92, 33602, "20181021", 0, 0, "20191123", 398,
    // );
    assert_person_data(
        "20181022", "P7", "19561023", 61, 22643, "20181023", 0, 1, "20191014", 357,
    );
}

fn assert_person_data(
    base_date: &str,
    name: &str,
    birth_date: &str,
    assert_age_in_years: u32,
    assert_age_in_days: u32,
    assert_next_birthday_date: &str,
    assert_next_birthday_in_years: u32,
    assert_next_birthday_in_days: u32,
    assert_next_jubidee_date: &str,
    assert_next_jubidee_in_days: u32,
) {
    let mut data = Data::init(base_date);
    data.add(name, birth_date);
    let p = &data.people[0];
    assert!(
        p.age.in_years == assert_age_in_years,
        "Assertion age in years failed: {} != {}, \n {:#?}",
        p.age.in_years,
        assert_age_in_years,
        data
    );
    assert!(
        p.age.in_days == assert_age_in_days,
        "age in days assertion failed: {} != {}, \n {:#?}",
        p.age.in_days,
        assert_age_in_days,
        data
    );
    let mut t = assert_next_birthday_date.to_owned() + " 00:00:00";
    let mut nb = Local.datetime_from_str(&t, "%Y%m%d %H:%M:%S").unwrap();
    assert!(
        p.next_birthday.date == nb,
        "next birthday date failed: {} != {}, \n {:#?}",
        p.next_birthday.date,
        nb,
        data
    );
    assert!(
        p.next_birthday.in_years == assert_next_birthday_in_years,
        "next birthday in years assertion failed: {} != {}, \n {:#?}",
        p.next_birthday.in_years,
        assert_next_birthday_in_years,
        data
    );
    assert!(
        p.next_birthday.in_days == assert_next_birthday_in_days,
        "next birthday in days assertion failed: {} != {}, \n {:#?}",
        p.next_birthday.in_days,
        assert_next_birthday_in_days,
        data
    );
    t = assert_next_jubidee_date.to_owned() + " 00:00:00";
    nb = Local.datetime_from_str(&t, "%Y%m%d %H:%M:%S").unwrap();
    assert!(
        p.next_jubidee.date == nb,
        "next jubidee date assertion failed: {} != {}, \n {:#?}",
        p.next_jubidee.date,
        nb,
        data
    );
    assert!(
        p.next_jubidee.in_days == assert_next_jubidee_in_days,
        "next jubidee in days assertion failed: {} != {}, \n {:#?}",
        p.next_jubidee.in_days,
        assert_next_jubidee_in_days,
        data
    );
}
