use std::env::{current_exe};

fn curexe() -> String {
        let c = current_exe().unwrap();
        return c.into_os_string().into_string().unwrap();
    }

pub fn config_file() -> String{
    return curexe().to_string() + ".yaml"
}
