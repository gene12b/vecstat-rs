use crate::errors::VecStatError;

//#[cfg(feature = "derive")]
//pub use vecstat_derive::*;
//pub mod derive;
pub mod calc_traits;
//#[cfg(not(feature = "derive"))]
pub mod def;
pub mod errors;
#[cfg(feature = "nonempty")]
pub mod nonempty;

#[cfg(feature = "partial")]
pub mod partial;

//TODO move to another trait?
//fn harmonic_mean(self) -> Option<T>;

pub trait StatisticsUnorderedF32<T> {
    fn mean(self) -> Option<T>;
    fn avg(self) -> Option<T>;
    fn geometric_mean(self) -> Option<T>;
    fn variance(self) -> Option<T>;
    fn variance_sample(self) -> Option<T>;
    fn std_dev(self) -> Option<T>;
    fn std_dev_sample(self) -> Option<T>;
    fn covariance(self, other: Self) -> Result<Option<T>, VecStatError<T>>;
    fn covariance_sample(self, other: Self) -> Result<Option<T>, VecStatError<T>>;
    fn quadratic_mean(self) -> Option<T>;
    fn rms(self) -> Option<T>;
}
pub trait StatisticsUnorderedF64<T> {
    fn mean(self) -> Option<T>;
    fn avg(self) -> Option<T>;
    fn geometric_mean(self) -> Option<T>;
    fn variance(self) -> Option<T>;
    fn variance_sample(self) -> Option<T>;
    fn std_dev(self) -> Option<T>;
    fn std_dev_sample(self) -> Option<T>;
    fn covariance(self, other: Self) -> Result<Option<T>, VecStatError<T>>;
    fn covariance_sample(self, other: Self) -> Result<Option<T>, VecStatError<T>>;
    fn quadratic_mean(self) -> Option<T>;
    fn rms(self) -> Option<T>;
}

pub trait StatisticsOrdered<T> {
    ///asdf calc min_stat
    fn min_stat(&self) -> Option<&T>;
    fn max_stat(&self) -> Option<&T>;
}

pub trait StatisticsOrderedAbs<T> {
    fn abs_min(&self) -> Option<T>;
    fn abs_max(&self) -> Option<T>;
}

pub trait StatisticsOrder<T> {
    fn order_statistic(&mut self, order: usize) -> T;
    fn median(&mut self) -> T;
    fn quantile(&mut self, tau: f64) -> T;
    fn percentile(&mut self, p: usize) -> T;
    fn lower_quartile(&mut self) -> T;
    fn upper_quartile(&mut self) -> T;
    fn interquartile_range(&mut self) -> T;
    fn ranks(&mut self, tie_breaker: RankTieBreaker) -> Vec<T>;
}

pub enum RankTieBreaker {
    Average,
    Min,
    Max,
    First,
}

pub trait StatisticsF32<T>: StatisticsUnsignedF32<T> + StatisticsOrderedAbs<T> {}
pub trait StatisticsF64<T>: StatisticsUnsignedF64<T> + StatisticsOrderedAbs<T> {}
pub trait StatisticsUnsignedF32<T>: StatisticsOrdered<T> + StatisticsUnorderedF32<T> {}
pub trait StatisticsUnsignedF64<T>: StatisticsOrdered<T> + StatisticsUnorderedF64<T> {}
