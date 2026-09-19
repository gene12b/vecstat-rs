use std::ops::{Add, Div, Mul, Sub};

use nonempty::NonEmpty;

use crate::{calc_traits::*, errors::VecStatError};

pub trait StatisticsUnorderedF32NonEmpty<T> {
    fn mean(self) -> T;
    fn avg(self) -> T;
    fn geometric_mean(self) -> T;
    fn variance(self) -> T;
    fn variance_sample(self) -> T;
    fn std_dev(self) -> T;
    fn std_dev_sample(self) -> T;
    fn covariance(self, other: Self) -> Result<T, VecStatError<T>>;
    fn covariance_sample(self, other: Self) -> Result<T, VecStatError<T>>;
    fn quadratic_mean(self) -> T;
    fn rms(self) -> T;
}
pub trait StatisticsUnorderedF64NonEmpty<T> {
    fn mean(self) -> T;
    fn avg(self) -> T;
    fn geometric_mean(self) -> T;
    fn variance(self) -> T;
    fn variance_sample(self) -> T;
    fn std_dev(self) -> T;
    fn std_dev_sample(self) -> T;
    fn covariance(self, other: Self) -> Result<T, VecStatError<T>>;
    fn covariance_sample(self, other: Self) -> Result<T, VecStatError<T>>;
    fn quadratic_mean(self) -> T;
    fn rms(self) -> T;
}
pub trait StatisticsOrderedNonEmpty<T> {
    fn min(&self) -> &T;
    fn max(&self) -> &T;
}
pub trait StatisticsOrderedAbsNonEmpty<T> {
    fn abs_min(&self) -> T;
    fn abs_max(&self) -> T;
}

impl<T> StatisticsUnorderedF32NonEmpty<T> for NonEmpty<T>
where
    T: Zeroable
        + Add<Output = T>
        + Div<f32, Output = T>
        + Sub<Output = T>
        + Floatable<f32>
        + Clone
        + Mul<Output = T>,
    f32: std::ops::Mul<T>,
{
    fn mean(self) -> T {
        let len = self.len();
        self.into_iter()
            .fold(T::zero(), |folding, val| folding + val)
            / (len as f32)
    }
    fn avg(self) -> T {
        self.mean()
    }
    fn variance(self) -> T {
        let len = self.len();
        let avg = self.clone().avg();
        self.into_iter().fold(T::zero(), |folding, val| {
            folding + (val - avg.clone()).powi(2)
        }) / (len as f32)
    }
    fn std_dev(self) -> T {
        self.variance().sqrt()
    }
    fn covariance(self, other: Self) -> Result<T, VecStatError<T>> {
        let len = self.len();
        if len != other.len() {
            return Err(VecStatError::UnequalLengthsNonEmpty(
                len,
                other.len(),
                self,
                other,
            ));
        }
        let avg_a = self.clone().avg();
        let avg_b = other.clone().avg();
        Ok(self
            .into_iter()
            .zip(other)
            .fold(T::zero(), |folding, (val_a, val_b)| {
                folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
            })
            / (len as f32))
    }
    fn geometric_mean(self) -> T {
        let len = self.len();
        self.into_iter()
            .fold(T::zero(), |folding, val| folding * val)
            .powf(1.0 / (len as f32))
    }
    fn quadratic_mean(self) -> T {
        let len = self.len();

        self.into_iter()
            .fold(T::zero(), |folding, val| folding + val.powi(2))
            .div(len as f32)
            .sqrt()
    }
    fn rms(self) -> T {
        self.quadratic_mean()
    }
    fn std_dev_sample(self) -> T {
        self.variance_sample().sqrt()
    }
    fn variance_sample(self) -> T {
        let len = self.len() - 1;
        let avg = self.clone().avg();
        self.into_iter().fold(T::zero(), |folding, val| {
            folding + (val - avg.clone()).powi(2)
        }) / (len as f32)
    }
    fn covariance_sample(self, other: Self) -> Result<T, VecStatError<T>> {
        let len = self.len() - 1;
        if len != other.len() {
            return Err(VecStatError::UnequalLengthsNonEmpty(
                len,
                other.len(),
                self,
                other,
            ));
        }
        let avg_a = self.clone().avg();
        let avg_b = other.clone().avg();
        Ok(self
            .into_iter()
            .zip(other)
            .fold(T::zero(), |folding, (val_a, val_b)| {
                folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
            })
            / (len as f32))
    }
}

impl<T> StatisticsUnorderedF64NonEmpty<T> for NonEmpty<T>
where
    T: Zeroable
        + Add<Output = T>
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Floatable<f64>
        + Clone
        + Mul<Output = T>,
    f64: std::ops::Mul<T>,
{
    fn mean(self) -> T {
        let len = self.len();

        self.into_iter()
            .fold(T::zero(), |folding, val| folding + val)
            / (len as f64)
    }
    fn avg(self) -> T {
        self.mean()
    }
    fn variance(self) -> T {
        let len = self.len();
        let avg = self.clone().avg();

        self.into_iter().fold(T::zero(), |folding, val| {
            folding + (val - avg.clone()).powi(2)
        }) / (len as f64)
    }
    fn std_dev(self) -> T {
        self.variance().sqrt()
    }
    fn covariance(self, other: Self) -> Result<T, VecStatError<T>> {
        let len = self.len();
        if len != other.len() {
            return Err(VecStatError::UnequalLengthsNonEmpty(
                len,
                other.len(),
                self,
                other,
            ));
        }
        let avg_a = self.clone().avg();
        let avg_b = other.clone().avg();
        Ok(self
            .into_iter()
            .zip(other)
            .fold(T::zero(), |folding, (val_a, val_b)| {
                folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
            })
            / (len as f64))
    }
    fn geometric_mean(self) -> T {
        let len = self.len();

        self.into_iter()
            .fold(T::zero(), |folding, val| folding * val)
            .powf(1.0 / (len as f64))
    }
    fn quadratic_mean(self) -> T {
        let len = self.len();

        self.into_iter()
            .fold(T::zero(), |folding, val| folding + val.powi(2))
            .div(len as f64)
            .sqrt()
    }
    fn rms(self) -> T {
        self.quadratic_mean()
    }
    fn std_dev_sample(self) -> T {
        self.variance_sample().sqrt()
    }
    fn variance_sample(self) -> T {
        let len = self.len() - 1;
        let avg = self.clone().avg();

        self.into_iter().fold(T::zero(), |folding, val| {
            folding + (val - avg.clone()).powi(2)
        }) / (len as f64)
    }
    fn covariance_sample(self, other: Self) -> Result<T, VecStatError<T>> {
        let len = self.len() - 1;
        if len != other.len() {
            return Err(VecStatError::UnequalLengthsNonEmpty(
                len,
                other.len(),
                self,
                other,
            ));
        }
        let avg_a = self.clone().avg();
        let avg_b = other.clone().avg();
        Ok(self
            .into_iter()
            .zip(other)
            .fold(T::zero(), |folding, (val_a, val_b)| {
                folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
            })
            / (len as f64))
    }
}

impl<T> StatisticsOrderedNonEmpty<T> for NonEmpty<T>
where
    T: Ord,
{
    fn min(&self) -> &T {
        //because this is a NonEmpty collection, unwrap() is supposed to happen here
        self.iter().min().unwrap()
    }
    fn max(&self) -> &T {
        //because this is a NonEmpty collection, unwrap() is supposed to happen here
        self.iter().max().unwrap()
    }
}
impl<T> StatisticsOrderedAbsNonEmpty<T> for NonEmpty<T>
where
    T: Ord + Absolutable,
{
    fn abs_min(&self) -> T {
        self.iter().min().map(Absolutable::abs).unwrap()
    }
    fn abs_max(&self) -> T {
        self.iter().max().map(Absolutable::abs).unwrap()
    }
}
