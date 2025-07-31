use std::process::{Child, Command};

pub fn launch_polybar(bar : String) -> Child {
    let child = match Command::new("polybar")
        .arg(bar)
        .spawn() {
            Ok(value) => value,
            Err(_) => panic!("Polybar could not be executed")
        };

    return child;
}

pub fn bar_command(bar: u32, action: String) -> bool {
    let _ = match Command::new("polybar-msg")
        .arg("-p")
        .arg(bar.to_string())
        .arg("cmd")
        .arg(action)
        .output() {
            Ok(value) => value,
            Err(_) => return false
        };

    return true;
}
