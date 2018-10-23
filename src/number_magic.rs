use people;

// jubidee calculation
pub fn next_jubidee_number(number: u32) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    v.push(next_jubilee(number));
    v.push(next_repdigit(number));
    v.sort();
    return v[0];
}

// jubilee related
fn next_jubilee(number: u32) -> u32 {
    let mut base_jubilee = make_base_jubilee(number);
    let jubistep = determine_jubistep(number);
    while base_jubilee < number {
        base_jubilee += jubistep;
    }
    return base_jubilee;
}

fn determine_jubistep(number: u32) -> u32 {
    let mut jubistep: u32 = 500;
    let l = get_length(number);
    if l == 3 {
        jubistep = 50;
    } else if l < 3 {
        jubistep = 10;
    }
    return jubistep;
}

fn make_base_jubilee(number: u32) -> u32 {
    let jubistep = determine_jubistep(number);
    return replace_last_characters(number, get_length(jubistep));
}

// repdigit related
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

fn make_base_repdigit(number: u32) -> u32 {
    let first_char = get_first_char(number);
    let length = get_length(number);
    let base_repdigit = make_repdigit(&first_char.to_string(), length);
    return base_repdigit;
}

fn make_repdigit(digit: &str, length: usize) -> u32 {
    let mut s = String::new();
    for _n in 0..length {
        s.push_str(&digit.to_string());
    }
    let u: u32 = s.parse::<u32>().unwrap();
    return u;
}

fn next_repdigit(number: u32) -> u32 {
    let mut next_repdigit = make_base_repdigit(number);
    let incrementor = make_repdigit("1", get_length(number));
    while next_repdigit < number
        || next_repdigit.to_string().len() < 2
        || is_repdigit(next_repdigit) == false
    {
        next_repdigit += incrementor;
    }
    return next_repdigit;
}

// general utils
fn get_first_char(number: u32) -> char {
    let number_str = number.to_string();
    let first_char = number_str.chars().nth(0).unwrap();
    return first_char;
}

fn get_length(number: u32) -> usize {
    return number.to_string().len();
}

fn replace_last_characters(number: u32, last_n: usize) -> u32 {
    if number > 10 {
        let mut s = String::new();
        let number_str = number.to_string();
        for (i, n) in number_str.chars().enumerate() {
            if i >= number_str.len() - (last_n) {
                s.push('0');
            } else {
                s.push(n);
            }
        }
        let u: u32 = s.parse::<u32>().unwrap();
        return u;
    } else {
        return 0;
    }
}

#[test]
fn test_next_jubidee() {
    assert_eq!(next_jubidee_number(5), 10);
    assert_eq!(next_jubidee_number(41), 44);
    assert_eq!(next_jubidee_number(99), 99);
    assert_eq!(next_jubidee_number(101), 111);
}

// jubilee tests
#[test]
fn test_base_jubilee() {
    assert_eq!(make_base_jubilee(7), 0);
    assert_eq!(make_base_jubilee(333), 300);
    assert_eq!(make_base_jubilee(377), 300);
}

#[test]
fn test_determine_jubistep() {
    assert_eq!(determine_jubistep(3), 10);
    assert_eq!(determine_jubistep(24), 10);
    assert_eq!(determine_jubistep(124), 50);
    assert_eq!(determine_jubistep(477), 50);
    assert_eq!(determine_jubistep(3477), 500);
    assert_eq!(determine_jubistep(36777), 500);
}

#[test]
fn test_next_jubilee() {
    assert_eq!(next_jubilee(7), 10);
    assert_eq!(next_jubilee(18), 20);
    assert_eq!(next_jubilee(47), 50);
    assert_eq!(next_jubilee(118), 150);
    assert_eq!(next_jubilee(301), 350);
    assert_eq!(next_jubilee(371), 400);
    assert_eq!(next_jubilee(3301), 3500);
    assert_eq!(next_jubilee(3601), 4000);
}

// repdigit tests
#[test]
fn test_is_repdigit() {
    assert_eq!(is_repdigit(111), true);
    assert_eq!(is_repdigit(121), false);
    assert_eq!(is_repdigit(39), false);
    assert_eq!(is_repdigit(4447444), false);
}

#[test]
fn test_make_base_repdigit() {
    assert_eq!(make_base_repdigit(101), 111);
    assert_eq!(make_base_repdigit(121), 111);
    assert_eq!(make_base_repdigit(994), 999);
    assert_eq!(make_base_repdigit(999), 999);
    assert_eq!(make_base_repdigit(111111), 111111);
    assert_eq!(make_base_repdigit(427281), 444444);
}

#[test]
fn test_make_repdigit() {
    assert_eq!(make_repdigit("1", 3), 111);
    assert_eq!(make_repdigit("4", 4), 4444);
    assert_eq!(make_repdigit("7", 4), 7777);
    assert_eq!(make_repdigit("9", 5), 99999);
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

#[test]
fn test_replace_last_characters() {
    assert_eq!(replace_last_characters(7, 1), 0);
    assert_eq!(replace_last_characters(55, 1), 50);
    assert_eq!(replace_last_characters(363, 1), 360);
    assert_eq!(replace_last_characters(99771, 1), 99770);
    assert_eq!(replace_last_characters(444263, 2), 444200);
    assert_eq!(replace_last_characters(3274263, 5), 3200000);
}
