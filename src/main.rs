extern crate chrono;

mod people;

fn main() {
    let data = people::data();
    println!("{:#?}", data);
}
