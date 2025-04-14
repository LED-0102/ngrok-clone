use std::env;
use std::str::FromStr;
use std::fmt::Debug;

pub fn get_config<T>(field: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Display, T: FromStr<Err: Debug>
{
    let val_str = env::var(field).expect(&format!("Environment variable {} not set", field));
    val_str.parse::<T>().expect(&format!("Could not parse {} into desired type", field))
}