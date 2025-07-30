use std::process::Command;

pub fn launch_polybar(bar : String) -> u32 {
    let child = match Command::new("polybar")
        .arg(bar)
        .spawn() {
            Ok(value) => value,
            Err(_) => return 0
        };

    return child.id();
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
