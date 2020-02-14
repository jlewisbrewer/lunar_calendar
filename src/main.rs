extern crate chrono;
mod lunar;
use chrono::prelude::*;
use std::time::{Instant, SystemTime};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);

    let year : i32 = args[3].parse().unwrap();
    let month : u32 = args[1].parse().unwrap();
    let day : u32 = args[2].parse().unwrap();

    println!{"{:?}", args};
    let dt = Utc.ymd(year, month, day).and_hms(17, 54, 1);
    println!{"Utc: {:?}", dt.timestamp()};

    lunar::get_lunar_phase(dt.timestamp());
}

