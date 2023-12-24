use crate::{units, Unit, UnitMatrix};

units! {
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
        0.9144 => Yard
        0.0254 => Inch
        10e-10 => Angstrom
    }
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
        Eon.scale() * 4.32 => Kalpa
        Year.scale() * 10e9 => Eon
        Year.scale() * 2.3e8 => GalacticYear
        Year.scale() * (2148.0 + 1.0/3.0) => Age
        Year.scale() * 1000.0 => Millenium
        Year.scale() * 100.0 => Century
        Year.scale() * 50.0 => Jubilee
        Year.scale() * 15.0 => Indiction
        Year.scale() * 10.0 => Decade
        Year.scale() + Day.scale() => LeapYear
        Year.scale() * 5.0 => Lustrum
        Year.scale() * 4.0 => Olympiad
        31_557_600.0 => Year
        Day.scale() * 354.37 => LunarYear
        Week.scale() * 18.0 => Semester
        Day.scale() * 40.0 => Quarantine
        Day.scale() * 30.0 => Month
        Week.scale() => Fortnight
        60.0e2 * 24.0 * 7.0 => Week
        Day.scale() * 0.001 => Milliday
        60.0e2 * 24.0 => Day
        60.0e2 => Hour
        60.0 => Minute
        10e-03 => Jiffy
        10e-08 => Shake
        5.39e-44 => PlanckTime
    }
}
