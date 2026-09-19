use crate::calc_traits::*;
use crate::*;
use std::ops::*;

impl<T> StatisticsUnorderedF32<T> for Vec<T>
where
    T: Zeroable
        + Add<Output = T>
        + Div<f32, Output = T>
        + Sub<Output = T>
        + Floatable<f32>
        + Clone
        + Mul<Output = T>,
    //f32: std::ops::Mul<T>,
{
    fn mean(self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(
                self.into_iter()
                    .fold(T::zero(), |folding, val| folding + val)
                    / (len as f32),
            )
        }
    }
    fn avg(self) -> Option<T> {
        self.mean()
    }
    fn variance(self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let len = self.len();
        let avg = self.clone().avg()?;
        Some(
            self.into_iter().fold(T::zero(), |folding, val| {
                folding + (val - avg.clone()).powi(2)
            }) / (len as f32),
        )
    }
    fn std_dev(self) -> Option<T> {
        Some(self.variance()?.sqrt())
    }
    fn covariance(self, other: Self) -> Result<Option<T>, VecStatError<T>> {
        let len = self.len();
        if len != other.len() {
            return Err(VecStatError::UnequalLengths(len, other.len(), self, other));
        }
        let Some(avg_a) = self.clone().avg() else {
            return Ok(None);
        };
        let Some(avg_b) = other.clone().avg() else {
            return Ok(None);
        };
        Ok(Some(
            self.into_iter()
                .zip(other)
                .fold(T::zero(), |folding, (val_a, val_b)| {
                    folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
                })
                / (len as f32),
        ))
    }
    fn geometric_mean(self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(
                self.into_iter()
                    .fold(T::zero(), |folding, val| folding * val)
                    .powf(1.0 / (len as f32)),
            )
        }
    }
    fn quadratic_mean(self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(
                self.into_iter()
                    .fold(T::zero(), |folding, val| folding + val.powi(2))
                    .div(len as f32)
                    .sqrt(),
            )
        }
    }
    fn rms(self) -> Option<T> {
        self.quadratic_mean()
    }
    fn std_dev_sample(self) -> Option<T> {
        Some(self.variance_sample()?.sqrt())
    }
    fn variance_sample(self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let len = self.len() - 1;
        let avg = self.clone().avg()?;
        Some(
            self.into_iter().fold(T::zero(), |folding, val| {
                folding + (val - avg.clone()).powi(2)
            }) / (len as f32),
        )
    }
    fn covariance_sample(self, other: Self) -> Result<Option<T>, VecStatError<T>> {
        let len = self.len() - 1;
        if len != other.len() {
            return Err(VecStatError::UnequalLengths(len, other.len(), self, other));
        }
        let Some(avg_a) = self.clone().avg() else {
            return Ok(None);
        };
        let Some(avg_b) = other.clone().avg() else {
            return Ok(None);
        };
        Ok(Some(
            self.into_iter()
                .zip(other)
                .fold(T::zero(), |folding, (val_a, val_b)| {
                    folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
                })
                / (len as f32),
        ))
    }
}

impl<T> StatisticsUnorderedF64<T> for Vec<T>
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
    fn mean(self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(
                self.into_iter()
                    .fold(T::zero(), |folding, val| folding + val)
                    / (len as f64),
            )
        }
    }
    fn avg(self) -> Option<T> {
        self.mean()
    }
    fn variance(self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let len = self.len();
        let avg = self.clone().avg()?;
        Some(
            self.into_iter().fold(T::zero(), |folding, val| {
                folding + (val - avg.clone()).powi(2)
            }) / (len as f64),
        )
    }
    fn std_dev(self) -> Option<T> {
        Some(self.variance()?.sqrt())
    }
    fn covariance(self, other: Self) -> Result<Option<T>, VecStatError<T>> {
        let len = self.len();
        if len != other.len() {
            return Err(VecStatError::UnequalLengths(len, other.len(), self, other));
        }
        let Some(avg_a) = self.clone().avg() else {
            return Ok(None);
        };
        let Some(avg_b) = other.clone().avg() else {
            return Ok(None);
        };
        Ok(Some(
            self.into_iter()
                .zip(other)
                .fold(T::zero(), |folding, (val_a, val_b)| {
                    folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
                })
                / (len as f64),
        ))
    }
    fn geometric_mean(self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(
                self.into_iter()
                    .fold(T::zero(), |folding, val| folding * val)
                    .powf(1.0 / (len as f64)),
            )
        }
    }
    fn quadratic_mean(self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(
                self.into_iter()
                    .fold(T::zero(), |folding, val| folding + val.powi(2))
                    .div(len as f64)
                    .sqrt(),
            )
        }
    }
    fn rms(self) -> Option<T> {
        self.quadratic_mean()
    }
    fn std_dev_sample(self) -> Option<T> {
        Some(self.variance_sample()?.sqrt())
    }
    fn variance_sample(self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let len = self.len() - 1;
        let avg = self.clone().avg()?;
        Some(
            self.into_iter().fold(T::zero(), |folding, val| {
                folding + (val - avg.clone()).powi(2)
            }) / (len as f64),
        )
    }
    fn covariance_sample(self, other: Self) -> Result<Option<T>, VecStatError<T>> {
        let len = self.len() - 1;
        if len != other.len() {
            return Err(VecStatError::UnequalLengths(len, other.len(), self, other));
        }
        let Some(avg_a) = self.clone().avg() else {
            return Ok(None);
        };
        let Some(avg_b) = other.clone().avg() else {
            return Ok(None);
        };
        Ok(Some(
            self.into_iter()
                .zip(other)
                .fold(T::zero(), |folding, (val_a, val_b)| {
                    folding + ((val_a - avg_a.clone()) * (val_b - avg_b.clone()))
                })
                / (len as f64),
        ))
    }
}

//fn harmonic_mean(self) -> Option<T> {
//    if self.is_empty() {
//        None
//    } else {
//        Some(
//            (self.len() as f32)
//                * self
//                    .into_iter()
//                    .fold(T::zero(), |folding, val| folding + val.powi(-1))
//                    .powi(-1),
//        )
//    }
//}

impl<T> StatisticsOrdered<T> for Vec<T>
where
    T: Ord,
{
    fn min_stat(&self) -> Option<&T> {
        self.iter().min()
    }
    fn max_stat(&self) -> Option<&T> {
        self.iter().max()
    }
}

impl<T> StatisticsOrderedAbs<T> for Vec<T>
where
    T: Ord + Absolutable,
{
    fn abs_min(&self) -> Option<T> {
        self.iter().min().map(Absolutable::abs)
    }
    fn abs_max(&self) -> Option<T> {
        self.iter().max().map(Absolutable::abs)
    }
}
