use std::ops::Neg;
use super::*;

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negate_complex() {
        let c1 = -(Complex(1_f32, 3_f32));
        assert_eq!(c1.real, -1_f32);
        assert_eq!(c1.imaginary, -3_f32);
    }
}
