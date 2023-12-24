mod macros;
pub mod units;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Default)]
pub struct UnitMatrix {
    pub length: i8,
    pub time: i8,
    pub mass: i8,
    pub current: i8,
    pub thermal: i8,
    pub amount: i8,
    pub candela: i8,
}

impl From<&[i8]> for UnitMatrix {
    fn from(value: &[i8]) -> Self {
        let mut output = Self::default();
        for (i, v) in value.iter().enumerate() {
            match i {
                0 => output.length = *v,
                1 => output.time = *v,
                2 => output.mass = *v,
                3 => output.current = *v,
                4 => output.thermal = *v,
                5 => output.amount = *v,
                6 => output.candela = *v,
                _ => unimplemented![],
            }
        }
        output
    }
}

impl From<[i8; 7]> for UnitMatrix {
    fn from(value: [i8; 7]) -> Self {
        Self {
            length: value[0],
            time: value[1],
            mass: value[2],
            current: value[3],
            thermal: value[4],
            amount: value[5],
            candela: value[6],
        }
    }
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
    fn to_base(&self, v: f64) -> f64;
    fn from_base(&self, v: f64) -> f64;
    fn matrix(&self) -> UnitMatrix;
}

#[derive(PartialEq, Clone)]
pub struct Measurement {
    matrix: UnitMatrix,
    value: f64,
}

impl Mul for Measurement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            matrix: self.matrix + rhs.matrix,
            value: self.value * rhs.value,
        }
    }
}

impl Div for Measurement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            matrix: self.matrix - rhs.matrix,
            value: self.value / rhs.value,
        }
    }
}

impl Add for Measurement {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.matrix == rhs.matrix {
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
        if self.matrix == rhs.matrix {
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
    pub fn with_unit_matrix(value: f64, matrix: UnitMatrix) -> Self {
        Self { matrix, value }
    }
    pub fn from_unit_matrix(&self, matrix: UnitMatrix) -> Option<f64> {
        if self.matrix == matrix {
            Some(self.value)
        } else {
            None
        }
    }
    pub fn from_unit(unit: impl Unit, v: f64) -> Self {
        Self {
            matrix: unit.matrix(),
            value: unit.to_base(v),
        }
    }
    pub fn to_unit(&self, unit: impl Unit) -> Option<f64> {
        if self.matrix == unit.matrix() {
            Some(unit.from_base(self.value))
        } else {
            None
        }
    }
    pub fn matrix(&self) -> UnitMatrix {
        self.matrix
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}
