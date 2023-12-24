use crate::{units, Unit, UnitMatrix};

units! {
    // Length
    [1, 0, 0, 0, 0, 0, 0] {
        10e+30 => Quettameter
        10e+27 => Ronnameter
        10e+24 => Yottameter
        10e+21 => Zettameter
        10e+18 => Exameter
        10e+15 => Petameter
        10e+12 => Tetrameter
        10e+09 => Gigameter
        10e+06 => Megameter
        10e+03 => Kilometer
        10e+02 => Hectometer
        10e+01 => Decameter
        1.0 => Meter
        10e-01 => Decimeter
        10e-02 => Centimeter
        10e-03 => Millimeter
        10e-06 => Micrometer
        10e-09 => Nanometer
        10e-12 => Picometer
        10e-15 => Femtometer
        10e-18 => Attometer
        10e-21 => Zeptometer
        10e-24 => Yoctometer
        10e-27 => Rontometer
        10e-30 => Quectometer
        1.6e-35 => Planck
        0.9144 => Yard
        0.0254 => Inch
        10e-10 => Angstrom
    }
    // Time
    [0, 1, 0, 0, 0, 0, 0] {
        10e+30 => Quettasecond
        10e+27 => Ronnasecond
        10e+24 => Yottasecond
        10e+21 => Zettasecond
        10e+18 => Exasecond
        10e+15 => Petasecond
        10e+12 => Tetrasecond
        10e+09 => Gigasecond
        10e+06 => Megasecond
        10e+03 => Kilosecond
        10e+02 => Hectosecond
        10e+01 => Decasecond
        1.0 => Second
        10e-01 => Decisecond
        10e-02 => Centisecond
        10e-03 => Millisecond
        10e-06 => Microsecond
        10e-09 => Nanosecond
        10e-12 => Picosecond
        10e-15 => Femtosecond
        10e-18 => Attosecond
        10e-21 => Zeptosecond
        10e-24 => Yoctosecond
        10e-27 => Rontosecond
        10e-30 => Quectosecond
        Eon::SCALE * 4.32 => Kalpa
        Year::SCALE * 10e9 => Eon
        Year::SCALE * 2.3e8 => GalacticYear
        Year::SCALE * (2148.0 + 1.0/3.0) => Age
        Year::SCALE * 1000.0 => Millenium
        Year::SCALE * 100.0 => Century
        Year::SCALE * 50.0 => Jubilee
        Year::SCALE * 15.0 => Indiction
        Year::SCALE * 10.0 => Decade
        Year::SCALE + Day::SCALE => LeapYear
        Year::SCALE * 5.0 => Lustrum
        Year::SCALE * 4.0 => Olympiad
        31_557_600.0 => Year
        Day::SCALE * 354.37 => LunarYear
        Week::SCALE * 18.0 => Semester
        Day::SCALE * 40.0 => Quarantine
        Day::SCALE * 30.0 => Month
        Week::SCALE => Fortnight
        60.0e2 * 24.0 * 7.0 => Week
        Day::SCALE * 0.001 => Milliday
        60.0e2 * 24.0 => Day
        60.0e2 => Hour
        60.0 => Minute
        10e-03 => Jiffy
        10e-08 => Shake
        5.39e-44 => PlanckTime
    }
    // Mass
    [0, 0, 1, 0, 0, 0, 0] {

        10e+30 => Quettagram
        10e+27 => Ronnagram
        10e+24 => Yottagram
        10e+21 => Zettagram
        10e+18 => Exagram
        10e+15 => Petagram
        10e+12 => Tetragram
        10e+09 => Gigagram
        10e+06 => Megagram
        10e+03 => Kilogram
        10e+02 => Hectogram
        10e+01 => Decagram
        1.0 => Gram
        10e-01 => Decigram
        10e-02 => Centigram
        10e-03 => Milligram
        10e-06 => Microgram
        10e-09 => Nanogram
        10e-12 => Picogram
        10e-15 => Femtogram
        10e-18 => Attogram
        10e-21 => Zeptogram
        10e-24 => Yoctogram
        10e-27 => Rontogram
        10e-30 => Quectogram
    }
    // Force
    [1, -2, 1, 0, 0, 0, 0] {
        1000.0 => Newton
    }
}
