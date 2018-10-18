extern crate chrono;

mod people;

fn main() {
    let mut data = people::Data::init();

    data.add("Matea", "2017-12-30 17:40:00");
    data.add("Flora", "2016-02-20 20:42:00");
    data.add("Bär", "1981-11-22 12:00:00");
    data.add("Er", "1981-08-07 11:55:00");
    data.people.sort();

    data.conky_output();
}
