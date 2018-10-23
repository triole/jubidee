use clap::ArgMatches;

pub fn bool(args: &ArgMatches, argname: &str) -> bool {
    let mut r = false;
    let n = args.occurrences_of(argname);
    if n > 0 {
        r = true;
    }
    return r;
}

// pub fn val_str(args: &ArgMatches, argname: &str) -> String {
//     let v = args.value_of(argname).unwrap();
//     let val = v.to_string();
//     return val;
// }
//
// pub fn val_op_str(args: &ArgMatches, argname: &str) -> Option<String> {
//     if bool(args, argname) == true {
//         Some(val_str(args, argname))
//     } else {
//         None
//     }
// }
//
pub fn val_usize(args: &ArgMatches, argname: &str) -> usize {
    let v = args.value_of(argname).unwrap();
    let val = v.parse::<usize>().unwrap();
    return val;
}
//
// pub fn val_op_usize(args: &ArgMatches, argname: &str) -> Option<usize> {
//     if bool(args, argname) == true {
//         Some(val_usize(args, argname))
//     } else {
//         None
//     }
// }
