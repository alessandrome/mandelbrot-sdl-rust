use std::ops::Add;
use super::*;

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Add<f32> for Complex {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real + rhs,
            imaginary: self.imaginary,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_complex() {
        let c1 = Complex::new(1_f32, 3_f32);
        let c2 = Complex::new(3_f32, 7_f32);
        let c_sum = c1 + c2;
        assert_eq!(c_sum.real, 4_f32);
        assert_eq!(c_sum.imaginary, 10_f32);
    }
}
