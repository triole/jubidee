#[macro_use]
extern crate clap;
extern crate chrono;
extern crate serde_yaml;

mod argparse;
mod config;
mod env;
mod number_magic;
mod people;
mod util;

fn main() {
    // argparse
    let yaml = load_yaml!("../.argsprod.yaml").to_owned();
    let args = argparse::Argparse::init(yaml);

    // read people's birthday config
    let config = config::read(&env::config_file());

    // calculate
    let mut data = people::Data::init(&"now".to_string());
    for (name, dob) in config {
        data.add(&name, &dob.to_string())
    }

    // print
    let limit = args.val_usize("limit");
    if args.bool("jubidee") == true {
        if args.bool("reverse") == true {
            data.people
                .sort_by(|a, b| a.next_jubidee.cmp(&b.next_jubidee).reverse());
        } else {
            data.people
                .sort_by(|a, b| a.next_jubidee.cmp(&b.next_jubidee));
        }
        data.print_jubidees(&limit);
    } else {
        if args.bool("reverse") == true {
            data.people
                .sort_by(|a, b| a.next_birthday.cmp(&b.next_birthday).reverse());
        } else {
            data.people
                .sort_by(|a, b| a.next_birthday.cmp(&b.next_birthday));
        }
        data.print_birthdays(&limit);
    }
}
