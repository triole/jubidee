#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Person {
    name: String,
    dob: String,
}

impl Person {
    fn new(name: &str, dob: &str) -> Person {
        Person {
            name: name.to_string(),
            dob: dob.to_string()
        }
    }
}

pub fn people() -> Vec<Person>{
    let mut vec = Vec::new();
    vec.push(Person::new("Matea", "2017-12-30 17:40:00"));
    vec.push(Person::new("Flora", "2016-02-20 20:42:00"));
    vec.push(Person::new("Bär", "1981-11-22 12:00:00"));
    vec.push(Person::new("Er", "1981-08-07 11:55:00"));
return vec;
}
