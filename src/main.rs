extern crate chrono;
mod lunar;
use chrono::prelude::*;
use std::time::{Instant, SystemTime};

fn main() {
    // s is start of new moon, can be divided into 8ths to find the phases
    // let s : u64 = 2551443;
    // let p = s / 8;
    // let so : u64 = 606900;
    // let n = Instant::now();
    // let mut sn = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    // println!("Now : {:?}", sn);
    
    
    // let diff = ((sn) - so) % s;
    // println!("{}", diff);
    // let phase : f64 = (diff as f64 / p as f64);
    // println!("{}", phase);
    // // Let's turn this into a percentage
    // let mut percentage : f64 = 0.0;
    // if phase > 4.0 {
    //     percentage = ((8.0 - phase) / 4.0) * 100.0;
    // }
    // else {
    //     percentage = (phase / 4.0) * 100.0;
    // }
    // println!("The moon is {:.2}% full", percentage);

    // // Ok so this just finds the phase of the moon based on the base of the era
    // // To do, we need to make it into constants and integrate it.
    // // Will need to convert any date to era and then find the moon time
    
    // // Need to convert seconds to a date
    // let today = Local::now();
    // println!("{}", today);

    // //Tests

    let dt = Utc.ymd(1955, 4, 25).and_hms(0, 0, 0);
    println!{"Utc: {:?}", dt.timestamp()};

    lunar::get_lunar_phase(dt.timestamp());
}

