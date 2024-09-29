use std::ops::Div;
use super::*;

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            real: ((self.real * rhs.real) + (self.imaginary * rhs.imaginary)) / (rhs.real * rhs.real + rhs.imaginary * rhs.imaginary),
            imaginary: ((self.imaginary * rhs.real) - (self.real * rhs.imaginary)) / (rhs.real * rhs.real + rhs.imaginary * rhs.imaginary),
        }
    }
}

impl Div<f32> for Complex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
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
    fn test_div_complex() {
        let (a1, a2, b1, b2) = (5_f32, -3_f32, 4_f32, 7_f32);
        let c1 = Complex::new(a1, b1);
        let c2 = Complex::new(a2, b2);
        let c_div = c1 / c2;
        let expected_divisor = (a2 * a2 + b2 * b2);
        assert_eq!(c_div.real, (a1 * a2 + b1 * b2) / expected_divisor);
        assert_eq!(c_div.imaginary, (a2 * b1 - a1 * b2) / expected_divisor);
    }
}