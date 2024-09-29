use std::ops::Mul;
use super::*;

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: (self.real * rhs.real) - (self.imaginary * rhs.imaginary),
            imaginary: (self.real * rhs.imaginary) + (rhs.real * self.imaginary),
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real * rhs,
            imaginary: self.imaginary * rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mul_complex() {
        let (a1, a2, b1, b2) = (5_f32, -3_f32, 4_f32, 7_f32);
        let c1 = Complex::new(a1, b1);
        let c2 = Complex::new(a2, b2);
        let c_mul = c1 * c2;
        assert_eq!(c_mul.real, a1 * a2 - b1 * b2);
        assert_eq!(c_mul.imaginary, a1 * b2 + a2 * b1);
    }
}
