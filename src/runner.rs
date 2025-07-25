use std::process::Command;

pub fn launch_polybar(bars : Vec<String>) -> bool {
    
    for bar in bars{
        let _childs  = match Command::new("polybar")
            .arg(bar)
            .spawn() {
                Ok(value) => value,
                Err(_) => return false
            };
    }
    return true;
}
