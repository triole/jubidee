extern crate chrono;

mod people;
mod output;

fn main() {
    let data = people::data();
    output::conky_out(data);
}
