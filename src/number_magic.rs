pub fn is_repdigit(number: u32) -> bool {
    let mut b = true;
    let number_str = number.to_string();
    let first_char = number_str.chars().nth(0).unwrap();
    println!("{:?}", first_char);
    for c in number_str.chars() {
        if first_char != c {
            b = false;
            break;
        }
    }
    return b;
}

pub fn make_repdigit(digit: &str, length: usize) -> u32 {
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
