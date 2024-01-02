mod macros;
pub mod units;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Default)]
pub struct UnitMatrix {
    pub length: i8,
    pub time: i8,
    pub mass: i8,
    pub current: i8,
    pub temperature: i8,
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
                4 => output.temperature = *v,
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
            temperature: value[4],
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
            temperature: self.temperature + rhs.temperature,
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
            temperature: self.temperature - rhs.temperature,
            amount: self.amount - rhs.amount,
            candela: self.candela - rhs.candela,
        }
    }
}

impl Mul<i8> for UnitMatrix {
    type Output = Self;
    fn mul(self, rhs: i8) -> Self {
        Self {
            length: self.length * rhs,
            time: self.time * rhs,
            mass: self.mass * rhs,
            current: self.current * rhs,
            temperature: self.temperature * rhs,
            amount: self.amount * rhs,
            candela: self.candela * rhs,
        }
    }
}

impl Div<i8> for UnitMatrix {
    type Output = Self;
    fn div(self, rhs: i8) -> Self {
        Self {
            length: self.length / rhs,
            time: self.time / rhs,
            mass: self.mass / rhs,
            current: self.current / rhs,
            temperature: self.temperature / rhs,
            amount: self.amount / rhs,
            candela: self.candela / rhs,
        }
    }
}

#[allow(non_upper_case_globals)]
impl UnitMatrix {
    pub const Scalar: Self = Self {
        length: 0,
        time: 0,
        mass: 0,
        current: 0,
        temperature: 0,
        amount: 0,
        candela: 0,
    };
    pub const Length: Self = Self {
        length: 1,
        time: 0,
        mass: 0,
        current: 0,
        temperature: 0,
        amount: 0,
        candela: 0,
    };
    pub const Time: Self = Self {
        length: 0,
        time: 1,
        mass: 0,
        current: 0,
        temperature: 0,
        amount: 0,
        candela: 0,
    };
    pub const Mass: Self = Self {
        length: 0,
        time: 0,
        mass: 1,
        current: 0,
        temperature: 0,
        amount: 0,
        candela: 0,
    };
    pub const Current: Self = Self {
        length: 0,
        time: 0,
        mass: 0,
        current: 1,
        temperature: 0,
        amount: 0,
        candela: 0,
    };
    pub const Temperature: Self = Self {
        length: 0,
        time: 0,
        mass: 0,
        current: 0,
        temperature: 1,
        amount: 0,
        candela: 0,
    };
    pub const Amount: Self = Self {
        length: 0,
        time: 0,
        mass: 0,
        current: 0,
        temperature: 0,
        amount: 1,
        candela: 0,
    };
    pub const Candela: Self = Self {
        length: 0,
        time: 0,
        mass: 0,
        current: 0,
        temperature: 0,
        amount: 0,
        candela: 1,
    };
}

pub trait Unit {
    fn to_base(&self, v: f64) -> f64;
    fn from_base(&self, v: f64) -> f64;
    fn matrix(&self) -> UnitMatrix;
    fn new(v: f64) -> Measurement
    where
        Self: Default,
    {
        Measurement::from_unit(Self::default(), v)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Measurement {
    matrix: UnitMatrix,
    value: f64,
}

impl Mul<f64> for Measurement {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value * rhs,
            ..self
        }
    }
}

impl Div<f64> for Measurement {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value / rhs,
            ..self
        }
    }
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
    pub fn from_unit(unit: impl Unit, v: f64) -> Self {
        Self {
            matrix: unit.matrix(),
            value: unit.to_base(v),
        }
    }
    pub fn convert_to(&self, unit: impl Unit) -> Option<f64> {
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
