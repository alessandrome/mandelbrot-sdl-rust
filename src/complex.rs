mod add;
mod sub;
mod mul;
mod div;
mod neg;
mod norm;

use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(PartialEq, Copy, Clone)]
pub struct Complex {
    pub real: f32,
    pub imaginary: f32,
}

impl Complex {
    pub fn new(real: f32, imaginary: f32) -> Self {
        Complex { real, imaginary }
    }

    pub fn is_real(&self) -> bool {
        self.imaginary == 0_f32
    }
}

// pub fn Complex(r: f32, i: f32) -> Complex {
//     Complex::new(r, i)
// }

static Complex: fn(f32, f32) -> Complex = Complex::new;

impl From<(f32, f32)> for Complex {
    fn from(tuple: (f32, f32)) -> Self {
        Self { real: tuple.0, imaginary: tuple.1 }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{:+}i)", self.real, self.imaginary)
    }
}

pub type C32 = Complex;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print() {
        let c = Complex(10_f32, 12_f32);
        assert_eq!(c.to_string(), "(10+12i)");
        let c = Complex(10_f32, -6.0005_f32);
        assert_eq!(c.to_string(), "(10-6.0005i)");
    }
}
