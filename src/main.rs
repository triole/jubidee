extern crate chrono;
extern crate serde_yaml;

mod config;
mod env;
mod people;
mod number_magic;

fn main() {
    let config = config::read(&env::config_file());

    let mut data = people::Data::init();
    for (name, dob) in config {
        data.add(&name, &dob.to_string())
    }
    data.people.sort();

    data.conky_output();
}
