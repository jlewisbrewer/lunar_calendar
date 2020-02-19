# lunar_calendar
The goal of this project is to find the phases of the moon and to translate Gregorian caldendar dates into various lunar and lunar-solar calendars.
Currently this program can only find the phases of the moon.

## Installation

Build this project using cargo. To find the phase of the moon for any date, type `cargo run DAY MONTH YEAR` on the command line.

## Examples

### Input
```
cargo run 7 15 257
```
### Output
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lunar_calendar 7 15 257`
The moon is 90.59% full.
It is a waning gibbous.

```
## To Do
  * Implement calendars to translate the date
  * Add finer search options to search by timezones
  * Find dates of eclipses
  * Translate this into an iOS app

