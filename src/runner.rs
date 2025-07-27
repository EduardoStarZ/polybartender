use std::process::Command;
use crate::configuration;

pub fn launch_polybar(bars : Vec<String>) -> bool {

    for bar in bars{
        let _childs  = match Command::new("polybar")
            .arg(bar)
            .spawn() {
                Ok(value) => value,
                Err(_) => return false
            };
    }

    configuration::read_config();
    return true;
}
