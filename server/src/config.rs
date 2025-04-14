use std::env;

pub fn get_config (field: &str) -> T {
    let val: T = env::var(field).expect(&format!("Environment variable {} not set", field));

    val
}