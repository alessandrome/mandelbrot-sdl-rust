use std::ops::{Sub};
use super::*;

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Sub<f32> for Complex {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real - rhs,
            imaginary: self.imaginary,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub_complex() {
        let c1 = Complex::new(1_f32, 3_f32);
        let c2 = Complex::new(3_f32, 7_f32);
        let c_sub = c1 - c2;
        assert_eq!(c_sub.real, -2_f32);
        assert_eq!(c_sub.imaginary, -4_f32);
    }
}
