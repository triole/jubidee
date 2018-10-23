#[macro_use]
extern crate clap;
extern crate chrono;
extern crate serde_yaml;

use clap::App;

mod argparse;
mod config;
mod env;
mod number_magic;
mod people;
mod util;

fn main() {
    let yaml = load_yaml!("args.yaml");
    let args = App::from_yaml(yaml).get_matches();

    let config = config::read(&env::config_file());

    let mut data = people::Data::init(&"now".to_string());
    for (name, dob) in config {
        data.add(&name, &dob.to_string())
    }

    if argparse::bool(&args, "jubidee") == true {
        data.people
            .sort_by(|a, b| a.next_jubidee.cmp(&b.next_jubidee));
        data.print_jubidees();
    } else {
        data.people
            .sort_by(|a, b| a.next_birthday.cmp(&b.next_birthday));
        data.print_birthdays();
    }
}
