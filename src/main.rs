extern crate serde_derive;
extern crate serde_yaml;

extern crate chrono;

mod env;
mod config;
mod people;

fn main() {
    let config = config::read(&env::config_file());

    let mut data = people::Data::init();
    for (name, dob) in config{
        data.add(&name, &dob.to_string())
    }
    data.people.sort();

    data.conky_output();
}
