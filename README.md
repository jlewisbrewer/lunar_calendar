# lunar_calendar
The goal of this project is to find the phases of the moon and to translate Gregorian caldendar dates into various lunar and lunar-solar calendars.
Currently this program can only find the phases of the moon.

## Installation

Build this project using cargo. To find the phase of the moon for any date, type `cargo run DAY MONTH YEAR` on the command line.

## Examples

```
cargo run 12 12 1983
Utc: 440099641
The moon is 50.52% full.
It is a first quarter moon`
```
## To Do
  * Implement calendars to translate the date
  * Add finer search options to search by timezones
  * Find dates of eclipses
  * Translate this into an iOS app

