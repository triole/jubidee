extern crate chrono;

mod config;
mod date;


fn main() {
    println!("{:#?}", date::now());
    println!("{:#?}", date::date_delta());
    println!("{:#?}", date::date_delta().num_days());

    println!("{:?}", config::people());
}
