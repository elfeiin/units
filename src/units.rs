use crate::{units, Unit, Units};

units! {
    // Length
    Units::Length; {
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
    Units::Time; {
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
        Eon.to_base(1.0) * 4.32 => Kalpa
        Year.to_base(1.0) * 10e9 => Eon
        Year.to_base(1.0) * 2.3e8 => GalacticYear
        Year.to_base(1.0) * (2148.0 + 1.0/3.0) => Age
        Year.to_base(1.0) * 1000.0 => Millenium
        Year.to_base(1.0) * 100.0 => Century
        Year.to_base(1.0) * 50.0 => Jubilee
        Year.to_base(1.0) * 15.0 => Indiction
        Year.to_base(1.0) * 10.0 => Decade
        Year.to_base(1.0) + Day.to_base(1.0) => LeapYear
        Year.to_base(1.0) * 5.0 => Lustrum
        Year.to_base(1.0) * 4.0 => Olympiad
        31_557_600.0 => Year
        Day.to_base(1.0) * 354.37 => LunarYear
        Week.to_base(1.0) * 18.0 => Semester
        Day.to_base(1.0) * 40.0 => Quarantine
        Day.to_base(1.0) * 30.0 => Month
        Week.to_base(1.0) => Fortnight
        60.0e2 * 24.0 * 7.0 => Week
        Day.to_base(1.0) * 0.001 => Milliday
        60.0e2 * 24.0 => Day
        60.0e2 => Hour
        60.0 => Minute
        10e-03 => Jiffy
        10e-08 => Shake
        5.39e-44 => PlanckTime
    }
    // Mass
    Units::Mass; {
        10e+27 => Quettagram
        10e+24 => Ronnagram
        10e+21 => Yottagram
        10e+18 => Zettagram
        10e+15 => Exagram
        10e+12 => Petagram
        10e+09 => Tetragram
        10e+06 => Gigagram
        10e+03 => Megagram
        1.0    => Kilogram
        10e-01 => Hectogram
        10e-02 => Decagram
        10e-03 => Gram
        10e-04 => Decigram
        10e-05 => Centigram
        10e-06 => Milligram
        10e-09 => Microgram
        10e-12 => Nanogram
        10e-15 => Picogram
        10e-18 => Femtogram
        10e-21 => Attogram
        10e-24 => Zeptogram
        10e-27 => Yoctogra
        10e-30 => Rontogra
        10e-33 => Quectogra
        1000.0 => MetricTon
        1016.047 => UsTon
    }
    // Current
    Units::Current; {
        1.0 => Amp
    }
    // Temperature
    Units::Temperature; {
        5.0 / 9.0, (459.67) => Fahrenheit
        1.0 => Kelvin
        1.0, 273.15 => Celsius
    }
    // Frequency
    Units::Time * -1; {
        1.0 => Hertz
    }
    // Force
    Units::Length + Units::Time * -2 + Units::Mass; {
        1.0 => Newton
    }
    // Pressure
    Units::Length + Units::Time * -2 + Units::Mass * -1; {
        1.0 => Pascal
    }
    // Energy
    Units::Length * 2 + Units::Time * -2 + Units::Mass; {
        1.0 => Joule
    }
    // Power
    Units::Length * 2 + Units::Time * -3 + Units::Mass; {
        1.0 => Watt
    }
    // Charge
    Units::Time + Units::Current; {
        1.0 => Coulomb
    }
    // Electric Potential
    Units::Length * -2 + Units::Time * -3 + Units::Mass + Units::Current * -1; {
        1.0 => Volt
    }
    // Capacitance
    Units::Length * -2 + Units::Time * 4 + Units::Mass * -1 + Units::Current; {
        1.0 => Farad
    }
    // Resistance
    Units::Length * 2 + Units::Time * -3 + Units::Mass + Units::Current * -2; {
        1.0 => Ohm
    }
    // Electrical Conductance
    Units::Length * -2 + Units::Time * 3 + Units::Mass * -1 + Units::Current * 2; {
        1.0 => Siemens
    }
    // Magnetic Flux
    Units::Length * 2 + Units::Time * -2 + Units::Mass + Units::Current * -1; {
        1.0 => Weber
    }
    // Magnetic Flux Density
    Units::Time * 2 + Units::Mass + Units::Current * -1; {
        1.0 => Tesla
    }
    // Inductance
    Units::Length * 2 + Units::Time * -2 + Units::Mass + Units::Current * -2; {
        1.0 => Henry
    }
    // Catalytic Activity
    Units::Time * -1 + Units::Amount; {
        1.0 => Katal
    }
    // Speed
    Units::Length + Units::Time * -1; {
        1.0 => MetersPerSecond
        1.0 / 3.6 => KMPH
    }
    // Acceleration
    Units::Length + Units::Time * -2; {
        1.0 => MetersPerSecondSquared
    }
    // Density
    Units::Mass + Units::Length * 3; {
        1.0 => KgPerM3
    }
    // Force Moment
    Newton.matrix() + Units::Length; {
        1.0 => NetwonMeter
    }
}
