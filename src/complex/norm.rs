use super::*;

impl Complex {
    pub fn norm_sqrt(&self) -> f32 {
        self.real * self.real + self.imaginary * self.imaginary
    }

    pub fn norm(&self) -> f32 {
        self.norm_sqrt().sqrt()
    }

    // Alias for "norm"
    pub fn abs(&self) -> f32 {
        self.norm()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_norm_sqrt() {
        let r = 3.5f32;
        let i = -2.15f32;
        let c1 = Complex::new(r, i);
        assert_eq!(c1.norm_sqrt(), (r * r) + (i * i));
    }

    #[test]
    fn test_norm() {
        let r = 3.5f32;
        let i = -2.15f32;
        let c1 = Complex::new(r, i);
        assert_eq!(c1.norm(), ((r * r) + (i * i)).sqrt());
    }
}
