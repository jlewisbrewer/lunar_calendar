//http://astropixels.com/ephemeris/moon/synodicmonth2001.html
// length of the mean synodic month in seconds
const SYNODIC_MONTH : i64 = 2551443;
// offset based on new moon on Jan 7, 12:35 PM, 1970
const SYNODIC_OFFSET : i64 = 606900;
const NEGATIVE_SYNODIC_OFFSET : i64 = SYNODIC_OFFSET - SYNODIC_MONTH;
// Value for tuning how exact the full, new, and quarter moon phases are
// Has to be between 0 and 1
const TUNING_VALUE : f64 = 0.3;



fn get_phase_value(date: i64) -> f64 {
    let period = SYNODIC_MONTH / 8;    
    let mut diff = (date - SYNODIC_OFFSET) % SYNODIC_MONTH;
    
    if date < SYNODIC_OFFSET {
        // If the date occured before epoch, we have to deal with negative
        // numbers.
        if date >= NEGATIVE_SYNODIC_OFFSET {
            // This will need some checks
            diff = (date - NEGATIVE_SYNODIC_OFFSET) % SYNODIC_MONTH;
        } else if date < NEGATIVE_SYNODIC_OFFSET {
            // This seems to work
            diff = SYNODIC_MONTH - ((date - NEGATIVE_SYNODIC_OFFSET) * -1 % SYNODIC_MONTH);
        } else {
            // If the date is less than the offset, then we need to base the
            // starting point of the new moon cycle to be a month - offset earler.
            diff = (date + SYNODIC_MONTH - SYNODIC_OFFSET) % SYNODIC_MONTH;
        }
    }

    diff as f64 / period as f64
}

fn get_phase_string (phase_var: f64) -> String {
    let mut phase : i64 = 0;
    
    assert!(TUNING_VALUE > 0.0, TUNING_VALUE < 1.0);
    if phase_var >= 8.0 - TUNING_VALUE && phase_var <= 0.0 + TUNING_VALUE {
        phase = 1;
    } else if phase_var <= 2.0 + TUNING_VALUE && phase_var >= 2.0 - TUNING_VALUE {
        phase = 2;
    } else if phase_var <= 4.0 + TUNING_VALUE && phase_var >= 4.0 - TUNING_VALUE {
        phase = 3;
    } else if phase_var <= 6.0 + TUNING_VALUE && phase_var >= 6.0 - TUNING_VALUE {
        phase = 4;
    }
    match phase {
        1 => {
            return "It is a new moon".to_string()
        },
        2 => {
            return "It is a first quarter moon".to_string()
        },
        3 => {
            return "it is a full moon".to_string()
        },
        4 => {
            return "It is a last quarter moon".to_string()
        },
        
        _ => {
            if phase_var < 2.0 {
                return "It is a waxing crescent.".to_string()
            } else if phase_var > 2.0 && phase_var < 4.0 {
                return "It is a waxing gibbous.".to_string()
            } else if phase_var > 4.0 && phase_var < 6.0 {
                return "It is a waning gibbous.".to_string()
            } else {
                return "It is a waning crescent.".to_string()
            }
        }
    };
}

fn get_fullness_percentage (phase_var: f64) -> f64 {
    if phase_var <= 4.0 {
        return (phase_var / 4.0) * 100.0
    }
    ((8.0 - phase_var) / 4.0) * 100.0
}

pub fn get_lunar_phase(date: i64) {
    let phase_var = get_phase_value(date);
    println!{"The moon is {:.2}% full.", get_fullness_percentage(phase_var)};
    println!{"{}", get_phase_string(phase_var)};

}

#[test]
fn test_get_phase_value() {
    let floor : f64 = 5.0;
    let ceiling : f64 = 6.0;
    let date : i64 = 1581567657;
    let test_case = get_phase_value(date);

    assert!(test_case < ceiling, test_case > floor);
}

#[test]
fn test_get_phase_string() {
    let test_phase_variable : f64 = 5.07;
    assert_eq!("It is a waning gibbous.".to_string(), 
        get_phase_string(test_phase_variable));
}