pub fn next_repdigit(i: u32) -> u32 {
    let in_int: u32;
    if i < 10 {
        in_int = 10;
    } else {
        in_int = i;
    }
    let in_str = in_int.to_string();
    let in_len = in_str.len();

    let mut out_str = "".to_string();
    let step_str = String::from_utf8(vec![b'1'; in_len]).unwrap();
    let step_int = step_str.parse::<u32>().unwrap();

    for _ in 0..in_len {
        out_str += &in_str[0..1];
    }

    let mut out_int = out_str.parse::<u32>().unwrap();
    while out_int <= in_int {
        out_int += step_int
    }

    return out_int;
}

#[test]
fn test_next_repdigit() {
    assert_eq!(next_repdigit(0), 11);
    assert_eq!(next_repdigit(3), 11);
    assert_eq!(next_repdigit(10), 11);
    assert_eq!(next_repdigit(12), 22);
    assert_eq!(next_repdigit(288), 333);
    assert_eq!(next_repdigit(12345), 22222);
    assert_eq!(next_repdigit(75000), 77777);
    assert_eq!(next_repdigit(4900111), 5555555);
}
