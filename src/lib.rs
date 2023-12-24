mod macros;
pub mod units;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct UnitMatrix {
    length: i8,
    time: i8,
    mass: i8,
    current: i8,
    thermal: i8,
    amount: i8,
    candela: i8,
}

impl Add for UnitMatrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            length: self.length + rhs.length,
            time: self.time + rhs.time,
            mass: self.mass + rhs.mass,
            current: self.current + rhs.current,
            thermal: self.thermal + rhs.thermal,
            amount: self.amount + rhs.amount,
            candela: self.candela + rhs.candela,
        }
    }
}

impl Sub for UnitMatrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            length: self.length - rhs.length,
            time: self.time - rhs.time,
            mass: self.mass - rhs.mass,
            current: self.current - rhs.current,
            thermal: self.thermal - rhs.thermal,
            amount: self.amount - rhs.amount,
            candela: self.candela - rhs.candela,
        }
    }
}

pub trait Unit {
    fn scale(&self) -> f64;
    fn matrix(&self) -> UnitMatrix;
}

impl Unit for (f64, UnitMatrix) {
    fn scale(&self) -> f64 {
        self.0
    }
    fn matrix(&self) -> UnitMatrix {
        self.1
    }
}

impl Unit for (UnitMatrix, f64) {
    fn scale(&self) -> f64 {
        self.1
    }
    fn matrix(&self) -> UnitMatrix {
        self.0
    }
}

#[derive(PartialEq, Clone)]
pub struct Measurement {
    units: UnitMatrix,
    value: f64,
}

impl Mul for Measurement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            units: self.units + rhs.units,
            value: self.value * rhs.value,
        }
    }
}

impl Div for Measurement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            units: self.units - rhs.units,
            value: self.value / rhs.value,
        }
    }
}

impl Add for Measurement {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.units == rhs.units {
            Some(Self {
                value: self.value + rhs.value,
                ..self
            })
        } else {
            None
        }
    }
}

impl Sub for Measurement {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.units == rhs.units {
            Some(Self {
                value: self.value - rhs.value,
                ..self
            })
        } else {
            None
        }
    }
}

impl Measurement {
    pub fn from_unit(unit: impl Unit, v: f64) -> Self {
        Self {
            units: unit.matrix(),
            value: v * unit.scale(),
        }
    }
    pub fn to_unit(&self, unit: impl Unit) -> Option<f64> {
        if self.units == unit.matrix() {
            Some(self.value / unit.scale())
        } else {
            None
        }
    }
}
