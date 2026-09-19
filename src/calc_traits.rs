pub trait Absolutable {
    fn abs(&self) -> Self;
}

impl Absolutable for f32 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

impl Absolutable for f64 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

pub trait Zeroable {
    fn zero() -> Self;
}

pub trait Floatable<F> {
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: F) -> Self;
    fn sqrt(self) -> Self;
}

impl Floatable<f32> for f32 {
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn powf(self, n: f32) -> Self {
        self.powf(n)
    }
}

impl Floatable<f64> for f64 {
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn powf(self, n: f64) -> Self {
        self.powf(n)
    }
}
