pub fn next_repdigit(number: u32) -> u32 {
    let mut next_repdigit = base_repdigit(number);
    let incrementor = make_repdigit("1", get_length(number));
    while next_repdigit < number
        || next_repdigit.to_string().len() < 2
        || is_repdigit(next_repdigit) == false
    {
        next_repdigit += incrementor;
    }
    return next_repdigit;
}

// utils
fn base_repdigit(number: u32) -> u32 {
    let first_char = get_first_char(number);
    let length = get_length(number);
    let base_repdigit = make_repdigit(&first_char.to_string(), length);
    return base_repdigit;
}

fn get_first_char(number: u32) -> char {
    let number_str = number.to_string();
    let first_char = number_str.chars().nth(0).unwrap();
    return first_char;
}

fn get_length(number: u32) -> usize {
    return number.to_string().len();
}

fn is_repdigit(number: u32) -> bool {
    let mut b = true;
    let number_str = number.to_string();
    let first_char = get_first_char(number);
    for c in number_str.chars() {
        if first_char != c {
            b = false;
            break;
        }
    }
    return b;
}

fn make_repdigit(digit: &str, length: usize) -> u32 {
    let mut s = String::new();
    for _n in 0..length {
        s.push_str(&digit.to_string());
    }
    let u: u32 = s.parse::<u32>().unwrap();
    return u;
}

#[test]
fn test_make_repdigit() {
    assert_eq!(make_repdigit("1", 3), 111);
    assert_eq!(make_repdigit("4", 4), 4444);
    assert_eq!(make_repdigit("7", 4), 7777);
    assert_eq!(make_repdigit("9", 5), 99999);
}

#[test]
fn test_is_repdigit() {
    assert_eq!(is_repdigit(111), true);
    assert_eq!(is_repdigit(121), false);
    assert_eq!(is_repdigit(39), false);
    assert_eq!(is_repdigit(4447444), false);
}

#[test]
fn test_base_repdigit() {
    assert_eq!(base_repdigit(101), 111);
    assert_eq!(base_repdigit(121), 111);
    assert_eq!(base_repdigit(994), 999);
    assert_eq!(base_repdigit(999), 999);
    assert_eq!(base_repdigit(111111), 111111);
    assert_eq!(base_repdigit(427281), 444444);
}

#[test]
fn test_next_repdigit() {
    assert_eq!(next_repdigit(1), 11);
    assert_eq!(next_repdigit(101), 111);
    assert_eq!(next_repdigit(224), 333);
    assert_eq!(next_repdigit(994), 999);
    assert_eq!(next_repdigit(999), 999);
    assert_eq!(next_repdigit(111111), 111111);
    assert_eq!(next_repdigit(427281), 444444);
}
