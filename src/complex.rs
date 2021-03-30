use std::ops::{Add, Div, Mul, Neg, Sub};

/// *Naïve* struct Complex for complex numbers.
/// Example to implement simple mathematical operations with a struct via Traits.
/// Not for production work! Use the *num* crate instead

#[derive(Debug)]
pub struct Complex {
    re: f64,
    im: f64,
}

#[allow(dead_code)]
impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Spawn a new complex number from the polar representation m e^(I phi)
    pub fn polar(modulus: f64, angle: f64) -> Self {
        modulus * Complex::exp_i(angle)
    }

    /// Euler's formula
    pub fn exp_i(x: f64) -> Self {
        Self {
            re: x.cos(),
            im: x.sin(),
        }
    }

    /// real part
    pub fn re(&self) -> f64 {
        self.re
    }

    /// imaginary part
    pub fn im(&self) -> f64 {
        self.im
    }

    ///Inverse of a complex number, i.e. z -> 1/z
    pub fn inv(&self) -> Self {
        let mod_sq = self.re * self.re + self.im * self.im;
        Self::new(self.re / mod_sq, -self.im / mod_sq)
    }

    /// Conjugate of a complex number, i.e. x + iy -> x - iy
    pub fn conj(&self) -> Self {
        Self::new(self.re, -self.im)
    }
}

impl Mul<f64> for Complex {
    // Complex * f64
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Mul<Complex> for f64 {
    // f64 * Complex
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}

impl Add<Complex> for Complex {
    type Output = Self;

    fn add(self, rhs: Complex) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
}

impl Sub<Complex> for Complex {
    type Output = Self;

    fn sub(self, rhs: Complex) -> Self {
        //in production code we would write:
        //Self::new(self.re - rhs.re, self.im - rhs.im)

        //but let's have some fun and use
        //the already implemented traits Add, Neg
        self + -rhs
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Self;

    fn div(self, rhs: Complex) -> Self::Output {
        //Again, let's reuse previous traits
        //Namely: z1/z2 = z1*(1/z2)
        self * rhs.inv()
    }
}
