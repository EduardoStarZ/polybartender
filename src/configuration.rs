use std::env::var;

pub fn read_config() {
    let config_file = var("HOME").unwrap();

    println!("{}", config_file);
}
